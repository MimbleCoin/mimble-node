// Copyright 2020 The Grin Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Definition of the genesis block. Placeholder for now.

// required for genesis replacement
//! #![allow(unused_imports)]

#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]

use crate::core;
use crate::core::hash::Hash;
use crate::pow::{Difficulty, Proof, ProofOfWork};
use chrono::prelude::{TimeZone, Utc};
use keychain::BlindingFactor;
use util;
use util::secp::constants::SINGLE_BULLET_PROOF_SIZE;
use util::secp::pedersen::{Commitment, RangeProof};
use util::secp::Signature;

/// Genesis block definition for development networks. The proof of work size
/// is small enough to mine it on the fly, so it does not contain its own
/// proof of work solution. Can also be easily mutated for different tests.
pub fn genesis_dev() -> core::Block {
	core::Block::with_header(core::BlockHeader {
		height: 0,
		timestamp: Utc.ymd(1997, 8, 4).and_hms(0, 0, 0),
		pow: ProofOfWork {
			nonce: 0,
			..Default::default()
		},
		..Default::default()
	})
}

/// Floonet genesis block
pub fn genesis_floo() -> core::Block {
	let gen = core::Block::with_header(core::BlockHeader {
		height: 0,
		timestamp: Utc.ymd(2020, 9, 21).and_hms(15, 38, 10),
		prev_root: Hash::from_hex(
			"0000000000000000000631c499f8909b085041da7251a9fd3b97b82971a5dab1",
		)
		.unwrap(),
		output_root: Hash::from_hex(
			"ca9f73015b87e336d74f77c027fa56e0a446cfc72e7daccdd971a122f1d54a36",
		)
		.unwrap(),
		range_proof_root: Hash::from_hex(
			"227169ebed875dd81cc7b5f676e822b143ad51f02b4f1de85e67ed11d6d7ff4c",
		)
		.unwrap(),
		kernel_root: Hash::from_hex(
			"fbe87ad46b0890e2da0bedd768ade5b4f3e72e5253ecec096c7276ee527b8685",
		)
		.unwrap(),
		total_kernel_offset: BlindingFactor::from_hex(
			"0000000000000000000000000000000000000000000000000000000000000000",
		)
		.unwrap(),
		output_mmr_size: 1,
		kernel_mmr_size: 1,
		pow: ProofOfWork {
			total_difficulty: Difficulty::from_num(10000),
			secondary_scaling: 1856,
			nonce: 1,
			proof: Proof {
				nonces: vec![6482295, 27597825, 30773961, 35278813, 36513949, 39655470, 61708881, 66244263, 73412920, 77985440, 89555393, 100993816, 168328967, 211982393, 234194477, 244594199, 247889114, 247902025, 264686981, 287673810, 287709293, 291038350, 300460441, 312969247, 339870092, 343857000, 364376213, 380706055, 384799465, 385511278, 387988979, 406423782, 414216685, 415788169, 424509323, 426748017, 441640536, 457936333, 467041084, 472785369, 513768435, 527121989],
				edge_bits: 29,
			},
		},
		..Default::default()
	});
	let kernel = core::TxKernel {
		features: core::KernelFeatures::Coinbase,
		excess: Commitment::from_vec(
			util::from_hex(
				&"092c7846522fbd1fc91d45f88297f6d6f8459afb07a6e0b94fb3489f00c9c28507".to_string(),
			)
			.unwrap(),
		),
		excess_sig: Signature::from_raw_data(&[137, 58, 198, 234, 249, 0, 160, 176, 164, 10, 16, 52, 38, 121, 146, 175, 60, 215, 103, 149, 139, 230, 112, 223, 30, 158, 210, 233, 63, 0, 63, 33, 108, 220, 94, 201, 69, 130, 74, 2, 245, 181, 137, 180, 20, 40, 144, 166, 78, 224, 178, 206, 125, 170, 123, 72, 20, 139, 145, 238, 133, 8, 107, 233])
		.unwrap(),
	};
	let output = core::Output {
		features: core::OutputFeatures::Coinbase,
		commit: Commitment::from_vec(
			util::from_hex(
				&"09101b8a6dbd9980b05315dcd81e3bc76dbff9dbf0b82b345dac564fbb07cad9eb".to_string(),
			)
			.unwrap(),
		),
		proof: RangeProof {
			plen: SINGLE_BULLET_PROOF_SIZE,
			proof: [54, 205, 206, 65, 184, 81, 50, 109, 200, 223, 133, 152, 8, 90, 6, 91, 11, 167, 11, 254, 63, 198, 195, 134, 14, 18, 195, 125, 210, 194, 51, 126, 225, 139, 232, 149, 213, 136, 229, 71, 205, 97, 61, 130, 24, 108, 157, 80, 2, 4, 181, 113, 150, 164, 18, 218, 147, 44, 34, 138, 196, 60, 120, 166, 6, 20, 22, 134, 196, 121, 139, 70, 7, 52, 130, 215, 72, 214, 184, 116, 202, 122, 157, 91, 123, 114, 111, 137, 133, 65, 186, 19, 14, 64, 64, 155, 32, 45, 202, 225, 243, 96, 130, 9, 8, 10, 174, 188, 64, 186, 130, 50, 62, 153, 58, 132, 42, 37, 154, 229, 229, 52, 203, 40, 237, 82, 91, 174, 151, 253, 236, 183, 63, 240, 32, 25, 87, 103, 118, 156, 18, 5, 168, 184, 56, 86, 114, 139, 16, 103, 63, 20, 254, 160, 78, 252, 180, 143, 69, 51, 155, 233, 124, 8, 254, 210, 24, 241, 71, 194, 188, 45, 216, 24, 23, 48, 110, 130, 74, 194, 52, 211, 143, 5, 106, 47, 179, 224, 113, 150, 114, 153, 14, 194, 60, 237, 191, 38, 194, 45, 236, 38, 105, 228, 50, 121, 126, 42, 125, 107, 127, 208, 0, 48, 167, 19, 81, 88, 207, 102, 161, 194, 80, 63, 63, 210, 160, 250, 34, 184, 244, 186, 15, 5, 83, 101, 145, 237, 187, 96, 241, 42, 126, 110, 221, 97, 96, 218, 209, 113, 141, 25, 113, 106, 186, 87, 86, 102, 135, 195, 138, 180, 102, 193, 187, 221, 116, 68, 116, 58, 181, 175, 161, 106, 125, 250, 158, 162, 114, 74, 223, 149, 123, 84, 214, 135, 243, 185, 198, 217, 43, 203, 231, 204, 203, 123, 195, 166, 205, 17, 237, 211, 55, 162, 149, 9, 221, 227, 204, 175, 226, 32, 206, 134, 32, 181, 153, 185, 223, 28, 70, 220, 44, 77, 8, 234, 67, 145, 179, 237, 67, 26, 232, 23, 25, 250, 208, 199, 57, 151, 207, 186, 41, 68, 249, 202, 3, 177, 67, 34, 98, 189, 124, 25, 0, 181, 90, 128, 112, 244, 102, 51, 161, 31, 224, 247, 207, 235, 49, 48, 38, 246, 242, 51, 210, 116, 187, 152, 252, 11, 187, 233, 235, 23, 163, 132, 147, 102, 240, 19, 70, 150, 132, 200, 62, 106, 235, 201, 52, 101, 66, 139, 104, 223, 178, 38, 217, 221, 56, 180, 121, 64, 43, 179, 235, 24, 27, 170, 40, 93, 243, 252, 183, 5, 72, 140, 203, 2, 200, 238, 143, 55, 100, 156, 81, 107, 218, 129, 98, 32, 135, 95, 247, 67, 60, 151, 252, 108, 59, 233, 154, 13, 57, 97, 165, 10, 49, 146, 114, 61, 64, 40, 122, 195, 5, 225, 69, 194, 207, 100, 119, 51, 229, 13, 19, 228, 183, 15, 43, 248, 158, 109, 147, 110, 249, 111, 227, 195, 224, 249, 254, 231, 11, 9, 47, 11, 134, 170, 95, 192, 177, 54, 219, 169, 59, 255, 142, 27, 198, 35, 20, 155, 28, 173, 216, 14, 47, 187, 183, 194, 1, 128, 156, 195, 76, 109, 22, 190, 117, 194, 58, 3, 22, 121, 244, 112, 198, 102, 172, 122, 255, 165, 72, 149, 215, 70, 224, 26, 160, 229, 39, 0, 61, 144, 83, 13, 133, 226, 189, 204, 16, 51, 219, 241, 34, 32, 170, 111, 175, 60, 148, 244, 174, 202, 102, 20, 53, 179, 23, 85, 99, 173, 114, 203, 99, 247, 185, 250, 154, 26, 52, 85, 189, 134, 202, 113, 41, 33, 30, 79, 39, 141, 52, 128, 153, 73, 28, 189, 240, 247, 47, 108, 196, 41, 41, 155, 43, 126, 155, 210, 238, 76, 169, 177, 0, 211, 223, 24, 56, 147, 122, 181, 72, 108, 58, 176, 82, 143, 188, 126, 184, 176, 213, 4, 221, 146, 50, 229, 245, 128, 42, 11, 228, 101, 33, 234, 235, 160, 200, 242, 51, 234, 235, 6, 85, 192, 156, 162, 2, 2, 61, 208, 74, 112, 14],
		},
	};
	gen.with_reward(output, kernel)
}

/// MWC GENESIS - here how genesis block is defined. gen_gen suppose to update the numbers in this file.
/// Mainnet genesis block 
pub fn genesis_main() -> core::Block {
	let gen = core::Block::with_header(core::BlockHeader{
		height: 0,
		timestamp: Utc.ymd(2020, 9, 20).and_hms(15, 28, 42),
		prev_root: Hash::from_hex(
			"0000000000000000000607004ac93fe0d31a754aad770d199c036846a3021449",
		)
		.unwrap(),
		output_root: Hash::from_hex(
			"ca9f73015b87e336d74f77c027fa56e0a446cfc72e7daccdd971a122f1d54a36",
		)
		.unwrap(),
		range_proof_root: Hash::from_hex(
			"227169ebed875dd81cc7b5f676e822b143ad51f02b4f1de85e67ed11d6d7ff4c",
		)
		.unwrap(),
		kernel_root: Hash::from_hex(
			"76f667faa4e0f4e6e701dc5751e15e36f4ca44cfa1c19f3c3c2e98078e3af096",
		)
		.unwrap(),
		total_kernel_offset: BlindingFactor::from_hex(
			"0000000000000000000000000000000000000000000000000000000000000000",
		)
		.unwrap(),
		output_mmr_size: 1,
		kernel_mmr_size: 1,
		pow: ProofOfWork {
			// MWC - TODO DEBUG  set difficulty to 1 because of testign
			//total_difficulty: Difficulty::from_num(10_u64.pow(5)),
			total_difficulty: Difficulty::from_num(10000),
			secondary_scaling: 1856,
			nonce: 60,
			proof: Proof {
				nonces: [22910605, 29897666, 34428756, 59719040, 67811179, 76018469, 79648067, 88757876, 107881804, 133092698, 134438673, 137222470, 144957990, 153231898, 172316710, 197721902, 204107458, 208310895, 223018761, 228253075, 228333033, 251935534, 252271732, 255571957, 264331296, 265483125, 279030773, 311886754, 323540657, 404806221, 415271426, 426632768, 432153464, 432478991, 476453970, 483931365, 497477360, 501174519, 507785849, 516544726, 518959986, 521396340].to_vec(),
				edge_bits: 29,
			},
		},
		..Default::default()
	});
	let kernel = core::TxKernel {
		features: core::KernelFeatures::Coinbase,
		excess: Commitment::from_vec(util::from_hex(&"092c7846522fbd1fc91d45f88297f6d6f8459afb07a6e0b94fb3489f00c9c28507".to_string()).unwrap()),

		excess_sig: Signature::from_raw_data(&[184, 5, 110, 116, 58, 108, 161, 187, 113, 31, 150, 2, 221, 234, 183, 61, 243, 68, 17, 87, 210, 191, 183, 88, 64, 136, 131, 255, 8, 50, 219, 85, 233, 142, 57, 85, 46, 171, 79, 110, 226, 28, 226, 94, 179, 240, 128, 233, 131, 133, 217, 168, 100, 105, 107, 222, 92, 52, 77, 55, 54, 128, 70, 59]).unwrap(),
	};
	let output = core::Output {
		features: core::OutputFeatures::Coinbase,
		commit: Commitment::from_vec(
			util::from_hex(&"09101b8a6dbd9980b05315dcd81e3bc76dbff9dbf0b82b345dac564fbb07cad9eb".to_string())
				.unwrap(),
		),
		proof: RangeProof {
			plen: SINGLE_BULLET_PROOF_SIZE,
			proof: [54, 205, 206, 65, 184, 81, 50, 109, 200, 223, 133, 152, 8, 90, 6, 91, 11, 167, 11, 254, 63, 198, 195, 134, 14, 18, 195, 125, 210, 194, 51, 126, 225, 139, 232, 149, 213, 136, 229, 71, 205, 97, 61, 130, 24, 108, 157, 80, 2, 4, 181, 113, 150, 164, 18, 218, 147, 44, 34, 138, 196, 60, 120, 166, 6, 20, 22, 134, 196, 121, 139, 70, 7, 52, 130, 215, 72, 214, 184, 116, 202, 122, 157, 91, 123, 114, 111, 137, 133, 65, 186, 19, 14, 64, 64, 155, 32, 45, 202, 225, 243, 96, 130, 9, 8, 10, 174, 188, 64, 186, 130, 50, 62, 153, 58, 132, 42, 37, 154, 229, 229, 52, 203, 40, 237, 82, 91, 174, 151, 253, 236, 183, 63, 240, 32, 25, 87, 103, 118, 156, 18, 5, 168, 184, 56, 86, 114, 139, 16, 103, 63, 20, 254, 160, 78, 252, 180, 143, 69, 51, 155, 233, 124, 8, 254, 210, 24, 241, 71, 194, 188, 45, 216, 24, 23, 48, 110, 130, 74, 194, 52, 211, 143, 5, 106, 47, 179, 224, 113, 150, 114, 153, 14, 194, 60, 237, 191, 38, 194, 45, 236, 38, 105, 228, 50, 121, 126, 42, 125, 107, 127, 208, 0, 48, 167, 19, 81, 88, 207, 102, 161, 194, 80, 63, 63, 210, 160, 250, 34, 184, 244, 186, 15, 5, 83, 101, 145, 237, 187, 96, 241, 42, 126, 110, 221, 97, 96, 218, 209, 113, 141, 25, 113, 106, 186, 87, 86, 102, 135, 195, 138, 180, 102, 193, 187, 221, 116, 68, 116, 58, 181, 175, 161, 106, 125, 250, 158, 162, 114, 74, 223, 149, 123, 84, 214, 135, 243, 185, 198, 217, 43, 203, 231, 204, 203, 123, 195, 166, 205, 17, 237, 211, 55, 162, 149, 9, 221, 227, 204, 175, 226, 32, 206, 134, 32, 181, 153, 185, 223, 28, 70, 220, 44, 77, 8, 234, 67, 145, 179, 237, 67, 26, 232, 23, 25, 250, 208, 199, 57, 151, 207, 186, 41, 68, 249, 202, 3, 177, 67, 34, 98, 189, 124, 25, 0, 181, 90, 128, 112, 244, 102, 51, 161, 31, 224, 247, 207, 235, 49, 48, 38, 246, 242, 51, 210, 116, 187, 152, 252, 11, 187, 233, 235, 23, 163, 132, 147, 102, 240, 19, 70, 150, 132, 200, 62, 106, 235, 201, 52, 101, 66, 139, 104, 223, 178, 38, 217, 221, 56, 180, 121, 64, 43, 179, 235, 24, 27, 170, 40, 93, 243, 252, 183, 5, 72, 140, 203, 2, 200, 238, 143, 55, 100, 156, 81, 107, 218, 129, 98, 32, 135, 95, 247, 67, 60, 151, 252, 108, 59, 233, 154, 13, 57, 97, 165, 10, 49, 146, 114, 61, 64, 40, 122, 195, 5, 225, 69, 194, 207, 100, 119, 51, 229, 13, 19, 228, 183, 15, 43, 248, 158, 109, 147, 110, 249, 111, 227, 195, 224, 249, 254, 231, 11, 9, 47, 11, 134, 170, 95, 192, 177, 54, 219, 169, 59, 255, 142, 27, 198, 35, 20, 155, 28, 173, 216, 14, 47, 187, 183, 194, 1, 128, 156, 195, 76, 109, 22, 190, 117, 194, 58, 3, 22, 121, 244, 112, 198, 102, 172, 122, 255, 165, 72, 149, 215, 70, 224, 26, 160, 229, 39, 0, 61, 144, 83, 13, 133, 226, 189, 204, 16, 51, 219, 241, 34, 32, 170, 111, 175, 60, 148, 244, 174, 202, 102, 20, 53, 179, 23, 85, 99, 173, 114, 203, 99, 247, 185, 250, 154, 26, 52, 85, 189, 134, 202, 113, 41, 33, 30, 79, 39, 141, 52, 128, 153, 73, 28, 189, 240, 247, 47, 108, 196, 41, 41, 155, 43, 126, 155, 210, 238, 76, 169, 177, 0, 211, 223, 24, 56, 147, 122, 181, 72, 108, 58, 176, 82, 143, 188, 126, 184, 176, 213, 4, 221, 146, 50, 229, 245, 128, 42, 11, 228, 101, 33, 234, 235, 160, 200, 242, 51, 234, 235, 6, 85, 192, 156, 162, 2, 2, 61, 208, 74, 112, 14]
,
		},
	};
	gen.with_reward(output, kernel)
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::core::hash::Hashed;
	use crate::ser::{self, ProtocolVersion};

	#[test]
	fn floonet_genesis_hash() {
		let gen_hash = genesis_floo().hash();
		println!("floonet genesis hash: {}", gen_hash.to_hex());
		let gen_bin = ser::ser_vec(&genesis_floo(), ProtocolVersion(1)).unwrap();
		println!("floonet genesis full hash: {}\n", gen_bin.hash().to_hex());
		assert_eq!(
			gen_hash.to_hex(),
			"61ef1c8ea4d393f0bbbf474ca86562e59c461ef017ba835b9a27bed1a8593cea"
		);
		assert_eq!(
			gen_bin.hash().to_hex(),
			"fed085cba82d7fe7b7de34154c225e55bc9601a81ce33344246fb8202f027d92"
		);
	}

	#[test]
	fn mainnet_genesis_hash() {
		let gen_hash = genesis_main().hash();
		println!("mainnet genesis hash: {}", gen_hash.to_hex());
		let gen_bin = ser::ser_vec(&genesis_main(), ProtocolVersion(1)).unwrap();
		println!("mainnet genesis full hash: {}\n", gen_bin.hash().to_hex());
		assert_eq!(
			gen_hash.to_hex(),
			"fe7fdfe45c304cecaeac147ea75b9f22411d5de27488f8c46d81fb6ded447062"
		);
		assert_eq!(
			gen_bin.hash().to_hex(),
			"4b930099eb086f934e7c2131d0de20d84af41efc13f8f06caa31657e46dede33"
		);
	}
}
