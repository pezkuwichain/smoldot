//! Re-signs the extrinsics of smoldot's `executor/runtime_call` fixtures under PezkuwiChain's
//! sr25519 context. Each fixture is a dev-chain block signed by //Alice under Substrate's context;
//! the signing payload is rebuilt (call, era/nonce/tip, spec/tx version, genesis twice), the old
//! signature is checked under the old context first, then replaced, and the header's extrinsics
//! root recomputed. Edits the files in place by replacing hex strings. A one-time conversion: the
//! fixtures in the tree are converted already; run it only on fixtures newly taken from upstream.
//!
//!     cargo run --release -- ../../lib/src/executor/runtime_call/*.json
use smoldot::{executor::host, identity::seed_phrase, trie};

fn compact(b: &[u8], i: usize) -> (usize, usize) {
    match b[i] & 3 {
        0 => ((b[i] >> 2) as usize, i + 1),
        1 => ((u16::from_le_bytes([b[i], b[i + 1]]) >> 2) as usize, i + 2),
        2 => (
            (u32::from_le_bytes(b[i..i + 4].try_into().unwrap()) >> 2) as usize,
            i + 4,
        ),
        _ => panic!("big compact"),
    }
}
fn twox(data: &[u8], n: usize) -> Vec<u8> {
    (0..n as u64)
        .flat_map(|s| twox_hash::XxHash64::oneshot(s, data).to_le_bytes())
        .collect()
}
fn unhex(s: &str) -> Vec<u8> {
    hex::decode(s.trim_start_matches("0x")).unwrap()
}

fn main() {
    let secret = schnorrkel::SecretKey::from_bytes(
        &*seed_phrase::decode_sr25519_private_key("//Alice").unwrap(),
    )
    .unwrap();
    let pair = secret.clone().to_keypair();
    for path in std::env::args().skip(1) {
        let mut text = std::fs::read_to_string(&path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&text).unwrap();
        let storage = json["parentStorage"]["mainTrie"].as_object().unwrap();
        let get = |k: &[u8]| {
            storage
                .iter()
                .find(|(key, _)| unhex(key) == k)
                .map(|(_, v)| unhex(v.as_str().unwrap()))
        };
        let code = get(b":code").unwrap();
        let vm = host::HostVmPrototype::new(host::Config {
            module: &code,
            heap_pages: smoldot::executor::DEFAULT_HEAP_PAGES,
            exec_hint: smoldot::executor::vm::ExecHint::ExecuteOnceWithNonDeterministicValidation,
            allow_unresolved_imports: true,
        })
        .unwrap();
        let v = vm.runtime_version().decode();
        let (spec, tx) = (v.spec_version, v.transaction_version.unwrap());
        let version = v.state_version.unwrap_or(trie::TrieEntryVersion::V0);
        let mut key = [
            twox(b"System", 2),
            twox(b"BlockHash", 2),
            twox(&0u32.to_le_bytes(), 1),
        ]
        .concat();
        key.extend_from_slice(&0u32.to_le_bytes());
        let header_hex = json["block"]["header"].as_str().unwrap().to_owned();
        // Genesis storage holds a placeholder for System.BlockHash(0); block 1's parent is genesis.
        let genesis = match compact(&unhex(&header_hex), 32).0 {
            1 => unhex(&header_hex)[..32].to_vec(),
            _ => get(&key).expect("System.BlockHash(0)"),
        };
        let body: Vec<String> = json["block"]["body"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_owned())
            .collect();
        let mut new_body = Vec::new();
        for x in &body {
            let mut b = unhex(x);
            let (_, i) = compact(&b, 0);
            if b[i] & 0x80 == 0 {
                new_body.push(b);
                continue;
            }
            assert_eq!(&b[i + 1..i + 2], &[0]); // MultiAddress::Id
            assert_eq!(b[i + 34], 1); // MultiSignature::Sr25519
            let sig_at = i + 35;
            let extra_at = sig_at + 64;
            assert_eq!(b[extra_at], 0); // immortal era
            let (_, after_nonce) = compact(&b, extra_at + 1);
            let (_, call_at) = compact(&b, after_nonce);
            let mut payload = b[call_at..].to_vec();
            payload.extend_from_slice(&b[extra_at..call_at]);
            payload.extend_from_slice(&spec.to_le_bytes());
            payload.extend_from_slice(&tx.to_le_bytes());
            payload.extend_from_slice(&genesis);
            payload.extend_from_slice(&genesis);
            if payload.len() > 256 {
                payload = blake2_rfc::blake2b::blake2b(32, &[], &payload)
                    .as_bytes()
                    .to_vec();
            }
            let old = schnorrkel::Signature::from_bytes(&b[sig_at..extra_at]).unwrap();
            assert!(
                pair.public
                    .verify_simple(b"substrate", &payload, &old)
                    .is_ok(),
                "{path}: payload not reconstructed"
            );
            let new = pair.sign(
                schnorrkel::signing_context(smoldot::SR25519_SIGNING_CONTEXT).bytes(&payload),
            );
            b[sig_at..extra_at].copy_from_slice(&new.to_bytes());
            new_body.push(b);
        }
        let mut header = unhex(&header_hex);
        let (_, after_number) = compact(&header, 32);
        // The extrinsics root's trie version is the runtime's choice: take the one that
        // reproduces the fixture's root, so the rebuilt header differs only by the new signatures.
        let old_body: Vec<Vec<u8>> = body.iter().map(|x| unhex(x)).collect();
        let old_root = header[after_number + 32..after_number + 64].to_vec();
        let version = [
            version,
            trie::TrieEntryVersion::V0,
            trie::TrieEntryVersion::V1,
        ]
        .into_iter()
        .find(|v| trie::ordered_root(*v, trie::HashFunction::Blake2, &old_body)[..] == old_root[..])
        .expect("extrinsics root not reproduced");
        let root = trie::ordered_root(version, trie::HashFunction::Blake2, &new_body);
        header[after_number + 32..after_number + 64].copy_from_slice(&root);
        let prefix = if header_hex.starts_with("0x") {
            "0x"
        } else {
            ""
        };
        text = text.replacen(&header_hex, &format!("{prefix}{}", hex::encode(&header)), 1);
        for (old, new) in body.iter().zip(&new_body) {
            text = text.replacen(old, &format!("{prefix}{}", hex::encode(new)), 1);
        }
        std::fs::write(&path, text).unwrap();
        println!(
            "{path}: spec {spec} tx {tx} re-signed {} extrinsics",
            body.len()
        );
    }
}
