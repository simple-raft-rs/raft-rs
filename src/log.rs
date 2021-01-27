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

//! Types related to Raft log storage.
//!
//! Raft requires a backing storage for entries of its distributed log as they are being replicated to and from other
//! nodes. The [`RaftLog`] trait is implemented for that purpose, and the implementation is supplied to
//! [`RaftNode`](crate::node::RaftNode).

#[cfg(any(feature = "test", test))]
#[macro_use]
pub mod tests;
pub mod mem;

use core::iter;
use crate::message::{LogEntry, LogIndex, TermId};

/// An interface for storage of the Raft log of a [`RaftNode`](crate::node::RaftNode).
///
/// # Initial state
///
/// A Raft log is initialized as empty, with both [`prev_index`] and [`last_index`] returning
/// [`LogIndex::default()`](crate::message::LogIndex::default). The index of the first appended log entry is `1` and all
/// indices are contiguous.
///
/// # Log truncation
///
/// A Raft log of bounded size may discard old entries previously taken from the beginning of the log via [`take_next`]
/// if, for example, it runs out of space. However, the term of the last discarded entry is preserved to be returned
/// from [`prev_term`] if requested. The log can also be truncated explicitly from the end via [`cancel_from`].
///
/// [`append`]: Self::append
/// [`cancel_from`]: Self::cancel_from
/// [`last_index`]: Self::last_index
/// [`prev_index`]: Self::prev_index
/// [`prev_term`]: Self::prev_term
/// [`take_next`]: Self::take_next
pub trait RaftLog {
    /// The type of error returned by fallable operations.
    type Error;

    /// Appends an entry to the end of the log.
    ///
    /// # Errors
    ///
    /// If there was any error modifying the log, an error is returned.
    fn append(&mut self, entry: LogEntry) -> Result<(), Self::Error>;

    /// Cancels all entries including and after the entry at index `from_index`, removing them from the log. Returns the
    /// number of entries removed.
    ///
    /// # Errors
    ///
    /// If there was any error modifying the log, or if the entries did not exist, an error is returned.
    fn cancel_from(&mut self, from_index: LogIndex) -> Result<usize, Self::Error>;

    /// Returns the approximate serialized length in bytes of a given log entry.
    fn entry_len(&self, entry: &LogEntry) -> usize;

    /// Returns the entry at a given index, or `None` if the index is greater than the length of the log or if the entry
    /// has been discarded.
    fn get(&mut self, index: LogIndex) -> Option<LogEntry>;

    /// Returns the term of the entry at a given index, or `None` if the index is greater than the length of the log or
    /// if the entry has been discarded.
    fn get_term(&mut self, index: LogIndex) -> Option<TermId>;

    /// Returns the approximate serialized length of the entry at a given index, or `None` if the index is greater than
    /// the length of the log or if the entry has been discarded.
    fn get_len(&mut self, index: LogIndex) -> Option<usize> {
        self.get(index).map(|entry: LogEntry| self.entry_len(&entry))
    }

    /// Returns the index of the last entry which has been returned by [`take_next`], or
    /// [`LogIndex::default()`](crate::message::LogIndex::default) if none have been.
    ///
    /// [`take_next`]: Self::take_next
    /// [`LogEntry`]: crate::message::LogEntry
    fn last_taken_index(&self) -> LogIndex;

    /// Returns the index of the last entry in the log, or [`LogIndex::default()`](crate::message::LogIndex::default) if
    /// empty.
    fn last_index(&self) -> LogIndex;

    /// Returns the term of the last entry in the log, or [`TermId::default()`](crate::message::TermId::default) if
    /// empty.
    fn last_term(&self) -> TermId;

    /// Returns the index immediately before the index of the first undiscarded entry in the log (see ["Log
    /// Truncation"](RaftLog#log-truncation)).
    fn prev_index(&self) -> LogIndex;

    /// Returns the term of the entry immediately preceding the first undiscarded entry in the log (see ["Log
    /// Truncation"](RaftLog#log-truncation)).
    fn prev_term(&self) -> TermId;

    /// Returns the next entry in the log not previously returned by this function, marking the returned entry eligible
    /// for future discard (see ["Log Truncation"](RaftLog#log-truncation)). Returns `None` if there is no such entry.
    fn take_next(&mut self) -> Option<LogEntry>;
}

pub(crate) struct RaftLogState<Log> {
    log: Log,
    pub commit_idx: LogIndex,
}

/// An iterator yielding committed [log entries][`LogEntry`].
///
/// A given [`LogEntry`] will be yielded only once over the lifetime of a Raft node.
///
/// [`LogEntry`]: crate::message::LogEntry
pub struct CommittedIter<'a, Log> {
    log: &'a mut RaftLogState<Log>,
}

//
// RaftLogState
//

impl<Log: RaftLog> RaftLogState<Log> {
    pub fn new(log: Log) -> Self {
        Self {
            log,
            commit_idx: LogIndex::default(),
        }
    }

    pub fn append(&mut self, entry: LogEntry) -> Result<(), Log::Error> {
        self.log.append(entry)
    }

    pub fn cancel_from(&mut self, from_index: LogIndex) -> Result<usize, Log::Error> {
        self.log.cancel_from(from_index)
    }

    pub fn entry_len(&self, entry: &LogEntry) -> usize {
        self.log.entry_len(entry)
    }

    pub fn get(&mut self, index: LogIndex) -> Option<LogEntry> {
        if index == LogIndex::default() {
            None
        } else {
            self.log.get(index)
        }
    }

    pub fn get_term(&mut self, index: LogIndex) -> Option<TermId> {
        if index == self.prev_index() {
            Some(self.prev_term())
        } else if index == LogIndex::default() {
            None
        } else {
            self.log.get_term(index)
        }
    }

    pub fn get_len(&mut self, index: LogIndex) -> Option<usize> {
        self.log.get_len(index)
    }

    pub fn last_index(&self) -> LogIndex {
        self.log.last_index()
    }

    pub fn last_term(&self) -> TermId {
        self.log.last_term()
    }

    pub fn log(&self) -> &Log {
        &self.log
    }

    pub fn log_mut(&mut self) -> &mut Log {
        &mut self.log
    }

    pub fn prev_index(&self) -> LogIndex {
        self.log.prev_index()
    }

    pub fn prev_term(&self) -> TermId {
        self.log.prev_term()
    }

    pub fn take_committed(&mut self) -> CommittedIter<'_, Log> {
        CommittedIter { log: self }
    }
}

//
// CommittedIter impls
//

impl<Log: RaftLog> Iterator for CommittedIter<'_, Log> {
    type Item = LogEntry;
    fn next(&mut self) -> Option<Self::Item> {
        if self.log.log.last_taken_index() < self.log.commit_idx {
            self.log.log.take_next()
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.log.commit_idx.id - self.log.log.last_taken_index().id) as usize;
        (remaining, Some(remaining))
    }
}

impl<Log: RaftLog> ExactSizeIterator for CommittedIter<'_, Log> {}

impl<Log: RaftLog> iter::FusedIterator for CommittedIter<'_, Log> {}
