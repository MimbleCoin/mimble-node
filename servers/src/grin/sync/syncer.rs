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

use grin_p2p::Peer;
use grin_p2p::PeerAddr;
use std::sync::Arc;
use std::thread;
use std::time;

use crate::chain::{self, SyncState, SyncStatus};
use crate::core::global;
use crate::core::pow::Difficulty;
use crate::grin::sync::body_sync::BodySync;
use crate::grin::sync::header_sync::HeaderSync;
use crate::grin::sync::state_sync::StateSync;
use crate::p2p;
use crate::util::StopState;

pub fn run_sync(
	sync_state: Arc<SyncState>,
	peers: Arc<p2p::Peers>,
	chain: Arc<chain::Chain>,
	stop_state: Arc<StopState>,
	duration_sync_long: i64,
	duration_sync_short: i64,
	header_cache_size: u64,
	preferred_peers: Option<Vec<PeerAddr>>,
) -> std::io::Result<std::thread::JoinHandle<()>> {
	thread::Builder::new()
		.name("sync".to_string())
		.spawn(move || {
			let runner = SyncRunner::new(sync_state, peers, chain, stop_state);
			runner.sync_loop(
				duration_sync_long,
				duration_sync_short,
				header_cache_size,
				preferred_peers,
			);
		})
}

pub struct SyncRunner {
	sync_state: Arc<SyncState>,
	peers: Arc<p2p::Peers>,
	chain: Arc<chain::Chain>,
	stop_state: Arc<StopState>,
}

impl SyncRunner {
	fn new(
		sync_state: Arc<SyncState>,
		peers: Arc<p2p::Peers>,
		chain: Arc<chain::Chain>,
		stop_state: Arc<StopState>,
	) -> SyncRunner {
		SyncRunner {
			sync_state,
			peers,
			chain,
			stop_state,
		}
	}

	fn wait_for_min_peers(&self) -> Result<(), chain::Error> {
		// Initial sleep to give us time to peer with some nodes.
		// Note: Even if we have skip peer wait we need to wait a
		// short period of time for tests to do the right thing.
		let wait_secs = if let SyncStatus::AwaitingPeers(true) = self.sync_state.status() {
			30
		} else {
			3
		};

		let head = self.chain.head()?;

		let mut n = 0;
		const MIN_PEERS: usize = 3;
		loop {
			if self.stop_state.is_stopped() {
				break;
			}
			let wp = self.peers.more_or_same_work_peers()?;
			// exit loop when:
			// * we have more than MIN_PEERS more_or_same_work peers
			// * we are synced already, e.g. grin was quickly restarted
			// * timeout
			if wp > MIN_PEERS
				|| (wp == 0
					&& self.peers.enough_outbound_peers()
					&& head.total_difficulty > Difficulty::zero())
				|| n > wait_secs
			{
				if wp > 0 || !global::is_production_mode() {
					break;
				}
			}
			thread::sleep(time::Duration::from_secs(1));
			n += 1;
		}
		Ok(())
	}

	/// Starts the syncing loop, just spawns two threads that loop forever
	fn sync_loop(
		&self,
		duration_sync_long: i64,
		duration_sync_short: i64,
		header_cache_size: u64,
		peers_preferred: Option<Vec<PeerAddr>>,
	) {
		macro_rules! unwrap_or_restart_loop(
	($obj: expr) =>(
		match $obj {
			Ok(v) => v,
			Err(e) => {
				error!("unexpected error: {:?}", e);
				thread::sleep(time::Duration::from_secs(1));
				continue;
			},
		}
	));

		// Wait for connections reach at least MIN_PEERS
		info!("Waiting for the peers");
		if let Err(e) = self.wait_for_min_peers() {
			error!("wait_for_min_peers failed: {:?}", e);
		}

		// Our 3 main sync stages
		let mut header_sync = HeaderSync::new(
			self.sync_state.clone(),
			self.peers.clone(),
			self.chain.clone(),
		);
		let mut body_sync = BodySync::new(
			self.sync_state.clone(),
			self.peers.clone(),
			self.chain.clone(),
		);
		let mut state_sync = StateSync::new(
			self.sync_state.clone(),
			self.peers.clone(),
			self.chain.clone(),
		);

		// Highest height seen on the network, generally useful for a fast test on
		// whether some sync is needed
		let mut highest_height = 0;

		// Header is blocked pretty often and can be locked for a long time.
		// As a result users see the false alarming message.
		// 'failed to obtain lock for try_header_head'
		// To make error reasonable,
		// We are adding counter, to reduce false alarms.
		let mut header_block_counter = 0;

		let mut try_smart_sync = true;
		thread::sleep(time::Duration::from_millis(1000));
		// Main syncing loop
		loop {
			if self.stop_state.is_stopped() {
				break;
			}

			thread::sleep(time::Duration::from_millis(10));

			let currently_syncing = self.sync_state.is_syncing();

			// check whether syncing is generally needed, when we compare our state with others
			let (needs_syncing, most_work_height, total_difficulty) =
				unwrap_or_restart_loop!(self.needs_syncing());
			if most_work_height > 0 {
				// we can occasionally get a most work height of 0 if read locks fail
				highest_height = most_work_height;
			}

			// quick short-circuit (and a decent sleep) if no syncing is needed
			if !needs_syncing {
				if currently_syncing {
					self.sync_state.update(SyncStatus::NoSync);

					// Initial transition out of a "syncing" state and into NoSync.
					// This triggers a chain compaction to keep out local node tidy.
					// Note: Chain compaction runs with an internal threshold
					// so can be safely run even if the node is restarted frequently.
					unwrap_or_restart_loop!(self.chain.compact());
				}

				// different approach from grin. Check more frequently.
				thread::sleep(time::Duration::from_millis(500));
				continue;
			}

			// needs syncing. first try smart sync
			if try_smart_sync {
				// only try once
				try_smart_sync = false;
				let res = self.smart_sync(total_difficulty, peers_preferred.clone());
				match res {
					Err(e) => {
						warn!(
							"Smart sync failed due to {:?}. Continuing with standard sync.",
							e
						);
					}
					_ => {}
				}
			}

			// if syncing is needed
			let head = unwrap_or_restart_loop!(self.chain.head());
			let tail = self.chain.tail().unwrap_or_else(|_| head.clone());

			// We still do not fully understand what is blocking this but if this blocks here after
			// we download and validate the txhashet we do not reliably proceed to block_sync,
			// potentially blocking for an extended period of time (> 10 mins).
			// Does not appear to be deadlock as it does resolve itself eventually.
			// So as a workaround we try_header_head with a relatively short timeout and simply
			// retry the syncer loop.
			let maybe_header_head =
				unwrap_or_restart_loop!(self.chain.try_header_head(time::Duration::from_secs(1)));

			// We are tolerating up to 60 retrys. During chain validation the chain access is blocked.
			// Normally in release and reasonable hardware 60 seconds more then is enough for that.
			// There will be bunch of threads waiting for the lock.
			if header_block_counter < 60 && maybe_header_head.is_none() {
				header_block_counter = header_block_counter + 1;
				thread::sleep(time::Duration::from_secs(1));
				continue;
			}

			// Header expected to be blocked duting the txhashset operations because it is pretty long
			let is_txhashset_operation = match self.sync_state.status() {
				SyncStatus::TxHashsetDownload { .. }
				| SyncStatus::TxHashsetSetup
				| SyncStatus::TxHashsetRangeProofsValidation { .. }
				| SyncStatus::TxHashsetKernelsValidation { .. }
				| SyncStatus::TxHashsetSave
				| SyncStatus::TxHashsetDone => true,
				_ => false,
			};
			if is_txhashset_operation && maybe_header_head.is_none() {
				thread::sleep(time::Duration::from_secs(1));
				continue;
			}

			let header_head = unwrap_or_restart_loop!(
				maybe_header_head.ok_or("failed to obtain lock for try_header_head. This error may be caused by running the debug version of this node, having a slow CPU, or having an unusually large blockchain.")
			);

			// lock was obtained, so we can reset the locking counter
			header_block_counter = 0;
			// run each sync stage, each of them deciding whether they're needed
			// except for state sync that only runs if body sync return true (means txhashset is needed)
			unwrap_or_restart_loop!(header_sync.check_run(
				&header_head,
				highest_height,
				duration_sync_long,
				duration_sync_short,
				header_cache_size,
			));

			let mut check_state_sync = false;
			match self.sync_state.status() {
				SyncStatus::TxHashsetDownload { .. }
				| SyncStatus::TxHashsetSetup
				| SyncStatus::TxHashsetRangeProofsValidation { .. }
				| SyncStatus::TxHashsetKernelsValidation { .. }
				| SyncStatus::TxHashsetSave
				| SyncStatus::TxHashsetDone => check_state_sync = true,
				_ => {
					// skip body sync if header chain is not synced.
					if header_head.height < highest_height {
						continue;
					}

					let check_run = match body_sync.check_run(&head, highest_height) {
						Ok(v) => v,
						Err(e) => {
							error!("check_run failed: {:?}", e);
							continue;
						}
					};

					if check_run {
						check_state_sync = true;
					}
				}
			}

			if check_state_sync {
				state_sync.check_run(&header_head, &head, &tail, highest_height);
			}
		}
	}

	/// Smart sync is attempted once by calling the peers that are in the peers_preferred list.
	/// if it fails, we revert to standard syncing procedures which may be slower.
	fn smart_sync(
		&self,
		most_work_difficulty: u64,
		peers_preferred: Option<Vec<PeerAddr>>,
	) -> Result<(), chain::Error> {
		let peers_preferred = peers_preferred.unwrap_or(vec![]);
		info!("peers = {:?}", peers_preferred);

		let mut pp = vec![];
		for peer in peers_preferred {
			let peer = self.peers.get_connected_peer(peer);
			if peer.is_some() {
				pp.push(peer.unwrap());
			}
		}

		info!("pp.len = {}, pp = {:?}", pp.len(), pp);

		if pp.len() == 0 {
			Err(
				chain::ErrorKind::SyncError("no smart sync servers specified, please use peers_preferred in mwc-server.toml to specify them, reverting to regular sync".to_string())
				.into(),
			)
		} else {
			if let Err(e) = self.do_smart_sync(pp, most_work_difficulty) {
				Err(chain::ErrorKind::SyncError(
					format!("smart_sync failed due to {}, reverting to regular sync", e)
						.to_string(),
				)
				.into())
			} else {
				Ok(())
			}
		}
	}

	fn do_smart_sync(
		&self,
		smart_peers: Vec<Arc<Peer>>,
		most_work_difficulty: u64,
	) -> Result<(), chain::Error> {
		for peer in smart_peers {
			let res = peer.send_ping(Difficulty::from_num(most_work_difficulty), 0);
			info!("res from peer {:?} was {:?}", peer, res);
		}
		Ok(())
	}

	/// Whether we're currently syncing the chain or we're fully caught up and
	/// just receiving blocks through gossip.
	fn needs_syncing(&self) -> Result<(bool, u64, u64), chain::Error> {
		let local_diff = self.chain.head()?.total_difficulty;
		let mut is_syncing = self.sync_state.is_syncing();
		let peer = self.peers.most_work_peer();

		let peer_info = if let Some(p) = peer {
			p.info.clone()
		} else {
			warn!("sync: no peers available, disabling sync");
			return Ok((false, 0, 0));
		};

		// if we're already syncing, we're caught up if no peer has a higher
		// difficulty than us
		if is_syncing {
			if peer_info.total_difficulty() <= local_diff {
				let ch = self.chain.head()?;
				info!(
					"synchronized at {} @ {} [{}]",
					local_diff.to_num(),
					ch.height,
					ch.last_block_h
				);
				is_syncing = false;
			}
		} else {
			// sum the last 5 difficulties to give us the threshold
			let threshold = {
				let diff_iter = match self.chain.difficulty_iter() {
					Ok(v) => v,
					Err(e) => {
						error!("failed to get difficulty iterator: {:?}", e);
						// we handle 0 height in the caller
						return Ok((false, 0, 0));
					}
				};
				diff_iter
					.map(|x| x.difficulty)
					.take(5)
					.fold(Difficulty::zero(), |sum, val| sum + val)
			};

			let peer_diff = peer_info.total_difficulty();
			if peer_diff > local_diff.clone() + threshold.clone() {
				info!(
					"sync: total_difficulty {}, peer_difficulty {}, threshold {} (last 5 blocks), enabling sync",
					local_diff,
					peer_diff,
					threshold,
				);
				is_syncing = true;
			}
		}
		Ok((
			is_syncing,
			peer_info.height(),
			peer_info.total_difficulty().to_num(),
		))
	}
}
