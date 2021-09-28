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

mod common;

use common::*;
use simple_raft::message::{RaftMessage, Rpc, VoteResponse};

#[test]
pub fn empty_group_become_leader() {
    let mut raft = raft(1, vec![], None, &mut init_random());
    assert!(!raft.is_leader());

    raft.timeout();
    assert!(raft.is_leader());
}

#[test]
pub fn _1_peer_become_leader() {
    let mut raft = raft(1, vec![2], None, &mut init_random());
    assert!(!raft.is_leader());

    let RaftMessage { term, .. } = raft.timeout().unwrap().message;
    assert!(!raft.is_leader());

    send(&mut raft, 2, term, Rpc::VoteResponse(VoteResponse { vote_granted: true }));
    assert!(raft.is_leader());
}

#[test]
pub fn become_leader() {
    let mut raft = raft(1, vec![2, 3], None, &mut init_random());
    assert!(!raft.is_leader());

    let RaftMessage { term, .. } = raft.timeout().unwrap().message;
    assert!(!raft.is_leader());

    send(&mut raft, 2, term, Rpc::VoteResponse(VoteResponse { vote_granted: false }));
    assert!(!raft.is_leader());

    send(&mut raft, 3, term, Rpc::VoteResponse(VoteResponse { vote_granted: true }));
    assert!(raft.is_leader());
}

#[test]
pub fn vote_old_term() {
    let mut raft = raft(1, vec![2, 3], None, &mut init_random());
    let RaftMessage { term, .. } = raft.timeout().unwrap().message;
    raft.timeout();

    send(&mut raft, 2, term, Rpc::VoteResponse(VoteResponse { vote_granted: true }));
    assert!(!raft.is_leader());
}

#[test]
pub fn vote_twice() {
    let mut raft = raft(1, vec![2, 3, 4, 5], None, &mut init_random());
    let RaftMessage { term, .. } = raft.timeout().unwrap().message;

    send(&mut raft, 2, term, Rpc::VoteResponse(VoteResponse { vote_granted: true }));
    send(&mut raft, 2, term, Rpc::VoteResponse(VoteResponse { vote_granted: true }));
    assert!(!raft.is_leader());

    send(&mut raft, 3, term, Rpc::VoteResponse(VoteResponse { vote_granted: true }));
    assert!(raft.is_leader());
}

#[test]
pub fn _1_timeout() {
    TestRaftGroup::new(1, &mut init_random(), config())
        .run_on_node(0, |raft| raft.timeout())
        .inspect(|group| assert!(group.has_leader()));
}

#[test]
pub fn _2_nodes_timeout() {
    TestRaftGroup::new(2, &mut init_random(), config())
        .run_on_node(0, |raft| raft.timeout())
        .inspect(|group| assert!(group.has_leader()));
}

#[test]
pub fn _2_nodes_failed_timeout() {
    TestRaftGroup::new(2, &mut init_random(), config().node_down(1))
        .run_on_node(0, |raft| raft.timeout())
        .inspect(|group| assert!(!group.has_leader()));
}

#[test]
pub fn _3_nodes_timeout() {
    TestRaftGroup::new(3, &mut init_random(), config())
        .run_on_node(0, |raft| raft.timeout())
        .inspect(|group| assert!(group.has_leader()));
}

#[test]
pub fn _3_nodes_degraded_timeout() {
    TestRaftGroup::new(3, &mut init_random(), config().isolate(1))
        .run_on_node(0, |raft| raft.timeout())
        .inspect(|group| assert!(group.has_leader()));
}

#[test]
pub fn _3_nodes_split_timeout() {
    TestRaftGroup::new(3, &mut init_random(), config().drop_between(0, 1))
        .run_on_node(0, |raft| raft.timeout())
        .inspect(|group| assert!(group.has_leader()));
}

#[test]
pub fn _3_nodes_failed_timeout() {
    TestRaftGroup::new(3, &mut init_random(), config().node_down(1).node_down(2))
        .run_on_node(0, |raft| raft.timeout())
        .inspect(|group| assert!(!group.has_leader()));
}

#[test]
pub fn _4_nodes_degraded_timeout() {
    TestRaftGroup::new(4, &mut init_random(), config().isolate(1))
        .run_on_node(0, |raft| raft.timeout())
        .inspect(|group| assert!(group.has_leader()));
}

#[test]
pub fn _4_nodes_failed_timeout() {
    TestRaftGroup::new(4, &mut init_random(), config().isolate(1).isolate(2))
        .run_on_node(0, |raft| raft.timeout())
        .inspect(|group| assert!(!group.has_leader()));
}

#[test]
pub fn _5_nodes_degraded_timeout() {
    TestRaftGroup::new(5, &mut init_random(), config().isolate(1).isolate(2))
        .run_on_node(0, |raft| raft.timeout())
        .inspect(|group| assert!(group.has_leader()));
}

#[test]
pub fn _5_nodes_failed_timeout() {
    TestRaftGroup::new(5, &mut init_random(), config().isolate(1).isolate(2).isolate(3))
        .run_on_node(0, |raft| raft.timeout())
        .inspect(|group| assert!(!group.has_leader()));
}

#[test]
pub fn election_timeout() {
    TestRaftGroup::new(3, &mut init_random(), config())
        .run_until(|group| group.has_leader())
        .run_for_inspect(10 * CONFIG.election_timeout_ticks, |group| assert!(group.has_leader()));
}

#[test]
pub fn degraded() {
    TestRaftGroup::new(3, &mut init_random(), config().isolate(0))
        .run_until(|group| group.has_leader())
        .run_for_inspect(10 * CONFIG.election_timeout_ticks, |group| assert!(group.has_leader()));
}

#[test]
pub fn split_unstable() {
    TestRaftGroup::new(3, &mut init_random(), config().drop_between(1, 2))
        .run_on_node(1, |raft| raft.timeout())
        .inspect(|group| assert!(group.nodes[1].is_leader()))
        .run_until(|group| !group.has_leader())
        .run_until(|group| group.has_leader());
}

#[test]
pub fn split_stable() {
    TestRaftGroup::new(3, &mut init_random(), config().drop_between(1, 2))
        .run_on_node(0, |raft| raft.timeout())
        .run_for_inspect(10 * CONFIG.election_timeout_ticks, |group| assert!(group.nodes[0].is_leader()));
}

#[test]
pub fn split_rejoin() {
    TestRaftGroup::new(3, &mut init_random(), config().drop_between(1, 2))
        .run_on_node(1, |raft| raft.timeout())
        .inspect(|group| assert!(group.nodes[1].is_leader()))
        .run_until(|group| !group.has_leader())
        .modify(|group| group.config = config())
        .run_until(|group| group.has_leader())
        .run_for_inspect(10 * CONFIG.election_timeout_ticks, |group| assert!(group.has_leader()));
}
