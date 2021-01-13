/*
 * Copyright (C) 2019 Open Whisper Systems
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

use crate::prelude::*;

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "prost", derive(prost::Message))]
#[cfg_attr(not(feature = "prost"), derive(Debug, Default))]
pub struct RaftMessage {
    #[cfg_attr(feature = "prost", prost(message, required, tag="2"))]
    pub term: TermId,
    #[cfg_attr(feature = "prost", prost(oneof="Rpc", tags="3, 4, 5, 6"))]
    pub rpc: Option<Rpc>,
}

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "prost", derive(prost::Oneof))]
#[cfg_attr(not(feature = "prost"), derive(Debug))]
pub enum Rpc {
    #[cfg_attr(feature = "prost", prost(message, tag="3"))]
    VoteRequest(VoteRequest),
    #[cfg_attr(feature = "prost", prost(message, tag="4"))]
    VoteResponse(VoteResponse),
    #[cfg_attr(feature = "prost", prost(message, tag="5"))]
    AppendRequest(AppendRequest),
    #[cfg_attr(feature = "prost", prost(message, tag="6"))]
    AppendResponse(AppendResponse),
}

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "prost", derive(prost::Message))]
#[cfg_attr(not(feature = "prost"), derive(Debug, Default))]
pub struct VoteRequest {
    #[cfg_attr(feature = "prost", prost(message, required, tag="2"))]
    pub last_log_idx: LogIdx,
    #[cfg_attr(feature = "prost", prost(message, required, tag="3"))]
    pub last_log_term: TermId,
}

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "prost", derive(prost::Message))]
#[cfg_attr(not(feature = "prost"), derive(Debug, Default))]
pub struct VoteResponse {
    #[cfg_attr(feature = "prost", prost(bool, required, tag="2"))]
    pub vote_granted: bool,
}

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "prost", derive(prost::Message))]
#[cfg_attr(not(feature = "prost"), derive(Debug, Default))]
pub struct AppendRequest {
    #[cfg_attr(feature = "prost", prost(message, required, tag="1"))]
    pub prev_log_idx: LogIdx,
    #[cfg_attr(feature = "prost", prost(message, required, tag="2"))]
    pub prev_log_term: TermId,
    #[cfg_attr(feature = "prost", prost(message, required, tag="3"))]
    pub leader_commit: LogIdx,
    #[cfg_attr(feature = "prost", prost(message, repeated, tag="4"))]
    pub entries: Vec<LogEntry>,
}

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "prost", derive(prost::Message))]
#[cfg_attr(not(feature = "prost"), derive(Debug, Default))]
pub struct AppendResponse {
    #[cfg_attr(feature = "prost", prost(bool, required, tag="1"))]
    pub success: bool,
    #[cfg_attr(feature = "prost", prost(message, required, tag="2"))]
    pub match_idx: LogIdx,
    #[cfg_attr(feature = "prost", prost(message, required, tag="3"))]
    pub last_log_idx: LogIdx,
}

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "prost", derive(prost::Message))]
#[cfg_attr(not(feature = "prost"), derive(Debug, Default))]
pub struct LogEntry {
    #[cfg_attr(feature = "prost", prost(message, required, tag="1"))]
    pub term: TermId,
    #[cfg_attr(feature = "prost", prost(bytes="vec", required, tag="2"))]
    pub data: Vec<u8>,
}

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "prost", derive(prost::Message))]
#[cfg_attr(not(feature = "prost"), derive(Debug, Default))]
pub struct TermId {
    #[cfg_attr(feature = "prost", prost(uint64, required, tag="1"))]
    pub id: u64,
}

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "prost", derive(prost::Message))]
#[cfg_attr(not(feature = "prost"), derive(Debug, Default))]
pub struct LogIdx {
    #[cfg_attr(feature = "prost", prost(uint64, required, tag="1"))]
    pub id: u64,
}
