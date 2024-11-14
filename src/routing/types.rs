
pub type NodeId = u64;

#[derive(Debug)]
pub enum ServerType{
    ChatServer, // only does chat
    TextServer, // only does text
    MediaServer, // does text and media
}
