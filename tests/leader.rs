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
use raft::message::{Rpc, TermId};

#[test]
pub fn append_request_update_leader() {
    let mut raft = raft(1, vec![2], None, &mut init_random());
    assert!(!raft.is_leader());
    let (_, &(mut term)) = raft.leader();
    term += 1;

    send(&mut raft, 2, term, Rpc::AppendRequest(Default::default()));
    assert_eq!(raft.leader(), (Some(&2.into()), &term));
}

#[test]
pub fn no_update_leader() {
    for rpc in rpc_types().iter().cloned().filter(|rpc| !matches!(rpc, Rpc::AppendRequest(_))) {
        let mut raft = raft(1, vec![2, 3], None, &mut init_random());
        let mut term = TermId::default();
        assert_eq!(raft.leader(), (None, &term));

        term += 1;
        send(&mut raft, 2, term, rpc);
        assert_eq!(raft.leader(), (None, &term));
    }
}
