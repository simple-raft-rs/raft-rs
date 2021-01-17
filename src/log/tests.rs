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

use bytes::Bytes;
use crate::message::{LogEntry, LogIdx, TermId};
use super::RaftLog;

//
// public API
//

#[macro_export]
macro_rules! raft_log_tests {
    ($ty:ty, $new:expr) => {
        $crate::raft_log_test! { $ty, $new, test_log_empty }
        $crate::raft_log_test! { $ty, $new, test_log_append }
        $crate::raft_log_test! { $ty, $new, test_log_cancel_from }
        $crate::raft_log_test! { $ty, $new, test_log_pop_front }
    };
}

#[macro_export]
macro_rules! raft_log_test {
    ($ty:ty, $new:expr, $test:ident) => {
        #[test]
        fn $test() {
            let mut log: $ty = $new;
            let dyn_log: &mut dyn $crate::log::RaftLog = &mut log;
            $crate::log::tests::$test(dyn_log);
        }
    }
}

pub fn test_log_empty(log: &mut dyn RaftLog) {
    verify_log(log, &[], LogIdx::default(), LogIdx::default());
}

pub fn test_log_append(log: &mut dyn RaftLog) {
    let entries = test_entries();
    for (index, entry) in entries.iter().cloned().enumerate() {
        log.append(entry).unwrap_or_else(|_| panic!());
        verify_log(log, &entries, LogIdx::default(), LogIdx { id: 1 + index as u64 });
    }

}

pub fn test_log_cancel_from(log: &mut dyn RaftLog) {
    let entries = append_test_entries(log);
    for &truncate_len in &[1, 2, 1] {
        let last_log_idx = log.last_idx();
        assert_eq!(log.cancel_from(last_log_idx + 2), Err(()));
        assert_eq!(log.cancel_from(last_log_idx + 1), Err(()));
        verify_log(log, &entries, LogIdx::default(), last_log_idx);
        assert_eq!(log.cancel_from(last_log_idx + 1 - truncate_len), Ok(truncate_len as usize));
        verify_log(log, &entries, LogIdx::default(), last_log_idx - truncate_len);
    }
    assert_eq!(log.cancel_from(log.last_idx() + 2), Err(()));
    assert_eq!(log.cancel_from(log.last_idx() + 1), Err(()));
}

pub fn test_log_pop_front(log: &mut dyn RaftLog) {
    let entries = append_test_entries(log);
    let mut prev_log_idx = LogIdx::default();
    let mut last_log_idx = log.last_idx();
    assert_eq!(log.pop_front(prev_log_idx), Err(()));
    verify_log(log, &entries, prev_log_idx, last_log_idx);

    prev_log_idx = prev_log_idx + 1;
    assert_eq!(log.pop_front(prev_log_idx), Ok(()));
    verify_log(log, &entries, prev_log_idx, last_log_idx);

    last_log_idx = last_log_idx - 1;
    assert_eq!(log.cancel_from(last_log_idx + 1), Ok(1));
    verify_log(log, &entries, prev_log_idx, last_log_idx);

    for _ in prev_log_idx.id..last_log_idx.id {
        prev_log_idx = prev_log_idx + 1;
        assert_eq!(log.pop_front(last_log_idx), Ok(()));
        verify_log(log, &entries, prev_log_idx, last_log_idx);
    }

    assert_eq!(log.pop_front(last_log_idx), Err(()));
    verify_log(log, &entries, prev_log_idx, last_log_idx);
}

//
// internal
//

fn test_entries() -> [LogEntry; 5] {
    [
        LogEntry { term: TermId { id: 1 }, data: Bytes::from_static(&[]) },
        LogEntry { term: TermId { id: 1 }, data: Bytes::from_static(&[2; 1]) },
        LogEntry { term: TermId { id: 2 }, data: Bytes::from_static(&[3; 2]) },
        LogEntry { term: TermId { id: 9 }, data: Bytes::from_static(&[4; 100]) },
        LogEntry { term: TermId { id: u64::max_value() }, data: Bytes::from_static(&[5; 100]) },
    ]
}

fn append_test_entries(log: &mut dyn RaftLog) -> [LogEntry; 5] {
    let entries = test_entries();
    entries.iter().cloned().for_each(|entry| log.append(entry).unwrap_or_else(|_| panic!()));
    entries
}

fn verify_log(log: &mut dyn RaftLog, entries: &[LogEntry], prev_log_idx: LogIdx, last_log_idx: LogIdx) {
    assert_eq!(log.prev_idx(), prev_log_idx);

    assert_eq!(log.get(LogIdx::default()), None);
    assert_eq!(log.get_len(LogIdx::default()), None);

    assert_eq!(log.get(prev_log_idx), None);
    assert_eq!(log.get_term(prev_log_idx), Some(prev_log_idx.id.checked_sub(1).map(|index| entries[index as usize].term).unwrap_or_default()));
    assert_eq!(log.get_len(prev_log_idx), None);

    assert_eq!(log.last_idx(), last_log_idx);
    assert_eq!(log.last_term(), log.last_idx().id.checked_sub(1).map(|index| entries[index as usize].term).unwrap_or_default());

    verify_entries(entries, prev_log_idx, last_log_idx, |log_idx, entry| {
        assert_eq!(log.get(log_idx).as_ref(), entry);
        assert_eq!(log.get_term(log_idx), entry.map(|entry| entry.term));
        assert_eq!(log.get_len(log_idx), entry.map(|entry| entry.data.len()));
    });
}

fn verify_entries<F>(entries: &[LogEntry], prev_log_idx: LogIdx, last_log_idx: LogIdx, mut fun: F)
where F: FnMut(LogIdx, Option<&LogEntry>),
{
    for log_index in 0..prev_log_idx.id {
        fun(LogIdx { id: log_index }, None);
    }
    for entry_index in prev_log_idx.id..last_log_idx.id {
        fun(LogIdx { id: 1 + entry_index }, Some(&entries[entry_index as usize]));
    }
    for entry_index in last_log_idx.id..=entries.len() as u64 {
        fun(LogIdx { id: 1 + entry_index }, None);
    }
}
