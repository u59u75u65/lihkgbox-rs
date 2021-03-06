use resources::*;
use resources::common::*;

pub struct DefaultResource {
}

impl DefaultResource {
    pub fn new() -> Self {
        DefaultResource {}
    }
}

impl Resource for DefaultResource {
    fn fetch(&mut self, item: &ChannelItem) -> ChannelItem {
        Default::default()
    }
}
