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

#[test]
pub fn _1_commit() {
    TestRaftGroup::new(1, &mut init_random(), config())
        .run_until(|group| group.has_leader())
        .modify(|group| assert!(group.nodes.iter_mut().any(|raft| raft.client_request("one".into()).is_ok())))
        .run_until_commit(|commit| { assert_eq!(commit.data, "one"); true });
}

#[test]
pub fn _2_commit() {
    TestRaftGroup::new(2, &mut init_random(), config())
        .run_until(|group| group.has_leader())
        .modify(|group| assert!(group.nodes.iter_mut().any(|raft| raft.client_request("one".into()).is_ok())))
        .run_until_commit(|commit| { assert_eq!(commit.data, "one"); true });
}

#[test]
pub fn _3_commit() {
    TestRaftGroup::new(3, &mut init_random(), config())
        .run_until(|group| group.has_leader())
        .modify(|group| assert!(group.nodes.iter_mut().any(|raft| raft.client_request("one".into()).is_ok())))
        .run_until_commit(|commit| { assert_eq!(commit.data, "one"); true });
}

#[test]
pub fn commit_leader_change() {
    let mut group = TestRaftGroup::new(3, &mut init_random(), config());
    group.run_on_node(0, |raft| raft.timeout());
    group.run_until(|group| group.nodes[0].is_leader());

    assert!(group.nodes[0].client_request("one".into()).is_ok());
    group.config = config().drop_to(0);
    group.run_for(1);

    assert!(group.take_committed().all(|commit| commit.data.is_empty()));
    group.config = config().isolate(0);
    group.run_until_commit(|commit| { assert_eq!(commit.data, "one"); true });
}

#[test]
pub fn cancel_uncommitted() {
    let mut group = TestRaftGroup::new(3, &mut init_random(), config());
    group.run_on_node(0, |raft| raft.timeout());
    group.run_until(|group| group.nodes[0].is_leader());

    assert!(group.nodes[0].client_request("one".into()).is_ok());
    group.config = config().isolate(0);
    group.run_until(|group| group.nodes[1..].iter().any(|raft| raft.is_leader()));

    assert!(group.nodes[1..].iter_mut().any(|raft| raft.client_request("two".into()).is_ok()));
    group.run_until_commit(|commit| { assert_eq!(commit.data, "two"); true });

    log::info!("committed two");
    group.config = config();
    group.run_until(|group| group.nodes[0].take_committed().any(|commit| {
        if !commit.data.is_empty() {
            assert_eq!(commit.data, "two");
            true
        } else {
            false
        }
    }));
}
