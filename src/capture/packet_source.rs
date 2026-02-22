use std::time::Duration;

#[allow(dead_code)]
pub struct RawFrame {
    pub data: Vec<u8>,
    pub timestamp: Duration,
}

pub trait PacketSource {
    fn next_packet(&mut self) -> Option<RawFrame>;
}

pub struct NullPacketSource;

impl PacketSource for NullPacketSource {
    fn next_packet(&mut self) -> Option<RawFrame> {
        None
    }
}

#[cfg(test)]
pub mod test_helpers {
    use std::collections::VecDeque;

    use super::*;

    pub struct MockPacketSource {
        frames: VecDeque<RawFrame>,
    }

    impl MockPacketSource {
        pub fn empty() -> Self {
            Self {
                frames: VecDeque::new(),
            }
        }
    }

    impl PacketSource for MockPacketSource {
        fn next_packet(&mut self) -> Option<RawFrame> {
            self.frames.pop_front()
        }
    }
}
