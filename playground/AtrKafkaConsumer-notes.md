# Kafka Consumer Actor

connected to 
a single topic
a single partition
has a destination actor that can handle RawBinary messages that are Numbered
has a ring buffer of these messages
keeps track of its offset

```rust
    pub topic               : String,
    pub partition           : u32,
        destination         : Rc<WrappedActor<MsgNumberedMsg<MsgRawBinary>>>,
        consumer            : Rc<StreamConsumer>,
        msg_buffer          : Rc<RwLock<AllocRingBuffer<MsgNumberedMsg<MsgRawBinary>>>>,
        last_received_offset: Rc<RwLock<u64>>
```

