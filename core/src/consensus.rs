// Copyright 2020 The Grin Developers
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

//! All the rules required for a cryptocurrency to have reach consensus across
//! the whole network are complex and hard to completely isolate. Some can be
//! simple parameters (like block reward), others complex algorithms (like
//! Merkle sum trees or reorg rules). However, as long as they're simple
//! enough, consensus-relevant constants and short functions should be kept
//! here.

use std::cmp::{max, min};

use crate::global;
use crate::core::block::HeaderVersion;
use crate::pow::Difficulty;
use crate::core::hash::{Hash, ZERO_HASH};

/// A Mimble is divisible to 10^9, following the SI prefixes
pub const MIMBLE_BASE: u64 = 1_000_000_000;
/// Millimimble, a thousand of a grin
pub const MILLI_MIMBLE: u64 = MIMBLE_BASE / 1_000;
/// Micromimble, a thousand of a milligrin
pub const MICRO_MIMBLE: u64 = MILLI_MIMBLE / 1_000;
/// Nanomimble, smallest unit, takes a billion to make a grin
pub const NANO_MIMBLE: u64 = 1;

/// Block interval, in seconds, the network will tune its next_target for. Note
/// that we may reduce this value in the future as we get more data on mining
/// with Cuckoo Cycle, networks improve and block propagation is optimized
/// (adjusting the reward accordingly).
pub const BLOCK_TIME_SEC: u64 = 60;

/// Mimble - Here is a block reward.
/// The block subsidy amount, depending on Epoch
//pub const REWARD: u64 = BLOCK_TIME_SEC * MIMBLE_BASE;

/// Actual block reward for a given total fee amount
pub fn reward(fee: u64, height: u64) -> u64 {
	// Mimble has block reward schedule similar to bitcoin 
	let block_reward = calc_mwc_block_reward(height);
	block_reward.saturating_add(fee)
}

/// Mimble genesis block reward in nanocoins (0,044100000 coins) to get to exactly 21M max Supply and close the difference from modifications
pub const GENESIS_BLOCK_REWARD: u64 = 44_100_000;

/// Nominal height for standard time intervals, hour is 60 blocks
pub const HOUR_HEIGHT: u64 = 3600 / BLOCK_TIME_SEC;
/// A day is 1440 blocks
pub const DAY_HEIGHT: u64 = 24 * HOUR_HEIGHT;
/// A week is 10_080 blocks
pub const WEEK_HEIGHT: u64 = 7 * DAY_HEIGHT;
/// A year is 524_160 blocks
pub const YEAR_HEIGHT: u64 = 52 * WEEK_HEIGHT;

/// Number of blocks before a coinbase matures and can be spent
pub const COINBASE_MATURITY: u64 = DAY_HEIGHT;

/// Ratio the secondary proof of work should take over the primary, as a
/// function of block height (time). Starts at 90% losing a percent
/// approximately every week. Represented as an integer between 0 and 100.
pub fn secondary_pow_ratio(height: u64) -> u64 {
	90u64.saturating_sub(height / (2 * YEAR_HEIGHT / 90))
}

/// The AR scale damping factor to use. Dependent on block height
/// to account for pre HF behavior on testnet4.
fn ar_scale_damp_factor(_height: u64) -> u64 {
	AR_SCALE_DAMP_FACTOR
}

/// Cuckoo-cycle proof size (cycle length)
pub const PROOFSIZE: usize = 42;

/// Default Cuckatoo Cycle edge_bits, used for mining and validating.
pub const DEFAULT_MIN_EDGE_BITS: u8 = 31;

/// Cuckaroo proof-of-work edge_bits, meant to be ASIC resistant.
pub const SECOND_POW_EDGE_BITS: u8 = 29;

/// Original reference edge_bits to compute difficulty factors for higher
/// Cuckoo graph sizes, changing this would hard fork
pub const BASE_EDGE_BITS: u8 = 24;

/// Default number of blocks in the past when cross-block cut-through will start
/// happening. Needs to be long enough to not overlap with a long reorg.
/// Rational
/// behind the value is the longest bitcoin fork was about 30 blocks, so 5h. We
/// add an order of magnitude to be safe and round to 7x24h of blocks to make it
/// easier to reason about.
pub const CUT_THROUGH_HORIZON: u32 = WEEK_HEIGHT as u32;

/// Default number of blocks in the past to determine the height where we request
/// a txhashset (and full blocks from). Needs to be long enough to not overlap with
/// a long reorg.
/// Rational behind the value is the longest bitcoin fork was about 30 blocks, so 5h.
/// We add an order of magnitude to be safe and round to 2x24h of blocks to make it
/// easier to reason about.
pub const STATE_SYNC_THRESHOLD: u32 = 2 * DAY_HEIGHT as u32;

/// Weight of an input when counted against the max block weight capacity
pub const BLOCK_INPUT_WEIGHT: usize = 1;

/// Weight of an output when counted against the max block weight capacity
pub const BLOCK_OUTPUT_WEIGHT: usize = 21;

/// Weight of a kernel when counted against the max block weight capacity
pub const BLOCK_KERNEL_WEIGHT: usize = 3;

/// Total maximum block weight. At current sizes, this means a maximum
/// theoretical size of:
/// * `(674 + 33 + 1) * (40_000 / 21) = 1_348_571` for a block with only outputs
/// * `(1 + 8 + 8 + 33 + 64) * (40_000 / 3) = 1_520_000` for a block with only kernels
/// * `(1 + 33) * 40_000 = 1_360_000` for a block with only inputs
///
/// Regardless of the relative numbers of inputs/outputs/kernels in a block the maximum
/// block size is around 1.5MB
/// For a block full of "average" txs (2 inputs, 2 outputs, 1 kernel) we have -
/// `(1 * 2) + (21 * 2) + (3 * 1) = 47` (weight per tx)
/// `40_000 / 47 = 851` (txs per block)
///
pub const MAX_BLOCK_WEIGHT: usize = 40_000;
/// Check whether the block version is valid at a given height, in case of a fork in the future
pub fn valid_header_version(height: u64, version: HeaderVersion) -> bool {

	version == HeaderVersion(1)


}
/// Check whether the block version is valid at a given height, in case there is ever a need for a Fork
pub fn header_version(height: u64) -> HeaderVersion {
	//if height < get_c31_hard_fork_block_height() {
		HeaderVersion(1)
	//} else {
	//	HeaderVersion(2)
	//}
}


/// Number of blocks used to calculate difficulty adjustments
pub const DIFFICULTY_ADJUST_WINDOW: u64 = HOUR_HEIGHT;

/// Average time span of the difficulty adjustment window
pub const BLOCK_TIME_WINDOW: u64 = DIFFICULTY_ADJUST_WINDOW * BLOCK_TIME_SEC;

/// Clamp factor to use for difficulty adjustment
/// Limit value to within this factor of goal
pub const CLAMP_FACTOR: u64 = 2;

/// Dampening factor to use for difficulty adjustment
pub const DIFFICULTY_DAMP_FACTOR: u64 = 3;

/// Dampening factor to use for AR scale calculation.
pub const AR_SCALE_DAMP_FACTOR: u64 = 13;

/// Compute weight of a graph as number of siphash bits defining the graph
/// Must be made dependent on height to phase out smaller size over the years
/// This can wait until end of 2019 at latest
pub fn graph_weight(height: u64, edge_bits: u8) -> u64 {
	if edge_bits <= 31 {
		(2u64 << ((edge_bits as u64) - global::base_edge_bits() as u64) as u64) * (edge_bits as u64)
	} else {
		1
	}
}

/// Minimum difficulty, enforced in diff retargetting
/// avoids getting stuck when trying to increase difficulty subject to dampening
pub const MIN_DIFFICULTY: u64 = DIFFICULTY_DAMP_FACTOR;

/// Minimum scaling factor for AR pow, enforced in diff retargetting
/// avoids getting stuck when trying to increase ar_scale subject to dampening
pub const MIN_AR_SCALE: u64 = AR_SCALE_DAMP_FACTOR;

/// unit difficulty, equal to graph_weight(SECOND_POW_EDGE_BITS)
pub const UNIT_DIFFICULTY: u64 =
	((2 as u64) << (SECOND_POW_EDGE_BITS - BASE_EDGE_BITS)) * (SECOND_POW_EDGE_BITS as u64);

/// The initial difficulty at launch. This should be over-estimated
/// and difficulty should come down at launch rather than up
/// Currently grossly over-estimated at 10% of current
/// ethereum GPUs (assuming 1GPU can solve a block at diff 1 in one block interval)
pub const INITIAL_DIFFICULTY: u64 = 1_000_000 * UNIT_DIFFICULTY;

/// Minimal header information required for the Difficulty calculation to
/// take place
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeaderInfo {
	/// Block hash, ZERO_HASH when this is a sythetic entry.
	pub block_hash: Hash,
	/// Timestamp of the header, 1 when not used (returned info)
	pub timestamp: u64,
	/// Network difficulty or next difficulty to use
	pub difficulty: Difficulty,
	/// Network secondary PoW factor or factor to use
	pub secondary_scaling: u32,
	/// Whether the header is a secondary proof of work
	pub is_secondary: bool,
}

impl HeaderInfo {
	/// Default constructor
	pub fn new(
		block_hash: Hash,
		timestamp: u64,
		difficulty: Difficulty,
		secondary_scaling: u32,
		is_secondary: bool,
	) -> HeaderInfo {
		HeaderInfo {
			block_hash,
			timestamp,
			difficulty,
			secondary_scaling,
			is_secondary,
		}
	}

	/// Constructor from a timestamp and difficulty, setting a default secondary
	/// PoW factor
	pub fn from_ts_diff(timestamp: u64, difficulty: Difficulty) -> HeaderInfo {
		HeaderInfo {
			block_hash: ZERO_HASH,
			timestamp,
			difficulty,
			secondary_scaling: global::initial_graph_weight(),

			is_secondary: true,
		}
	}

	/// Constructor from a difficulty and secondary factor, setting a default
	/// timestamp
	pub fn from_diff_scaling(difficulty: Difficulty, secondary_scaling: u32) -> HeaderInfo {
		HeaderInfo {
			block_hash: ZERO_HASH,
			timestamp: 1,
			difficulty,
			secondary_scaling,
			is_secondary: true,
		}
	}
}

/// Move value linearly toward a goal
pub fn damp(actual: u64, goal: u64, damp_factor: u64) -> u64 {
	(actual + (damp_factor - 1) * goal) / damp_factor
}

/// limit value to be within some factor from a goal
pub fn clamp(actual: u64, goal: u64, clamp_factor: u64) -> u64 {
	max(goal / clamp_factor, min(actual, goal * clamp_factor))
}

/// Computes the proof-of-work difficulty that the next block should comply
/// with. Takes an iterator over past block headers information, from latest
/// (highest height) to oldest (lowest height).
///
/// The difficulty calculation is based on both Digishield and GravityWave
/// family of difficulty computation, coming to something very close to Zcash.
/// The reference difficulty is an average of the difficulty over a window of
/// DIFFICULTY_ADJUST_WINDOW blocks. The corresponding timespan is calculated
/// by using the difference between the median timestamps at the beginning
/// and the end of the window.
///
/// The secondary proof-of-work factor is calculated along the same lines, as
/// an adjustment on the deviation against the ideal value.
pub fn next_difficulty<T>(height: u64, cursor: T) -> HeaderInfo
where
	T: IntoIterator<Item = HeaderInfo>,
{
	// Create vector of difficulty data running from earliest
	// to latest, and pad with simulated pre-genesis data to allow earlier
	// adjustment if there isn't enough window data length will be
	// DIFFICULTY_ADJUST_WINDOW + 1 (for initial block time bound)
	let diff_data = global::difficulty_data_to_vector(cursor);

	// First, get the ratio of secondary PoW vs primary, skipping initial header
	let sec_pow_scaling = secondary_pow_scaling(height, &diff_data[1..]);

	// Get the timestamp delta across the window
	let ts_delta: u64 =
		diff_data[DIFFICULTY_ADJUST_WINDOW as usize].timestamp - diff_data[0].timestamp;

	// Get the difficulty sum of the last DIFFICULTY_ADJUST_WINDOW elements
	let diff_sum: u64 = diff_data
		.iter()
		.skip(1)
		.map(|dd| dd.difficulty.to_num())
		.sum();

	// adjust time delta toward goal subject to dampening and clamping
	let adj_ts = clamp(
		damp(ts_delta, BLOCK_TIME_WINDOW, DIFFICULTY_DAMP_FACTOR),
		BLOCK_TIME_WINDOW,
		CLAMP_FACTOR,
	);
	// minimum difficulty avoids getting stuck due to dampening
	let difficulty = max(MIN_DIFFICULTY, diff_sum * BLOCK_TIME_SEC / adj_ts);

	HeaderInfo::from_diff_scaling(Difficulty::from_num(difficulty), sec_pow_scaling)
}

/// Count, in units of 1/100 (a percent), the number of "secondary" (AR) blocks in the provided window of blocks.
pub fn ar_count(_height: u64, diff_data: &[HeaderInfo]) -> u64 {
	100 * diff_data.iter().filter(|n| n.is_secondary).count() as u64
}

/// Factor by which the secondary proof of work difficulty will be adjusted
pub fn secondary_pow_scaling(height: u64, diff_data: &[HeaderInfo]) -> u32 {
	// Get the scaling factor sum of the last DIFFICULTY_ADJUST_WINDOW elements
	let scale_sum: u64 = diff_data.iter().map(|dd| dd.secondary_scaling as u64).sum();

	// compute ideal 2nd_pow_fraction in pct and across window
	let target_pct = secondary_pow_ratio(height);
	let target_count = DIFFICULTY_ADJUST_WINDOW * target_pct;

	// Get the secondary count across the window, adjusting count toward goal
	// subject to dampening and clamping.
	let adj_count = clamp(
		damp(
			ar_count(height, diff_data),
			target_count,
			ar_scale_damp_factor(height),
		),
		target_count,
		CLAMP_FACTOR,
	);
	let scale = scale_sum * target_pct / max(1, adj_count);

	// minimum AR scale avoids getting stuck due to dampening
	max(MIN_AR_SCALE, scale) as u32
}

// Mimble has block reward schedule similar to bitcoin
/// Mimble Size of the block group
const MIMBLE_BLOCKS_PER_GROUP: u64 = 2_100_000; // 4 years
const MIMBLE_BLOCKS_PER_GROUP_FLOO: u64 = 2_880; // 2 days
/// Mimble Block reward for the first group
pub const MIMBLE_FIRST_GROUP_REWARD: u64 = 5_238_095_238;
pub const MIMBLE_SECOND_GROUP_REWARD: u64 = 2_380_952_380;
const MIMBLE_GROUPS_NUM: u64 = 32;
/// Calculate Mimble block reward. The scedure is similar to bitcoints.
/// 1st 2.1 million blocks - 5.142857143 Mimble - This period is "boosted", after that it's the default halfing.
/// 2nd 2.1 million blocks - 2.380952380 Mimble
/// 2nd 2.1 million blocks - 1.190476190 Mimble
/// 3rd 2.1 million blocks - 0.595238090 Mimble
/// 4th 2.1 million blocks - 0.297619040 Mimble
/// 5th 2.1 million blocks - 0.148809520 Mimble
/// 6th 2.1 million blocks - 0.074404760 Mimble
// ...
/// 32nd 2.1 million blocks - 0.000000001 Mimble
//All blocks after that - 0 Mimble (miner fees only)
pub fn calc_mwc_block_reward(height: u64) -> u64 {
	if height == 0 {
		// Genesis block
		return GENESIS_BLOCK_REWARD;
	}
	let group_num = if global::is_floonet() {
		(height-1) / MIMBLE_BLOCKS_PER_GROUP_FLOO
	} else {
		(height-1) / MIMBLE_BLOCKS_PER_GROUP
	};
	if group_num < 1 {
		let start_reward = MIMBLE_FIRST_GROUP_REWARD;
		return start_reward
		 // First period, increased reward to distribute more coins to first adopters
	} else if group_num >= MIMBLE_GROUPS_NUM {
		 0 // far far future, no rewards, sorry
	} else {
		//Still in a normal group, calc distribution 
		let start_reward = MIMBLE_SECOND_GROUP_REWARD * 2;
		let group_div = 1 << group_num;
		println!("{}", group_div);
		return start_reward / group_div
	}
}

/// Mimble  calculate the total number of rewarded coins in all blocks including this one
pub fn calc_mwc_block_overage(height: u64, genesis_had_reward: bool) -> u64 {
	let blocks_per_group = if global::is_floonet() {
		MIMBLE_BLOCKS_PER_GROUP_FLOO
	} else {
		MIMBLE_BLOCKS_PER_GROUP
	};

	// including this one happens implicitly.
	// Because "this block is included", but 0 block (genesis) block is excluded, we will keep height as it is
	let mut block_count = height;
	let reward_per_block = MIMBLE_SECOND_GROUP_REWARD;
	let boostedreward_per_block = MIMBLE_FIRST_GROUP_REWARD;
	let mut overage: u64 = GENESIS_BLOCK_REWARD; // genesis block reward

	for _x in 0..MIMBLE_GROUPS_NUM {
		if _x == 0 {
			//exclude first froup due to special rewards, after that go back to main distribution plan
			overage += min(block_count, blocks_per_group) * boostedreward_per_block;
		} else {
			overage += min(block_count, blocks_per_group) * calc_mwc_block_reward(_x * blocks_per_group +  1);
		}
		if block_count < blocks_per_group {
			break;
		}

		block_count -= blocks_per_group;
	}

	if !genesis_had_reward {
		// Deducting the first block reward if it is 0. This case is used into the tests.
		overage -= GENESIS_BLOCK_REWARD;
	}

	overage
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_graph_weight() {
		// initial weights
		assert_eq!(graph_weight(1, 29), 64 * 29);
		assert_eq!(graph_weight(1, 31), 256 * 31);
		assert_eq!(graph_weight(1, 32), 1);
		assert_eq!(graph_weight(1, 33), 1);

		// Make sure nothing changes after a Year, no changes planned
		assert_eq!(graph_weight(YEAR_HEIGHT, 31), 256 * 31);
		assert_eq!(graph_weight(YEAR_HEIGHT, 32), 1);
		assert_eq!(graph_weight(YEAR_HEIGHT, 33), 1);

		
	}

	// Mimble  testing calc_mwc_block_reward output for the scedule that documented at definition of calc_mwc_block_reward
	#[test]
	fn test_calc_mwc_block_reward() {
		// Code is crucial, so just checking all groups one by one manually.
		// We don't use the constants here because we can mess up with them as well.
		assert_eq!(calc_mwc_block_reward(0), 44_100_000 ); // group 1, genesis ( 44_100_000 to match exactly 21KK)
		assert_eq!(calc_mwc_block_reward(1), MIMBLE_FIRST_GROUP_REWARD); // group 1 2.84695238
		assert_eq!(calc_mwc_block_reward(2), MIMBLE_FIRST_GROUP_REWARD); // group 1
        	assert_eq!(calc_mwc_block_reward(2_100_000 - 1), MIMBLE_FIRST_GROUP_REWARD); // group 1
        	assert_eq!(calc_mwc_block_reward(2_100_000), MIMBLE_FIRST_GROUP_REWARD); // group 1
		assert_eq!(calc_mwc_block_reward(MIMBLE_BLOCKS_PER_GROUP - 1), MIMBLE_FIRST_GROUP_REWARD); // group 1
       	 	assert_eq!(calc_mwc_block_reward(MIMBLE_BLOCKS_PER_GROUP), MIMBLE_FIRST_GROUP_REWARD); // group 1
		assert_eq!(calc_mwc_block_reward(2_100_000+1), MIMBLE_SECOND_GROUP_REWARD); // group 2
		assert_eq!(calc_mwc_block_reward(MIMBLE_BLOCKS_PER_GROUP + 1), MIMBLE_SECOND_GROUP_REWARD); // group 2
		assert_eq!(calc_mwc_block_reward(2_100_000 + 200), MIMBLE_SECOND_GROUP_REWARD); // group 2
		assert_eq!(calc_mwc_block_reward(2_100_000 * 2 + 200), MIMBLE_SECOND_GROUP_REWARD /2); // group 21_190_476_190
		assert_eq!(calc_mwc_block_reward(2_100_000 * 3 + 200), MIMBLE_SECOND_GROUP_REWARD /4); // group 4595_238_095
		assert_eq!(calc_mwc_block_reward(2_100_000 * 4 + 200), MIMBLE_SECOND_GROUP_REWARD /8); // group 5297_619_047
		assert_eq!(calc_mwc_block_reward(2_100_000 * 5 + 200), MIMBLE_SECOND_GROUP_REWARD /16);
		assert_eq!(calc_mwc_block_reward(2_100_000 * 6 + 200), MIMBLE_SECOND_GROUP_REWARD /32); // group 6
		assert_eq!(calc_mwc_block_reward(2_100_000 * 7 + 200), MIMBLE_SECOND_GROUP_REWARD /64); // group 7
		assert_eq!(calc_mwc_block_reward(2_100_000 * 8 + 200), MIMBLE_SECOND_GROUP_REWARD /128); // group 8
		assert_eq!(calc_mwc_block_reward(2_100_000 * 9 + 200), MIMBLE_SECOND_GROUP_REWARD /256); // group 9
		assert_eq!(calc_mwc_block_reward(2_100_000 * 10 + 200), MIMBLE_SECOND_GROUP_REWARD /512); // group 10
		assert_eq!(calc_mwc_block_reward(2_100_000 * 11 + 200), MIMBLE_SECOND_GROUP_REWARD /1024); // group 11
		assert_eq!(calc_mwc_block_reward(2_100_000 * 12 + 200), MIMBLE_SECOND_GROUP_REWARD /2048); // group 12
		assert_eq!(calc_mwc_block_reward(2_100_000 * 13 + 200), MIMBLE_SECOND_GROUP_REWARD /4096); // group 13

		assert_eq!(calc_mwc_block_reward(2_100_000 * 33 + 200), 0); // group 33+
		assert_eq!(calc_mwc_block_reward(2_100_000 * 320 + 200), 0); // group 33+
	}

	// MWC  testing calc_mwc_block_overage output for the schedule that documented at definition of calc_mwc_block_reward
	#[test]
	fn test_calc_mwc_block_overage() {
		let genesis_reward: u64 = GENESIS_BLOCK_REWARD;

		assert_eq!(calc_mwc_block_overage(0, true), genesis_reward); // Doesn't make sence to call for the genesis block

		assert_eq!(calc_mwc_block_reward(1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_001) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 2 + 1) * MIMBLE_BLOCKS_PER_GROUP 
+ calc_mwc_block_reward(2_100_000 * 3 + 1) * MIMBLE_BLOCKS_PER_GROUP 
+ calc_mwc_block_reward(2_100_000 * 4 + 1) * MIMBLE_BLOCKS_PER_GROUP 
+ calc_mwc_block_reward(2_100_000 * 5 + 1) * MIMBLE_BLOCKS_PER_GROUP 
+ calc_mwc_block_reward(2_100_000 * 6 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 7 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 8 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 9 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 10 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 11 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 12 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 13 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 14 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 15 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 16 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 17 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 18 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 19 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 20 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 21 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 22 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 23 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 24 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 25 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 26 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 27 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 28 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 29 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 30 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 31 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 32 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 33 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ calc_mwc_block_reward(2_100_000 * 34 + 1) * MIMBLE_BLOCKS_PER_GROUP
+ genesis_reward , 21_000_000 * MIMBLE_BASE ); // group 2
	
		assert_eq!(calc_mwc_block_overage(0, false), 0); // Doesn't make sence to call for the genesis block
		assert_eq!(
			calc_mwc_block_overage(1, true),
			genesis_reward + MIMBLE_FIRST_GROUP_REWARD * 1

		);

		assert_eq!(
			calc_mwc_block_overage(30, true),
			genesis_reward + MIMBLE_FIRST_GROUP_REWARD * 30
		);
		assert_eq!(
			calc_mwc_block_overage(30, false),
			MIMBLE_FIRST_GROUP_REWARD * 30
		);
		// pre last block in the first group
		assert_eq!(
			calc_mwc_block_overage(MIMBLE_BLOCKS_PER_GROUP - 1, true),
			genesis_reward + MIMBLE_FIRST_GROUP_REWARD * (MIMBLE_BLOCKS_PER_GROUP - 1)
		);
        // last block in the first group
		assert_eq!(
			calc_mwc_block_overage(MIMBLE_BLOCKS_PER_GROUP, true),
			genesis_reward + MIMBLE_FIRST_GROUP_REWARD * MIMBLE_BLOCKS_PER_GROUP
		);
        // first block in the second group
        assert_eq!(
            calc_mwc_block_overage(MIMBLE_BLOCKS_PER_GROUP + 1, true),
            genesis_reward
                + MIMBLE_FIRST_GROUP_REWARD * MIMBLE_BLOCKS_PER_GROUP 
		+ MIMBLE_SECOND_GROUP_REWARD                    
        );





        // 60th block in the second group
        assert_eq!(
            calc_mwc_block_overage(MIMBLE_BLOCKS_PER_GROUP + 60, true),
            genesis_reward
                + MIMBLE_FIRST_GROUP_REWARD * MIMBLE_BLOCKS_PER_GROUP
                + MIMBLE_SECOND_GROUP_REWARD * 60
        );

       // 60th block in the second group
        assert_eq!(
            calc_mwc_block_overage(MIMBLE_BLOCKS_PER_GROUP * 2 + 60, true),
            genesis_reward
                + MIMBLE_FIRST_GROUP_REWARD * MIMBLE_BLOCKS_PER_GROUP
                + MIMBLE_SECOND_GROUP_REWARD * MIMBLE_BLOCKS_PER_GROUP
                + MIMBLE_SECOND_GROUP_REWARD / 2 * 60
        );


		assert_eq!(
			calc_mwc_block_overage(MIMBLE_BLOCKS_PER_GROUP + 5000, true),
			genesis_reward

				+ MIMBLE_FIRST_GROUP_REWARD * MIMBLE_BLOCKS_PER_GROUP 
				+ 5000 * MIMBLE_SECOND_GROUP_REWARD 

		);

		// Calculating the total number of coins 
		let total_blocks_reward = calc_mwc_block_overage(2_100_000_000 * 320, true);
		// Expected 20M in total. The coin base is exactly 20M
		assert_eq!(calc_mwc_block_reward(1) * MIMBLE_BLOCKS_PER_GROUP, 5_238_095_238 * MIMBLE_BLOCKS_PER_GROUP);
		assert_eq!(calc_mwc_block_reward(2_100_001) * MIMBLE_BLOCKS_PER_GROUP, 2_380_952_380 * MIMBLE_BLOCKS_PER_GROUP);
		assert_eq!(calc_mwc_block_reward(2_100_000 * 3 + 1) * MIMBLE_BLOCKS_PER_GROUP, 595_238_095 * MIMBLE_BLOCKS_PER_GROUP);


assert_eq!( total_blocks_reward, 21_000_000 * MIMBLE_BASE );

	}
}
