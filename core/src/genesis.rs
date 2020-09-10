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
		timestamp: Utc.ymd(2020, 9, 8).and_hms(16, 26, 53),
		prev_root: Hash::from_hex(
			"0000000000000000000d30d2a096ee02aa46dbe9e59d6fcac13217ca916bf0c9",
		)
		.unwrap(),
		output_root: Hash::from_hex(
			"3ec300b50221bfc6534e3612571c5c672a15194ca8f9731e3d1f640eae464aa7",
		)
		.unwrap(),
		range_proof_root: Hash::from_hex(
			"5782bc1c76e4fb55df6ee70acda1aaa44359232f2815a452667d5d1e0834b1d7",
		)
		.unwrap(),
		kernel_root: Hash::from_hex(
			"cf0b8d0cb6ed3c8df29910912b84b447f8282b06ca4908c9a35a0af09691fd08",
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
			nonce: 63,
			proof: Proof {
				nonces: vec![2663055, 15952349, 17933353, 32089640, 34710399, 46249255, 50825842, 72158390, 92158134, 102296096, 103625589, 147231721, 162478083, 184239352, 189208163, 221064280, 223911509, 253018525, 273130597, 283176421, 292261615, 297809801, 300945635, 317217498, 321822021, 336347951, 338938035, 346325960, 353917964, 359271970, 420600807, 424291888, 431611097, 443030924, 472403726, 474944756, 478151716, 478666722, 491069719, 494084621, 530578149, 533782068],
				edge_bits: 29,
			},
		},
		..Default::default()
	});
	let kernel = core::TxKernel {
		features: core::KernelFeatures::Coinbase,
		excess: Commitment::from_vec(
			util::from_hex(
				&"0829b09f07530516219d703736b46372a93207ccdf29c053daa0089d81b63968dd".to_string(),
			)
			.unwrap(),
		),
		excess_sig: Signature::from_raw_data(&[84, 180, 97, 25, 236, 188, 7, 208, 156, 230, 186, 169, 189, 131, 74, 25, 68, 0, 189, 188, 25, 250, 81, 34, 21, 242, 221, 49, 52, 106, 78, 10, 61, 37, 200, 181, 154, 98, 30, 18, 33, 56, 50, 142, 61, 246, 58, 190, 3, 149, 242, 204, 26, 194, 33, 230, 185, 243, 29, 178, 112, 68, 45, 47])
		.unwrap(),
	};
	let output = core::Output {
		features: core::OutputFeatures::Coinbase,
		commit: Commitment::from_vec(
			util::from_hex(
				&"08490fdf120927b83c1a672b11152bd1c3707b61cb379d2ef302fb6047689b9a8a".to_string(),
			)
			.unwrap(),
		),
		proof: RangeProof {
			plen: SINGLE_BULLET_PROOF_SIZE,
			proof: [247, 27, 154, 134, 45, 116, 143, 18, 200, 237, 1, 188, 166, 58, 122, 171, 81, 11, 27, 245, 0, 155, 235, 0, 117, 22, 43, 46, 197, 65, 234, 188, 177, 144, 201, 8, 229, 65, 107, 253, 199, 28, 70, 220, 237, 108, 156, 74, 30, 243, 220, 15, 192, 155, 37, 55, 105, 186, 101, 2, 40, 205, 117, 79, 9, 55, 168, 75, 88, 64, 38, 3, 197, 240, 110, 78, 197, 34, 83, 34, 188, 236, 47, 137, 162, 210, 65, 247, 247, 69, 158, 230, 117, 48, 146, 173, 125, 159, 91, 18, 106, 46, 144, 11, 170, 5, 233, 106, 190, 73, 109, 77, 152, 17, 80, 66, 143, 9, 47, 251, 36, 142, 46, 106, 216, 130, 98, 44, 56, 129, 65, 238, 236, 114, 91, 86, 173, 195, 232, 205, 43, 184, 55, 245, 32, 9, 24, 77, 18, 50, 157, 22, 133, 17, 10, 193, 81, 247, 131, 149, 41, 43, 19, 199, 68, 97, 175, 89, 149, 167, 93, 111, 113, 153, 244, 231, 196, 39, 250, 48, 4, 48, 233, 121, 132, 153, 26, 94, 147, 220, 103, 100, 250, 203, 20, 19, 179, 32, 145, 117, 191, 60, 83, 178, 7, 112, 203, 63, 89, 35, 222, 227, 186, 82, 126, 145, 84, 234, 228, 234, 132, 90, 90, 61, 42, 41, 185, 222, 223, 165, 41, 167, 55, 39, 202, 154, 65, 66, 7, 205, 195, 166, 120, 42, 230, 250, 121, 54, 117, 24, 50, 118, 78, 168, 184, 41, 46, 97, 97, 9, 196, 84, 222, 86, 152, 84, 84, 120, 16, 106, 130, 195, 32, 84, 180, 197, 206, 73, 52, 217, 59, 13, 215, 84, 77, 108, 211, 99, 196, 84, 31, 26, 71, 73, 201, 214, 65, 247, 85, 241, 188, 129, 182, 95, 204, 32, 140, 145, 120, 125, 74, 19, 176, 108, 212, 99, 60, 163, 127, 61, 235, 121, 53, 220, 121, 106, 163, 9, 171, 16, 233, 79, 101, 60, 178, 155, 215, 224, 212, 145, 130, 26, 237, 135, 19, 178, 90, 31, 22, 83, 199, 213, 48, 200, 0, 87, 183, 142, 251, 142, 238, 207, 221, 94, 136, 197, 254, 17, 206, 25, 62, 157, 253, 163, 89, 171, 97, 65, 146, 211, 6, 240, 235, 100, 35, 228, 127, 87, 102, 86, 173, 223, 238, 212, 209, 21, 151, 207, 101, 232, 9, 81, 54, 124, 50, 141, 146, 166, 36, 85, 226, 99, 205, 100, 137, 10, 125, 253, 238, 36, 168, 42, 176, 181, 124, 210, 178, 112, 55, 206, 197, 162, 82, 93, 150, 40, 150, 214, 144, 47, 60, 58, 110, 19, 235, 10, 225, 68, 146, 27, 114, 96, 113, 108, 24, 114, 197, 142, 83, 24, 158, 255, 6, 72, 32, 137, 61, 0, 194, 72, 33, 127, 213, 82, 60, 36, 171, 77, 221, 52, 218, 116, 139, 198, 158, 173, 152, 169, 52, 74, 101, 102, 67, 239, 49, 215, 217, 35, 0, 46, 63, 173, 174, 139, 27, 253, 139, 35, 168, 81, 246, 238, 191, 155, 175, 0, 4, 104, 48, 8, 189, 242, 16, 246, 103, 38, 248, 82, 34, 214, 82, 65, 96, 245, 109, 154, 228, 22, 27, 116, 133, 37, 130, 200, 118, 255, 159, 209, 204, 60, 137, 114, 115, 122, 70, 52, 74, 227, 156, 215, 93, 129, 198, 35, 93, 179, 147, 195, 124, 82, 76, 209, 94, 216, 70, 235, 140, 126, 72, 246, 147, 131, 236, 210, 195, 202, 4, 207, 184, 154, 174, 254, 202, 59, 9, 47, 38, 1, 172, 198, 140, 62, 203, 197, 102, 21, 146, 47, 123, 254, 103, 251, 110, 168, 76, 245, 185, 47, 243, 73, 128, 118, 158, 67, 189, 162, 52, 60, 183, 121, 111, 146, 41, 50, 61, 144, 250, 255, 238, 186, 172, 23, 13, 248, 78, 209, 176, 196, 105, 28, 37, 205, 7, 122, 174, 179, 126, 9, 64, 157, 41, 203, 0, 82, 127, 158, 167, 66, 127, 56, 41, 174, 208, 229, 140],
		},
	};
	gen.with_reward(output, kernel)
}

/// MWC GENESIS - here how genesis block is defined. gen_gen suppose to update the numbers in this file.
/// Mainnet genesis block
pub fn genesis_main() -> core::Block {
	let gen = core::Block::with_header(core::BlockHeader{
		height: 0,
		timestamp: Utc.ymd(2020, 9, 10).and_hms(00, 15, 49),
		prev_root: Hash::from_hex(
			"0000000000000000000fca48a1523bd60eae92871ac410f5b885ed67533bd44d",
		)
		.unwrap(),
		output_root: Hash::from_hex(
			"c17079260586bb4a68a6f8430c487d53ee57d7ea36e3aa89121ddd2f1db9b446",
		)
		.unwrap(),
		range_proof_root: Hash::from_hex(
			"c50e4878584c26c7b2014a5a43177be13779b399f1f8a0aeb430dc1c96b569c2",
		)
		.unwrap(),
		kernel_root: Hash::from_hex(
			"0d271f1df606b9206beb6b9a1afcefc11b2c1d8fe2e0c951949cb0a36211c760",
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
			nonce: 11,
			proof: Proof {
				nonces: [1005118, 7037627, 29600588, 31599245, 42371267, 52987449, 53463814, 56319413, 56669836, 71529820, 75693963, 79620055, 110109226, 129525631, 140787092, 144536706, 218636254, 238243931, 242888311, 264575713, 274963982, 283840266, 287132530, 301620350, 304420100, 305520998, 352503574, 354223666, 372201754, 377267188, 396467649, 396567553, 405517285, 417725419, 420319696, 444980615, 461757977, 462039928, 473243990, 474709843, 489118980, 504230777].to_vec(),
				edge_bits: 29,
			},
		},
		..Default::default()
	});
	let kernel = core::TxKernel {
		features: core::KernelFeatures::Coinbase,
		excess: Commitment::from_vec(util::from_hex(&"096a194693b2f419f39f63e118ffac4dce97c5d91a30f31681dc10cff9e1a85da8".to_string()).unwrap()),

		excess_sig: Signature::from_raw_data(&[211, 119, 238, 114, 110, 161, 20, 155, 173, 198, 91, 200, 79, 140, 138, 135, 20, 236, 152, 242, 219, 227, 21, 180, 78, 97, 153, 61, 172, 88, 235, 190, 33, 69, 124, 162, 163, 145, 153, 18, 161, 122, 244, 229, 236, 107, 32, 87, 230, 194, 3, 48, 100, 211, 126, 79, 238, 37, 169, 110, 237, 139, 30, 110]).unwrap(),
	};
	let output = core::Output {
		features: core::OutputFeatures::Coinbase,
		commit: Commitment::from_vec(
			util::from_hex(&"082e5735afe2d2f7a9f34dd21ec7d07a98b117e4f8bf55367bf1dc2e2505fccd2b".to_string())
				.unwrap(),
		),
		proof: RangeProof {
			plen: SINGLE_BULLET_PROOF_SIZE,
			proof: [30, 210, 255, 127, 41, 138, 118, 83, 126, 139, 124, 112, 175, 52, 129, 240, 189, 161, 179, 11, 167, 38, 81, 212, 56, 4, 116, 245, 13, 2, 144, 192, 213, 4, 34, 121, 227, 166, 202, 216, 0, 133, 130, 75, 28, 143, 72, 247, 61, 138, 80, 114, 143, 114, 30, 237, 20, 226, 209, 222, 227, 103, 122, 7, 4, 158, 249, 31, 59, 100, 154, 49, 39, 73, 71, 148, 199, 89, 94, 48, 146, 119, 189, 27, 112, 241, 166, 114, 1, 141, 75, 54, 212, 62, 114, 77, 180, 242, 47, 22, 151, 247, 231, 207, 136, 72, 45, 136, 76, 142, 26, 186, 33, 165, 241, 218, 109, 241, 99, 20, 144, 24, 125, 247, 150, 20, 157, 45, 5, 69, 60, 195, 147, 57, 119, 113, 233, 121, 201, 142, 254, 74, 178, 157, 238, 58, 251, 15, 144, 201, 163, 250, 44, 215, 78, 154, 60, 134, 30, 230, 243, 2, 224, 166, 207, 242, 131, 233, 105, 79, 134, 251, 13, 195, 134, 21, 173, 212, 155, 62, 212, 5, 84, 27, 210, 74, 24, 194, 26, 247, 2, 15, 69, 59, 157, 29, 233, 179, 22, 207, 159, 174, 174, 2, 174, 224, 193, 206, 146, 97, 147, 59, 98, 47, 57, 197, 13, 31, 121, 134, 4, 191, 151, 245, 222, 91, 71, 161, 193, 168, 32, 213, 152, 55, 23, 191, 196, 169, 204, 61, 77, 143, 210, 17, 66, 63, 62, 161, 186, 242, 73, 43, 68, 169, 207, 86, 194, 96, 247, 64, 136, 100, 135, 131, 152, 102, 154, 196, 198, 226, 254, 211, 29, 138, 18, 186, 30, 180, 167, 152, 51, 51, 52, 182, 134, 28, 60, 27, 200, 111, 35, 169, 123, 25, 53, 59, 91, 192, 239, 56, 118, 243, 49, 187, 163, 118, 167, 78, 51, 56, 23, 75, 111, 157, 227, 250, 116, 20, 246, 14, 26, 61, 75, 247, 169, 145, 10, 182, 157, 69, 228, 182, 108, 136, 31, 245, 32, 207, 48, 110, 126, 204, 190, 39, 242, 23, 127, 171, 123, 5, 223, 12, 59, 39, 2, 240, 58, 176, 222, 60, 213, 224, 16, 47, 57, 248, 11, 32, 60, 38, 39, 10, 189, 32, 92, 151, 228, 181, 147, 219, 220, 38, 159, 159, 41, 132, 27, 34, 228, 108, 196, 96, 220, 82, 46, 63, 65, 156, 42, 183, 130, 129, 225, 70, 215, 113, 248, 33, 136, 100, 47, 77, 149, 156, 155, 73, 35, 132, 130, 137, 61, 255, 83, 98, 9, 162, 9, 156, 241, 136, 110, 32, 252, 205, 225, 173, 160, 206, 75, 255, 41, 250, 96, 136, 16, 175, 127, 48, 224, 244, 125, 19, 31, 167, 114, 22, 235, 86, 156, 246, 114, 165, 22, 105, 44, 248, 224, 18, 203, 147, 109, 25, 182, 5, 199, 7, 26, 177, 181, 219, 249, 194, 57, 234, 236, 176, 78, 222, 198, 54, 152, 3, 6, 126, 106, 173, 60, 35, 97, 74, 133, 51, 210, 140, 181, 151, 131, 61, 88, 254, 167, 179, 19, 187, 238, 28, 154, 162, 70, 181, 197, 152, 191, 227, 26, 163, 202, 176, 214, 191, 187, 186, 247, 44, 11, 155, 18, 222, 176, 147, 222, 200, 114, 97, 19, 220, 222, 223, 8, 197, 44, 30, 111, 30, 128, 255, 87, 210, 83, 53, 252, 77, 18, 72, 171, 141, 157, 162, 53, 141, 193, 194, 22, 234, 248, 241, 129, 184, 199, 81, 164, 26, 23, 14, 11, 100, 104, 182, 0, 95, 148, 144, 186, 91, 106, 83, 178, 243, 120, 11, 212, 9, 151, 34, 60, 215, 1, 198, 67, 189, 117, 54, 170, 77, 227, 21, 229, 219, 238, 12, 147, 170, 151, 227, 191, 235, 18, 152, 250, 90, 109, 110, 26, 180, 40, 232, 236, 172, 147, 86, 247, 170, 192, 68, 91, 176, 133, 235, 103, 135, 117, 60, 197, 243, 26, 24, 106, 167, 160, 34, 106, 133, 210, 45, 248, 233, 249, 75, 30, 56, 137, 58, 164, 206, 235],
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
			"a10f32177e0b8de4495637c5735577512963cb3dca42ee893fc9c5fade29dfa7"
		);
		assert_eq!(
			gen_bin.hash().to_hex(),
			"1ed0cd8d166353ce22f14a47fd383e78888315b58a670aac95f77a3d49ce973c"
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
			"1baab283914655e6a8798129b7b93945d7ff9ce8987f1cee45dc8d3c9529bcbf"
		);
		assert_eq!(
			gen_bin.hash().to_hex(),
			"f905918a9ee5ee911deaa601b5a8bfa107aaa925f2f46d09ebd2992b57798f13"
		);
	}
}
