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

use alloc::collections::VecDeque;
use core::convert::{TryFrom, TryInto};
use crate::message::{LogEntry, LogIdx, TermId};
use super::{RaftLog, RaftLogAppendError};

pub struct RaftLogMemory {
    entries:       VecDeque<LogEntry>,
    prev_log_idx:  LogIdx,
    prev_log_term: TermId,
    data_len:      usize,
    data_capacity: usize,
}

impl RaftLogMemory {
    pub fn new(entries_initial_capacity: usize, data_capacity: usize) -> Self {
        Self {
            entries:       VecDeque::with_capacity(entries_initial_capacity),
            prev_log_idx:  LogIdx::default(),
            prev_log_term: TermId::default(),
            data_len:      0,
            data_capacity,
        }
    }

    fn entry_index(&self, log_idx: LogIdx) -> Option<usize> {
        log_idx.id
               .checked_sub(self.prev_log_idx.id)?
               .checked_sub(1)?
               .try_into()
               .ok()
    }
}

impl RaftLog for RaftLogMemory {
    fn append(&mut self, log_entry: LogEntry) -> Result<(), RaftLogAppendError> {
        if log_entry.data.len() > self.data_capacity {
            return Err(RaftLogAppendError::TooLarge { size: log_entry.data.len() });
        }
        self.data_len = match self.data_len.checked_add(log_entry.data.len()) {
            Some(new_data_len) if new_data_len <= self.data_capacity =>
                new_data_len,
            Some(_) | None =>
                return Err(RaftLogAppendError::OutOfSpace { log_entry }),
        };
        self.entries.push_back(log_entry);
        Ok(())
    }
    fn pop_front(&mut self, truncate_to: LogIdx) -> Result<(), ()> {
        self.entry_index(truncate_to)
            .ok_or(())?;
        let prev_log = self.entries.pop_front().ok_or(())?;
        self.prev_log_idx = self.prev_log_idx + 1;
        self.prev_log_term = prev_log.term;
        Ok(())
    }
    fn cancel_from(&mut self, from_log_idx: LogIdx) -> Result<usize, ()> {
        let from_index = self.entry_index(from_log_idx).ok_or(())?;
        match self.entries.len().checked_sub(from_index) {
            Some(0) | None =>
                Err(()),
            Some(cancelled_len) => {
                self.entries.truncate(from_index);
                Ok(cancelled_len)
            }
        }
    }
    fn entry_len(&self, log_entry: &LogEntry) -> usize {
        log_entry.data.capacity()
    }
    fn get(&mut self, log_idx: LogIdx) -> Option<LogEntry> {
        let index = self.entry_index(log_idx)?;
        self.entries.get(index).cloned()
    }
    fn get_term(&mut self, log_idx: LogIdx) -> Option<TermId> {
        if log_idx != self.prev_log_idx {
            self.get(log_idx)
                .map(|log_entry: LogEntry| log_entry.term)
        } else {
            Some(self.prev_log_term)
        }
    }
    fn get_len(&mut self, log_idx: LogIdx) -> Option<usize> {
        self.get(log_idx)
            .map(|log_entry: LogEntry| log_entry.data.len())
    }
    fn prev_idx(&self) -> LogIdx {
        self.prev_log_idx
    }
    fn last_idx(&self) -> LogIdx {
        let entries_len = u64::try_from(self.entries.len())
            .unwrap_or_else(|_| panic!("more than 2^64 log entries"));
        self.prev_log_idx + entries_len
    }
    fn last_term(&self) -> TermId {
        self.entries
            .iter()
            .map(|log_entry: &LogEntry| log_entry.term)
            .last()
            .unwrap_or(self.prev_log_term)
    }
}

#[cfg(test)]
mod test {
    use crate::raft_log_tests;
    use super::*;

    raft_log_tests!(RaftLogMemory, RaftLogMemory::new(0, usize::max_value()));
}
