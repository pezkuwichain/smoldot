// Smoldot
// Copyright (C) 2023  Pierre Krieger
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// TODO: add more tests

#![cfg(test)]

use core::{num::NonZero, time::Duration};

use super::{Config, HeaderVerifySuccess, NonFinalizedTree};
use crate::{chain::chain_information, header};

/// Two consecutive Zagros relay chain blocks, #526431 and #526432, inserted on top of #526430
/// as the finalized block, with the BABE epochs and GRANDPA set the chain reports at #526430.
#[test]
fn zagros_two_blocks_after_finalized() {
    fn hex32(s: &str) -> [u8; 32] {
        <[u8; 32]>::try_from(hex::decode(s).unwrap()).unwrap()
    }

    let finalized = hex::decode("d06f57b59c57f5df4f6e9b27f9281593c9443b0657bcac0f4ff2efc47ab087417a212000fa09b0a7225fe08a62819073739d145a7e018fb25f9bd0f0198011adfcdcce7590cebb8488781704141f41b65b22d41e0b1a96704c29b48ecf9df7a86f2fc56f0c0642414245b501030c0000008a49d111000000008a7a5f3c07ba91d21bad058b6814c16b70081a98c47993c435d4e14c0cb66316557d8569ae839f5b6ef993cf16266aa1e3c0769cfc13f6d4954015acd66f870c5cfa23fcfe510a6795d67bd4e56e09d9bfcc09433e6af03cfa448b81b87df00604424545468403ba87fbe133080a9d56639fd61d4f91a206ef67dcc71a99bc810ccff08ba78c4805424142450101e4395de4a1f35926bdc597bdb55ddb6d1bc76140b442fa1afcd3e16520e5c250708f2b32c3ebd45ea0539fe71f404cb7febd896848e1a355cc3afae9cc306287").unwrap();
    let block1 = hex::decode("ffea9bfb4f7289369984d722f55b42da77726c408217b51cdb92da77a7e748697e21200044d83ef4d3e2b2b7e4a2c5f6e56bdd843f5d2debcfcec43fb70d0a0bf39bb9d898ff424bb097ffb49d1a1ac376c335abf19d95609161ac587c43bb69fdc816470c0642414245b50103050000008b49d111000000002e6cbbcfdf73a142e647f8da60948f9ed206c59d717609778db89c30dd674f4b09631b1470bc14a2035330763bb361978be6912e3a4de1efa84b52c2d876f70495e190ededed7370403ab0043ae3fe4b9d2a3fde3085d2cc8971f5f09c00110f04424545468403c6c95ba118a9aa26aab5ecfee54fb1380e9485f6396cf2e30aa8a6e4ee3674e90542414245010122caa153471872c4eb429d08a385a618d3b75aea0209e67b5101d1d8f31d744135c65135f31aae35492c5d3a6edeeee16ea7bcb98d4445976f9ca7222c28118f").unwrap();
    let block2 = hex::decode("5fcccc4f9eb7ec1d05fffccf234426a86aaffbff8a83fc265ab7cffbb078671d822120000925ad7dacedd626b68bf94c8128138f8d360ac39389de621a5e2b8c7807e10ded337742a32212ea34c7aa5e72d422ea5f3bee9c047f81ecc94c56792a40aedd0c0642414245b501030c0000008c49d111000000000a56c264b6bd3432d561b00f932ae27092110dff62c014989edbfff40d29f172dd35673123df0a3820569109347610eb0973166945f1bfa4dfa4f20378b92404a3d05206276d05b2efffc7845d1dbfa76c774ec9660e7b8f625f2417d664f80c04424545468403aa34f6b4ea6452753fd87a183c5f712cd77ea4638fed8c2ed68d79ad521e0702054241424501019ea0cd7303e48b965dde1be3dfd2098975d70e516be4a99385051c693eb1720be2b3b9b553305da868502d94ab180f2b7a2220085f26c178391931fe4c803789").unwrap();

    let mut tree = NonFinalizedTree::new(Config {
        chain_information: chain_information::ChainInformation {
            finalized_block_header: Box::new(header::decode(&finalized, 4).unwrap().into()),
            consensus: chain_information::ChainInformationConsensus::Babe {
                slots_per_epoch: NonZero::<u64>::new(600).unwrap(),
                finalized_block_epoch_information: Some(Box::new(chain_information::BabeEpochInformation {
                    epoch_index: 895,
                    start_slot_number: Some(298928016),
                    authorities: vec![
                        header::BabeAuthority { public_key: hex32("22a5e0172444e42a1ea3e29830c83a6e0a84a33263cfe2a7008871b001a8e74b"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("507fc640a1bb08b2e118846de14705579dfe58672cf38f21db5752926acd9417"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("ac680114b0cf7808f05deeddebd067ed520eb71f9dccca4d3290a91760950e74"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("620f7e55a0af1d74edfd807d77e2f125ed28a1fcd865699da5897e6cf7d8e93a"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("4e4977fb1f2802b5c49b59b0154ac8f5f888bb72d91a21b0b84cb896a900ff54"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("0c5c662c5202d3791ea8d5a19f8121d2422790dbe46420eb83d6f12d90fb6f67"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("c015d853b41e00914525c852fc89bad17ed90a8df1a6be9bd7e945967e59de6e"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("8c54eb78ec5f530317c3f74ec43e3c0d14460755377b147be26f6e25ace5f03d"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("2c6fd021135a0de764bd7a26bad6b5a329c00d3195182f2f29e1694b4e46ab39"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("6896d03c0d08da73257a98332f418dbd84dedb10515a6cb20608b065953b833f"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("3a35ad97f29553dbf45438c7aabb1c6703cda4edfe31a6d8e97d75e77c7e6258"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("1e307845454d1930a44728d3a4b0c3a8a796ca7864f02337506d95bbba23a64d"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("26f1a959083449c89877ab91a027133806ea41a7363dccd66b751c4f31b3ab1a"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("f2e648c3df9281cbe3514b6a4c244a4d3d10bcf6dcd9aa8446149d8fe5cf821a"), weight: 1 },
                    ],
                    randomness: hex32("2487c75473402731f365fed142d6845cc91cedc16103b1399aec4a9ab108e142"),
                    c: (1, 4),
                    allowed_slots: header::BabeAllowedSlots::PrimaryAndSecondaryVrfSlots,
                })),
                finalized_next_epoch_transition: Box::new(chain_information::BabeEpochInformation {
                    epoch_index: 896,
                    start_slot_number: Some(298928616),
                    authorities: vec![
                        header::BabeAuthority { public_key: hex32("22a5e0172444e42a1ea3e29830c83a6e0a84a33263cfe2a7008871b001a8e74b"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("507fc640a1bb08b2e118846de14705579dfe58672cf38f21db5752926acd9417"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("ac680114b0cf7808f05deeddebd067ed520eb71f9dccca4d3290a91760950e74"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("620f7e55a0af1d74edfd807d77e2f125ed28a1fcd865699da5897e6cf7d8e93a"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("4e4977fb1f2802b5c49b59b0154ac8f5f888bb72d91a21b0b84cb896a900ff54"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("0c5c662c5202d3791ea8d5a19f8121d2422790dbe46420eb83d6f12d90fb6f67"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("c015d853b41e00914525c852fc89bad17ed90a8df1a6be9bd7e945967e59de6e"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("8c54eb78ec5f530317c3f74ec43e3c0d14460755377b147be26f6e25ace5f03d"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("2c6fd021135a0de764bd7a26bad6b5a329c00d3195182f2f29e1694b4e46ab39"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("6896d03c0d08da73257a98332f418dbd84dedb10515a6cb20608b065953b833f"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("3a35ad97f29553dbf45438c7aabb1c6703cda4edfe31a6d8e97d75e77c7e6258"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("1e307845454d1930a44728d3a4b0c3a8a796ca7864f02337506d95bbba23a64d"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("26f1a959083449c89877ab91a027133806ea41a7363dccd66b751c4f31b3ab1a"), weight: 1 },
                        header::BabeAuthority { public_key: hex32("f2e648c3df9281cbe3514b6a4c244a4d3d10bcf6dcd9aa8446149d8fe5cf821a"), weight: 1 },
                    ],
                    randomness: hex32("9ad2e5ff3269a667c32ec2c38f45c103db72d13d73bf7af581c5f4a91c374e43"),
                    c: (1, 4),
                    allowed_slots: header::BabeAllowedSlots::PrimaryAndSecondaryVrfSlots,
                }),
            },
            finality: chain_information::ChainInformationFinality::Grandpa {
                after_finalized_block_authorities_set_id: 4,
                finalized_triggered_authorities: vec![
                    header::GrandpaAuthority { public_key: hex32("140ec6acc3d2ccfe8017c2b87a0dfe634c6de1104574059e3ba05cdbb6190f08"), weight: NonZero::<u64>::new(1).unwrap() },
                    header::GrandpaAuthority { public_key: hex32("e78c542579cfcb4bf781268a57c7ae19abe426ed075f298f1ead033e901da882"), weight: NonZero::<u64>::new(1).unwrap() },
                    header::GrandpaAuthority { public_key: hex32("47d1e266d0f8411dc03808acbaf330499c03a288f0dd80d0b7a558e1ee7b6e8f"), weight: NonZero::<u64>::new(1).unwrap() },
                    header::GrandpaAuthority { public_key: hex32("1cee1e8b20ef554efaa29ac5a2c2418503df3a4449a769f3649a2add036bd38f"), weight: NonZero::<u64>::new(1).unwrap() },
                    header::GrandpaAuthority { public_key: hex32("2600c4b144906b2dc3ba8aefca015a8532e5589b11b164439fb0b4fb004383aa"), weight: NonZero::<u64>::new(1).unwrap() },
                    header::GrandpaAuthority { public_key: hex32("0d3be87d022659068339f82bb458976f83b7c75287b842a2dc179d556b34fbff"), weight: NonZero::<u64>::new(1).unwrap() },
                    header::GrandpaAuthority { public_key: hex32("3e6aae6e3c55c25f0b6765818b513ccceac769d371a4bb401240282d251151e7"), weight: NonZero::<u64>::new(1).unwrap() },
                    header::GrandpaAuthority { public_key: hex32("37cd7e2689782aa1d5f52ed24bd9c4bcf8f811effbefac2039a027a51f33a595"), weight: NonZero::<u64>::new(1).unwrap() },
                    header::GrandpaAuthority { public_key: hex32("b34d50cbb391e36f5820fe8d4c572b8c20e3b7ce82dfbdac08ac800b90554d43"), weight: NonZero::<u64>::new(1).unwrap() },
                    header::GrandpaAuthority { public_key: hex32("0e96449f0667776ca4507b30fef3e4168f20da1acb0dd57e71ecca74d7e7ace5"), weight: NonZero::<u64>::new(1).unwrap() },
                    header::GrandpaAuthority { public_key: hex32("c57688bc9175f1ea6e2cf7dd8a3593ca6b2764b1b78165c8e8c6b791f66b48a7"), weight: NonZero::<u64>::new(1).unwrap() },
                    header::GrandpaAuthority { public_key: hex32("fb6f3bc5a49916d1917500d3ff1882c525f2d20d32f16d5429ec1595e05db182"), weight: NonZero::<u64>::new(1).unwrap() },
                    header::GrandpaAuthority { public_key: hex32("7d4567a4100b9fd2db6d741a60aef2693f6ca1c875a588036ab2b7cd3d449b63"), weight: NonZero::<u64>::new(1).unwrap() },
                    header::GrandpaAuthority { public_key: hex32("87f3c60d6f12aeee640840f82b9deb899d287dffc605a9a115d46dc84f4739b4"), weight: NonZero::<u64>::new(1).unwrap() },
                ],
                finalized_scheduled_change: None,
            },
        }
        .try_into()
        .unwrap(),
        blocks_capacity: 8,
        block_number_bytes: 4,
        // Zagros headers carry BEEFY (`BEEF`) consensus logs, which smoldot does not verify.
        allow_unknown_consensus_engines: true,
    });

    let verified_header1 = match tree.verify_header(block1, Duration::new(0, 0)).unwrap() {
        HeaderVerifySuccess::Verified {
            verified_header, ..
        } => verified_header,
        _ => panic!(),
    };

    tree.insert_verified_header(verified_header1, ());

    let verified_header2 = match tree.verify_header(block2, Duration::new(0, 0)).unwrap() {
        HeaderVerifySuccess::Verified {
            verified_header, ..
        } => verified_header,
        _ => panic!(),
    };

    tree.insert_verified_header(verified_header2, ());
}
