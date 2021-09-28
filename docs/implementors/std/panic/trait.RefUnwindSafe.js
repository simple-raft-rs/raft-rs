(function() {var implementors = {};
implementors["simple_raft"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"simple_raft/core/struct.ReplicationState.html\" title=\"struct simple_raft::core::ReplicationState\">ReplicationState</a>","synthetic":true,"types":["simple_raft::core::ReplicationState"]},{"text":"impl&lt;Log, Random, NodeId&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"simple_raft/core/struct.RaftState.html\" title=\"struct simple_raft::core::RaftState\">RaftState</a>&lt;Log, Random, NodeId&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Log: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;NodeId: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Random: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a>,&nbsp;</span>","synthetic":true,"types":["simple_raft::core::RaftState"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"simple_raft/log/mem/struct.RaftLogMemory.html\" title=\"struct simple_raft::log::mem::RaftLogMemory\">RaftLogMemory</a>","synthetic":true,"types":["simple_raft::log::mem::RaftLogMemory"]},{"text":"impl&lt;'a, Log&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"simple_raft/log/struct.CommittedIter.html\" title=\"struct simple_raft::log::CommittedIter\">CommittedIter</a>&lt;'a, Log&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Log: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a>,&nbsp;</span>","synthetic":true,"types":["simple_raft::log::CommittedIter"]},{"text":"impl&lt;NodeId&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"simple_raft/message/struct.SendableRaftMessage.html\" title=\"struct simple_raft::message::SendableRaftMessage\">SendableRaftMessage</a>&lt;NodeId&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;NodeId: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a>,&nbsp;</span>","synthetic":true,"types":["simple_raft::message::SendableRaftMessage"]},{"text":"impl&lt;NodeId&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"enum\" href=\"simple_raft/message/enum.RaftMessageDestination.html\" title=\"enum simple_raft::message::RaftMessageDestination\">RaftMessageDestination</a>&lt;NodeId&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;NodeId: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a>,&nbsp;</span>","synthetic":true,"types":["simple_raft::message::RaftMessageDestination"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"simple_raft/message/struct.RaftMessage.html\" title=\"struct simple_raft::message::RaftMessage\">RaftMessage</a>","synthetic":true,"types":["simple_raft::message::RaftMessage"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"enum\" href=\"simple_raft/message/enum.Rpc.html\" title=\"enum simple_raft::message::Rpc\">Rpc</a>","synthetic":true,"types":["simple_raft::message::Rpc"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"simple_raft/message/struct.VoteRequest.html\" title=\"struct simple_raft::message::VoteRequest\">VoteRequest</a>","synthetic":true,"types":["simple_raft::message::VoteRequest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"simple_raft/message/struct.VoteResponse.html\" title=\"struct simple_raft::message::VoteResponse\">VoteResponse</a>","synthetic":true,"types":["simple_raft::message::VoteResponse"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"simple_raft/message/struct.AppendRequest.html\" title=\"struct simple_raft::message::AppendRequest\">AppendRequest</a>","synthetic":true,"types":["simple_raft::message::AppendRequest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"simple_raft/message/struct.AppendResponse.html\" title=\"struct simple_raft::message::AppendResponse\">AppendResponse</a>","synthetic":true,"types":["simple_raft::message::AppendResponse"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"simple_raft/message/struct.LogEntry.html\" title=\"struct simple_raft::message::LogEntry\">LogEntry</a>","synthetic":true,"types":["simple_raft::message::LogEntry"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"simple_raft/message/struct.TermId.html\" title=\"struct simple_raft::message::TermId\">TermId</a>","synthetic":true,"types":["simple_raft::message::TermId"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"simple_raft/message/struct.LogIndex.html\" title=\"struct simple_raft::message::LogIndex\">LogIndex</a>","synthetic":true,"types":["simple_raft::message::LogIndex"]},{"text":"impl&lt;Log, Random, NodeId&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"simple_raft/node/struct.RaftNode.html\" title=\"struct simple_raft::node::RaftNode\">RaftNode</a>&lt;Log, Random, NodeId&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Log: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;NodeId: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Random: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a>,&nbsp;</span>","synthetic":true,"types":["simple_raft::node::RaftNode"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"simple_raft/node/struct.RaftConfig.html\" title=\"struct simple_raft::node::RaftConfig\">RaftConfig</a>","synthetic":true,"types":["simple_raft::node::RaftConfig"]},{"text":"impl&lt;E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"enum\" href=\"simple_raft/node/enum.AppendError.html\" title=\"enum simple_raft::node::AppendError\">AppendError</a>&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.55.0/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a>,&nbsp;</span>","synthetic":true,"types":["simple_raft::node::AppendError"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()