/*
 * Copyright (C) 2021 jessa0
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

//! Raft consensus algorithm implementation.
//!
//! Raft is a consensus algorithm which replicates a strongly-consistent distributed log of entries with arbitrary data
//! amongst a group of peers. It is also fault-tolerant, allowing replication to continue while a majority of peers can
//! still communicate with each other. This crate provides an implementation of the Raft consensus algorithm with some
//! optional features not implemented, such as pre-voting, membership changes, and snapshots.
//!
//! The Raft algorithm is implemented as a state machine driven in a few ways:
//!
//! * When attempting to append a new entry to the distributed log: [`append`](node::RaftNode::append) is called.
//! * When a message is received from a peer: [`receive`](node::RaftNode::receive) is called.
//! * Every time a fixed amount of time has elapsed: [`timer_tick`](node::RaftNode::timer_tick) is called.
//!
//! Each of these functions modifies the internal state and returns [messages](message::SendableRaftMessage) to be sent
//! to peers. Once a log entry is "committed", or guaranteed to be returned at the same index on every functioning peer
//! in the group, it may be retrieved using [`take_committed`](node::RaftNode::take_committed). An append to the log may
//! be cancelled before reaching the committed state, however, which is discussed in more detail in ["Appending entries to the distributed log"].
//!
//! The backing storage for the distributed log must be provided as an implementation of the [`RaftLog`](log::RaftLog)
//! trait, with careful attention to following the trait specification. A trivial in-memory implementation is provided
//! by [`RaftLogMemory`](log::mem::RaftLogMemory).
//!
//! # Example
//!
//! ```
//! use raft::log::mem::RaftLogMemory;
//! use raft::node::{RaftConfig, RaftNode};
//! use raft::message::RaftMessageDestination;
//! use rand_chacha::ChaChaRng;
//! use rand_core::SeedableRng;
//! use std::collections::VecDeque;
//! use std::str;
//!
//! // Construct 5 Raft peers
//! let config = RaftConfig {
//!     election_timeout_ticks: 10,
//!     heartbeat_interval_ticks: 1,
//!     replication_chunk_size: usize::max_value(),
//! };
//! let mut peers = (0..5)
//!     .map(|id| RaftNode::new(id, (0..5).collect(),
//!                             RaftLogMemory::new_unbounded(),
//!                             ChaChaRng::seed_from_u64(id as u64),
//!                             config.clone()))
//!     .collect::<Vec<_>>();
//!
//! // Loop until a log entry is committed on all peers
//! let mut appended = false;
//! let mut peers_committed = vec![false; peers.len()];
//! let mut messages = VecDeque::new();
//! while !peers_committed.iter().all(|seen| *seen) {
//!     // Tick the timer on each peer
//!     for (peer_id, peer) in peers.iter_mut().enumerate() {
//!         let new_messages = peer.timer_tick();
//!         messages.extend(new_messages.map(|message| (peer_id, message)));
//!     }
//!
//!     // Append a log entry on the leader
//!     for (peer_id, peer) in peers.iter_mut().enumerate() {
//!         if !appended && peer.is_leader() {
//!             if let Ok(new_messages) = peer.append("Hello world!") {
//!                 messages.extend(new_messages.map(|message| (peer_id, message)));
//!                 appended = true;
//!             }
//!         }
//!     }
//!
//!     // Deliver messages between peers
//!     while let Some((src_id, sendable)) = messages.pop_front() {
//!         let dst_ids = match sendable.dest {
//!             RaftMessageDestination::Broadcast  => 0..=peers.len() - 1,
//!             RaftMessageDestination::To(dst_id) => dst_id..=dst_id,
//!         };
//!         for (peer_id, peer) in dst_ids.clone().zip(&mut peers[dst_ids]) {
//!             let new_messages = peer.receive(sendable.message.clone(), src_id);
//!             messages.extend(new_messages.map(|message| (peer_id, message)));
//!         }
//!     }
//!
//!     // Check for committed log entries
//!     for (peer_id, peer) in peers.iter_mut().enumerate() {
//!         for log_entry in peer.take_committed() {
//!             if !log_entry.data.is_empty() {
//!                 println!("peer {} saw commit {}", peer_id, str::from_utf8(&log_entry.data).unwrap());
//!                 assert!(!peers_committed[peer_id]);
//!                 peers_committed[peer_id] = true;
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! ["Appending entries to the distributed log"]: node::RaftNode#appending-entries-to-the-distributed-log

#![no_std]

#![allow(unused_parens)]
#![warn(missing_docs)]

extern crate alloc;

#[macro_use]
mod macros;

pub mod core;
pub mod log;
pub mod message;
pub mod node;
mod prelude;
