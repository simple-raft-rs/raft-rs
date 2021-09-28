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
use simple_raft::message::{RaftMessage, Rpc, TermId, VoteResponse};

#[test]
pub fn leader_update_term() {
    for rpc in rpc_types().iter().cloned() {
        let mut raft = raft(1, vec![2, 3], None, &mut init_random());
        let mut term = TermId::default();
        assert_eq!(raft.leader().1, &term);

        term += 1;
        let RaftMessage { term: new_term, .. } = raft.timeout().unwrap().message;
        assert_eq!(new_term, term);
        assert_eq!(raft.leader().1, &term);

        send(&mut raft, 2, term, Rpc::VoteResponse(VoteResponse { vote_granted: true }));
        assert_eq!(raft.leader(), (Some(raft.node_id()), &term));

        term += 1;
        send(&mut raft, 2, term, rpc);
        assert_eq!(raft.leader().1, &term);
    }
}

#[test]
pub fn candidate_update_term() {
    for rpc in rpc_types().iter().cloned() {
        let mut raft = raft(1, vec![2, 3], None, &mut init_random());
        let mut term = TermId::default();
        assert_eq!(raft.leader().1, &term);

        term += 1;
        let RaftMessage { term: new_term, .. } = raft.timeout().unwrap().message;
        assert_eq!(new_term, term);
        assert_eq!(raft.leader(), (None, &term));

        term += 1;
        send(&mut raft, 2, term, rpc);
        assert_eq!(raft.leader().1, &term);
    }
}

#[test]
pub fn follower_update_term() {
    for rpc in rpc_types().iter().cloned() {
        let mut raft = raft(1, vec![2, 3], None, &mut init_random());
        let mut term = TermId::default();
        assert_eq!(raft.leader(), (None, &term));

        term += 1;
        send(&mut raft, 2, term, rpc);
        assert_eq!(raft.leader().1, &term);
    }
}
