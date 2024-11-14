
use crate::routing::types::{NodeId, ServerType};


#[derive(Debug)]
pub struct Message {
    source_id: NodeId,
    session_id: u64,
    content: MessageContent,
}

#[derive(Debug)]
pub enum MessageContent {
    // Client -> Server
    ReqServerType,
    ReqFilesList,
    ReqFile(u64),
    ReqMedia(u64),

    ReqClientList,
    ReqMessageSend { to: NodeId, message: Vec<u8> },

    // Server -> Client
    RespServerType(ServerType),
    RespFilesList(Vec<u64>),
    RespFile(Vec<u8>),
    RespMedia(Vec<u8>),
    ErrUnsupporedRequestType,
    ErrRequestedNotFound,

    RespClientList(Vec<NodeId>),
    RespMessageFrom { from: NodeId, message: Vec<u8> },
    ErrWrongClientId,
}

impl Message {
    pub fn new(
        source_id: NodeId,
        session_id: u64,
        content: MessageContent,
    ) -> Self {
        Self {
            source_id,
            session_id,
            content,
        }
    }
}
