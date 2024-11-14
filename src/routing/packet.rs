
use crate::routing::types::NodeId;

pub struct Packet {
    pub pack_type: PacketType,
    pub routing_header: SourceRoutingHeader,
    pub session_id: u64,
}

pub enum PacketType {
    MsgFragment(Fragment),
    Nack(Nack),
    Ack(Ack),
    Query(Query),
    QueryResult(QueryResult),
}

#[derive(Clone, Copy)]
pub enum NodeType{Client, Drone, Server}

#[derive(Clone)]
pub struct Query {
	/// Unique identifier of the flood, to prevent loops.
	pub flood_id: u64,
	/// ID of client or server
	pub initiator_id: NodeId,
	/// Time To Live, decremented at each hop to limit the query's lifespan.
	/// When ttl reaches 0, we start a QueryResult message that reaches back to the initiator
	pub ttl: u8,
	/// Records the nodes that have been traversed (to track the connections).
	pub path_trace: Vec<(NodeId, NodeType)>,
	/// Broadcasting query, this means that no QueryResult needs to be sent back
	pub broadcasting: bool
}

#[derive(Clone)]
pub struct QueryResult {
	/// Unique indentifier of the flood, this allows the initiator to identify the information obtained by the latest flood only
	pub flood_id: u64,
	/// Record of the nodes traversed by the flooding query
	pub path_trace: Vec<(NodeId, NodeType)>
}

pub struct Nack {
    fragment_index: u64,
    time_of_fail: std::time::Instant,
    nack_type: NackType,
}

pub enum NackType {
    ErrorInRouting(NodeId), // contains id of not neighbor
    Dropped(),
}

pub struct Ack {
    fragment_index: u64,
    time_received: std::time::Instant,
}

impl Ack {
    pub fn new(fragment_index: u64) -> Ack {
        Ack {
            fragment_index,
            time_received: std::time::Instant::now(),
        }
    }
}

pub struct Fragment {
    fragment_index: u64,
    total_n_fragments: u64,
    length: u8,
    data: [u8; 80],
}

#[derive(Debug)]
pub struct SourceRoutingHeader {
    pub hops: Vec<NodeId>,
}

