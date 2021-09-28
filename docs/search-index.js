var searchIndex = JSON.parse('{\
"simple_raft":{"doc":"Raft consensus algorithm implementation.","t":[0,0,0,0,3,3,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,12,11,12,11,11,5,11,11,11,12,11,11,11,11,11,11,11,11,11,11,3,16,8,10,11,11,10,10,11,10,11,10,11,11,10,10,10,0,11,10,10,11,10,11,11,11,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,13,3,13,13,3,3,3,4,4,3,3,13,3,13,3,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,12,12,12,11,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,4,13,3,13,3,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12],"n":["core","log","message","node","RaftState","ReplicationState","append_entries","borrow","borrow","borrow_mut","borrow_mut","client_request","commit_idx","config","from","from","inflight","into","into","is_leader","leader","log","log_mut","match_idx","new","next_idx","node_id","peers","quorum_size","receive","replication_state","reset_peer","send_probe","set_config","take_committed","timeout","timer_tick","try_from","try_from","try_into","try_into","type_id","type_id","CommittedIter","Error","RaftLog","append","borrow","borrow_mut","cancel_from","entry_len","from","get","get_len","get_term","into","into_iter","last_index","last_taken_index","last_term","mem","next","prev_index","prev_term","size_hint","take_next","try_from","try_into","type_id","RaftLogMemory","append","borrow","borrow_mut","cancel_from","entry_len","from","get","get_term","into","last_index","last_taken_index","last_term","new_unbounded","prev_index","prev_term","take_next","try_from","try_into","type_id","with_capacity","AppendRequest","AppendRequest","AppendResponse","AppendResponse","Broadcast","LogEntry","LogIndex","RaftMessage","RaftMessageDestination","Rpc","SendableRaftMessage","TermId","To","VoteRequest","VoteRequest","VoteResponse","VoteResponse","add","add_assign","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","checked_sub","clear","clear","clear","clear","clear","clear","clear","clear","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","cmp","cmp","data","default","default","default","default","default","default","default","default","dest","encode","encode_raw","encode_raw","encode_raw","encode_raw","encode_raw","encode_raw","encode_raw","encode_raw","encoded_len","encoded_len","encoded_len","encoded_len","encoded_len","encoded_len","encoded_len","encoded_len","encoded_len","entries","eq","eq","eq","eq","eq","eq","eq","eq","eq","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from","id","id","into","into","into","into","into","into","into","into","into","into","into","last_log_idx","last_log_idx","last_log_term","leader_commit","match_idx","merge","merge_field","merge_field","merge_field","merge_field","merge_field","merge_field","merge_field","merge_field","message","ne","ne","ne","ne","ne","ne","ne","ne","ne","partial_cmp","partial_cmp","prev_log_idx","prev_log_term","rpc","sub","success","term","term","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_string","to_string","to_string","to_string","to_string","to_string","to_string","to_string","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","vote_granted","AppendError","Cancelled","RaftConfig","RaftLogErr","RaftNode","append","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone_into","config","election_timeout_ticks","eq","from","from","from","heartbeat_interval_ticks","into","into","into","is_leader","last_committed_log_index","leader","log","log_mut","ne","new","node_id","peers","receive","replication_chunk_size","replication_state","state","state_mut","take_committed","timer_tick","to_owned","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","data"],"q":["simple_raft","","","","simple_raft::core","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","simple_raft::log","","","","","","","","","","","","","","","","","","","","","","","","","","simple_raft::log::mem","","","","","","","","","","","","","","","","","","","","","simple_raft::message","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","simple_raft::node","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","simple_raft::node::AppendError"],"d":["Unstable, low-level API for the complete state of a Raft …","Types related to Raft log storage.","Raft message types for sending between nodes.","Higher-level API for a Raft node.","The complete state of a Raft node.","The state of Raft log replication from a Raft node to one …","","","","","","","","","","","The index of the last log entry sent to this peer but …","","","","","","","The index of the last log entry on this peer to up which …","","The index of the next log entry to be sent to this peer.","","","Computes the minimum size of a quorum of nodes in a Raft …","","","","Whether this node is currently probing to discover the …","","","","","","","","","","","An iterator yielding committed log entries.","The type of error returned by fallable operations.","An interface for storage of the Raft log of a <code>RaftNode</code>.","Appends an entry to the end of the log.","","","Cancels all entries including and after the entry at …","Returns the approximate serialized length in bytes of a …","","Returns the entry at a given index, or <code>None</code> if the index …","Returns the approximate serialized length of the entry at …","Returns the term of the entry at a given index, or <code>None</code> …","","","Returns the index of the last entry in the log, or …","Returns the index of the last entry which has been …","Returns the term of the last entry in the log, or …","A naive in-memory implementation of <code>RaftLog</code>, primarily …","","Returns the index immediately before the index of the …","Returns the term of the entry immediately preceding the …","","Returns the next entry in the log not previously returned …","","","","A naive in-memory implementation of <code>RaftLog</code>, primarily …","","","","","","","","","","","","","Constructs an empty Raft log with unbounded capacity.","","","","","","","Constructs an empty Raft log with bounded capacity.","A request to append entries to a Raft node’s log.","A request to append entries to a Raft node’s log.","The response to an <code>AppendRequest</code> allowing or denying an …","A response to an <code>AppendRequest</code> allowing or denying an …","The associated message should be sent to all known peers.","An entry in a Raft log.","A 1-based index into a Raft log.","A message sent between Raft nodes.","The destination for a <code>SendableRaftMessage</code>.","A Remote Procedure Call message to a Raft node.","A <code>RaftMessage</code> to be sent to a destination.","The unique, monotonically-increasing ID for a term of …","The associated message should be sent to one particular …","A request to obtain leadership amongst Raft nodes.","A request to obtain leadership amongst Raft nodes.","The response to a <code>VoteRequest</code> granting or denying …","A response to a <code>VoteRequest</code> granting or denying …","","","","","","","","","","","","","","","","","","","","","","","","","Subtraction with a non-negative integer, checking for …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Arbitrary data associated with the log entry.","","","","","","","","","The destination for the message.","","","","","","","","","","","","","","","","","","","A list of consecutive Raft log entries to append.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","The non-negative integer assigned to this term.","The integer representing this log index.","","","","","","","","","","","","The Raft log index of the last log entry stored by this …","The Raft log index of the last log entry in the responder…","The Raft leadership term of the last log entry stored by …","The Raft log index of the last log entry known by the …","The Raft log index of the last log entry up to which the …","","","","","","","","","","The message to be sent.","","","","","","","","","","","","The Raft log index immediately before the index of the …","The Raft leadership term of the log entry immediately …","The Remote Procedure Call contained by this message.","","Whether the <code>AppendRequest</code> was granted or not.","The greatest Raft leadership term ID seen by the sender.","The term of leadership of the node which appended this …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Whether the <code>VoteRequest</code> was granted or not.","An error returned while attempting to append to a Raft …","The append to the Raft log was cancelled and should be …","Configurable parameters of a Raft node.","An error was returned by the <code>RaftLog</code> implementation.","A Raft node, used for replicating a strongly-consistent …","Request appending an entry with arbitrary <code>data</code> to the …","","","","","","","","","Returns this node’s configurable parameters.","The minimum number of timer ticks between leadership …","","","","","The number of timer ticks between sending heartbeats to …","","","","Returns whether this node is the leader of the latest …","Returns the index of the last <code>LogEntry</code> which has been …","Returns the ID of the leader, if there is one, of the …","Returns a reference to the Raft log storage.","Returns a mutable reference to the Raft log storage.","","Constructs a new Raft node with specified peers and …","Returns this node’s ID.","Returns the IDs of this node’s peers.","Processes receipt of a <code>message</code> from a peer with ID <code>from</code>, …","The maximum number of bytes to replicate to a peer at a …","Returns the replication state corresponding to the peer …","Returns a reference to the low-level state of the Raft …","Returns a mutable reference to the low-level state of the …","Returns an iterator yielding committed log entries. A …","Ticks forward this node’s internal clock by one tick, …","","","","","","","","","","","Arbitrary data associated with the log entry."],"i":[0,0,0,0,0,0,1,2,1,2,1,1,1,1,2,1,2,2,1,1,1,1,1,2,1,2,1,1,0,1,1,1,2,1,1,1,1,2,1,2,1,2,1,0,3,0,3,4,4,3,3,4,3,3,3,4,4,3,3,3,0,4,3,3,4,3,4,4,4,0,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,0,6,0,6,7,0,0,0,0,0,0,0,7,0,6,0,6,8,9,10,7,11,6,12,13,14,15,16,9,8,10,7,11,6,12,13,14,15,16,9,8,8,11,12,13,14,15,16,9,8,11,6,12,13,14,15,16,9,8,11,6,12,13,14,15,16,9,8,9,8,16,11,12,13,14,15,16,9,8,10,6,11,12,13,14,15,16,9,8,11,6,12,13,14,15,16,9,8,14,11,6,12,13,14,15,16,9,8,11,11,6,6,12,12,13,13,14,14,15,15,16,9,9,8,8,10,7,11,6,12,13,14,15,16,9,8,9,8,10,7,11,6,12,13,14,15,16,9,8,12,15,12,14,15,6,11,12,13,14,15,16,9,8,10,11,6,12,13,14,15,16,9,8,9,8,14,14,11,8,15,11,16,11,6,12,13,14,15,16,9,8,11,6,12,13,14,15,9,8,10,7,11,6,12,13,14,15,16,9,8,10,7,11,6,12,13,14,15,16,9,8,10,7,11,6,12,13,14,15,16,9,8,13,0,17,0,17,0,18,18,17,19,18,17,19,19,19,18,19,19,18,17,19,19,18,17,19,18,18,18,18,18,19,18,18,18,18,19,18,18,18,18,18,19,18,17,19,18,17,19,18,17,19,20],"f":[null,null,null,null,null,null,[[],[["option",4,["sendableraftmessage"]],["sendableraftmessage",3]]],[[]],[[]],[[]],[[]],[[["bytes",3]],[["result",4,["appenderror"]],["appenderror",4]]],[[],["logindex",3]],[[],["raftconfig",3]],[[]],[[]],null,[[]],[[]],[[],["bool",15]],[[]],[[]],[[]],null,[[["btreeset",3],["raftconfig",3]]],null,[[]],[[],["btreeset",3]],[[["usize",15]],["usize",15]],[[["raftmessage",3]],[["option",4,["sendableraftmessage"]],["sendableraftmessage",3]]],[[],[["replicationstate",3],["option",4,["replicationstate"]]]],[[],[["option",4,["sendableraftmessage"]],["sendableraftmessage",3]]],null,[[["raftconfig",3]]],[[],["committediter",3]],[[],[["option",4,["sendableraftmessage"]],["sendableraftmessage",3]]],[[],[["option",4,["sendableraftmessage"]],["sendableraftmessage",3]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,[[["logentry",3]],["result",4]],[[]],[[]],[[["logindex",3]],[["usize",15],["result",4,["usize"]]]],[[["logentry",3]],["usize",15]],[[]],[[["logindex",3]],[["option",4,["logentry"]],["logentry",3]]],[[["logindex",3]],[["usize",15],["option",4,["usize"]]]],[[["logindex",3]],[["termid",3],["option",4,["termid"]]]],[[]],[[]],[[],["logindex",3]],[[],["logindex",3]],[[],["termid",3]],null,[[],["option",4]],[[],["logindex",3]],[[],["termid",3]],[[]],[[],[["option",4,["logentry"]],["logentry",3]]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,[[["logentry",3]],["result",4]],[[]],[[]],[[["logindex",3]],[["usize",15],["result",4,["usize"]]]],[[["logentry",3]],["usize",15]],[[]],[[["logindex",3]],[["option",4,["logentry"]],["logentry",3]]],[[["logindex",3]],[["termid",3],["option",4,["termid"]]]],[[]],[[],["logindex",3]],[[],["logindex",3]],[[],["termid",3]],[[]],[[],["logindex",3]],[[],["termid",3]],[[],[["option",4,["logentry"]],["logentry",3]]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[["usize",15]]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["u64",15]]],[[["u64",15]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["u64",15]],["option",4]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["raftmessage",3]],[[],["rpc",4]],[[],["voterequest",3]],[[],["voteresponse",3]],[[],["appendrequest",3]],[[],["appendresponse",3]],[[],["logentry",3]],[[],["termid",3]],[[],["logindex",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["ordering",4]],[[],["ordering",4]],null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],null,[[["raftmessage",3]],["bool",15]],[[["rpc",4]],["bool",15]],[[["voterequest",3]],["bool",15]],[[["voteresponse",3]],["bool",15]],[[["appendrequest",3]],["bool",15]],[[["appendresponse",3]],["bool",15]],[[["logentry",3]],["bool",15]],[[["termid",3]],["bool",15]],[[["logindex",3]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,null,null,null,null,[[["wiretype",4],["option",4],["u32",15],["decodecontext",3]],[["result",4,["decodeerror"]],["decodeerror",3]]],[[["wiretype",4],["u32",15],["decodecontext",3]],[["result",4,["decodeerror"]],["decodeerror",3]]],[[["wiretype",4],["u32",15],["decodecontext",3]],[["result",4,["decodeerror"]],["decodeerror",3]]],[[["wiretype",4],["u32",15],["decodecontext",3]],[["result",4,["decodeerror"]],["decodeerror",3]]],[[["wiretype",4],["u32",15],["decodecontext",3]],[["result",4,["decodeerror"]],["decodeerror",3]]],[[["wiretype",4],["u32",15],["decodecontext",3]],[["result",4,["decodeerror"]],["decodeerror",3]]],[[["wiretype",4],["u32",15],["decodecontext",3]],[["result",4,["decodeerror"]],["decodeerror",3]]],[[["wiretype",4],["u32",15],["decodecontext",3]],[["result",4,["decodeerror"]],["decodeerror",3]]],[[["wiretype",4],["u32",15],["decodecontext",3]],[["result",4,["decodeerror"]],["decodeerror",3]]],null,[[["raftmessage",3]],["bool",15]],[[["rpc",4]],["bool",15]],[[["voterequest",3]],["bool",15]],[[["voteresponse",3]],["bool",15]],[[["appendrequest",3]],["bool",15]],[[["appendresponse",3]],["bool",15]],[[["logentry",3]],["bool",15]],[[["termid",3]],["bool",15]],[[["logindex",3]],["bool",15]],[[],[["ordering",4],["option",4,["ordering"]]]],[[],[["ordering",4],["option",4,["ordering"]]]],null,null,null,[[["u64",15]]],null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,[[["into",8,["bytes"]],["bytes",3]],[["result",4,["appenderror"]],["appenderror",4]]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["raftconfig",3]],[[]],[[],["raftconfig",3]],null,[[["raftconfig",3]],["bool",15]],[[]],[[]],[[]],null,[[]],[[]],[[]],[[],["bool",15]],[[],["logindex",3]],[[]],[[]],[[]],[[["raftconfig",3]],["bool",15]],[[["btreeset",3],["raftconfig",3]]],[[]],[[],["btreeset",3]],[[["raftmessage",3]]],null,[[],[["replicationstate",3],["option",4,["replicationstate"]]]],[[],["raftstate",3]],[[],["raftstate",3]],[[],["committediter",3]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null],"p":[[3,"RaftState"],[3,"ReplicationState"],[8,"RaftLog"],[3,"CommittedIter"],[3,"RaftLogMemory"],[4,"Rpc"],[4,"RaftMessageDestination"],[3,"LogIndex"],[3,"TermId"],[3,"SendableRaftMessage"],[3,"RaftMessage"],[3,"VoteRequest"],[3,"VoteResponse"],[3,"AppendRequest"],[3,"AppendResponse"],[3,"LogEntry"],[4,"AppendError"],[3,"RaftNode"],[3,"RaftConfig"],[13,"Cancelled"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};