/*
 * Copyright (C) 2019 Open Whisper Systems
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

#[cfg(any(feature = "test", test))]
#[macro_use]
pub mod tests;
pub mod mem;

use crate::message::{LogEntry, LogIdx, TermId};

pub trait RaftLog {
    fn append(&mut self, log_entry: LogEntry) -> Result<(), RaftLogAppendError>;
    fn pop_front(&mut self, truncate_to: LogIdx) -> Result<(), ()>;
    fn cancel_from(&mut self, from_log_idx: LogIdx) -> Result<usize, ()>;
    fn entry_len(&self, log_entry: &LogEntry) -> usize;
    fn get(&mut self, log_idx: LogIdx) -> Option<LogEntry>;
    fn get_term(&mut self, log_idx: LogIdx) -> Option<TermId>;
    fn get_len(&mut self, log_idx: LogIdx) -> Option<usize>;
    fn prev_idx(&self) -> LogIdx;
    fn last_idx(&self) -> LogIdx;
    fn last_term(&self) -> TermId;
}

#[allow(variant_size_differences)]
pub enum RaftLogAppendError {
    TooLarge {
        size: usize,
    },
    OutOfSpace {
        log_entry: LogEntry,
    },
    InternalError,
}
