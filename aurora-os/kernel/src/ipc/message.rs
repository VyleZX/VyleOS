//! Message Passing IPC

use alloc::vec::Vec;
use spin::Mutex;
use super::IpcResult;

/// Maximum message size in bytes
const MAX_MESSAGE_SIZE: usize = 4096;

/// A message for IPC communication
#[derive(Debug, Clone)]
pub struct Message {
    /// Source port/handle
    pub from: u32,
    /// Destination port/handle
    pub to: u32,
    /// Message type identifier
    pub msg_type: u32,
    /// Message payload
    pub data: Vec<u8>,
}

impl Message {
    pub fn new(from: u32, to: u32, msg_type: u32, data: Vec<u8>) -> Self {
        Self { from, to, msg_type, data }
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            from: 0,
            to: 0,
            msg_type: 0,
            data: Vec::with_capacity(capacity.min(MAX_MESSAGE_SIZE)),
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

/// Message queue for a port
pub struct MessageQueue {
    /// Port number
    port: u32,
    /// Queued messages
    messages: Mutex<Vec<Message>>,
    /// Maximum queue size
    max_size: usize,
}

impl MessageQueue {
    pub fn new(port: u32, max_size: usize) -> Self {
        Self {
            port,
            messages: Mutex::new(Vec::new()),
            max_size,
        }
    }
    
    /// Send a message to this queue
    pub fn send(&self, msg: Message) -> IpcResult<()> {
        let mut messages = self.messages.lock();
        
        if messages.len() >= self.max_size {
            return Err(super::IpcError::QueueFull);
        }
        
        if msg.len() > MAX_MESSAGE_SIZE {
            return Err(super::IpcError::QueueFull);
        }
        
        messages.push(msg);
        Ok(())
    }
    
    /// Receive a message from this queue (blocking would be implemented with wait queue)
    pub fn receive(&self) -> IpcResult<Message> {
        let mut messages = self.messages.lock();
        
        if messages.is_empty() {
            return Err(super::IpcError::QueueEmpty);
        }
        
        Ok(messages.remove(0))
    }
    
    /// Try to receive without blocking
    pub fn try_receive(&self) -> IpcResult<Message> {
        self.receive()
    }
    
    /// Get the number of pending messages
    pub fn len(&self) -> usize {
        self.messages.lock().len()
    }
    
    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.messages.lock().is_empty()
    }
}

/// Global port registry
static PORT_REGISTRY: Mutex<Option<MessagePortRegistry>> = Mutex::new(None);

struct MessagePortRegistry {
    next_port: u32,
    ports: alloc::collections::BTreeMap<u32, Arc<MessageQueue>>,
}

impl MessagePortRegistry {
    fn new() -> Self {
        Self {
            next_port: 1,
            ports: alloc::collections::BTreeMap::new(),
        }
    }
    
    fn allocate_port(&mut self) -> u32 {
        let port = self.next_port;
        self.next_port += 1;
        port
    }
    
    fn create_queue(&mut self, port: u32, max_size: usize) -> Arc<MessageQueue> {
        let queue = Arc::new(MessageQueue::new(port, max_size));
        self.ports.insert(port, queue.clone());
        queue
    }
}

/// Create a new message port
pub fn create_port(max_size: usize) -> Arc<MessageQueue> {
    let mut registry = PORT_REGISTRY.lock();
    if registry.is_none() {
        *registry = Some(MessagePortRegistry::new());
    }
    
    let reg = registry.as_mut().unwrap();
    let port = reg.allocate_port();
    reg.create_queue(port, max_size)
}
