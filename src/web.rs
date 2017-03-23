use std::io::{stdout, stdin, Write};
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use cancellation::{CancellationToken, CancellationTokenSource, OperationCanceled};
use std::sync::{Arc, Mutex};

use caches::file_cache::*;
use resources::*;
use resources::common::*;
use resources::index_resource::*;
use resources::show_resource::*;
use resources::image_resource::*;
use resources::web_resource::*;

pub struct Requester {}

impl Requester {
    pub fn new(rx_req: Receiver<ChannelItem>, tx_res: Sender<ChannelItem>) -> Self {

        // web client
        thread::spawn(move || {
            let mut wr = WebResource::new();
            let mut fc = Box::new(FileCache::new());
            let ct = CancellationTokenSource::new();
            ct.cancel_after(::std::time::Duration::new(10, 0));
            loop {
                match rx_req.recv() {
                    Ok(item) => {

                        let th = thread::current();
                        ct.run(|| { th.unpark(); }, || match item.extra.clone() {
                            ChannelItemType::Index(_) => {
                                let mut index_resource = IndexResource::new(&mut wr, &ct, &mut fc);
                                tx_res.send(index_resource.fetch(&item)).expect("[web client] fail to send index request");
                            }
                            ChannelItemType::Show(_) => {
                                let mut show_resource = ShowResource::new(&mut wr, &ct, &mut fc);
                                tx_res.send(show_resource.fetch(&item)).expect("[web client] fail to send show request");
                            }
                            ChannelItemType::Image(_) => {
                                let mut image_resource = ImageResource::new(&mut wr, &ct, &mut fc);
                                tx_res.send(image_resource.fetch(&item)).expect("[web client] fail to send image request");
                            }
                        });

                        if ct.is_canceled() {
                            thread::park_timeout(::std::time::Duration::new(0, 250));
                            // Err(OperationCanceled)
                        } else {
                            // Ok(())
                        }
                    }
                    Err(_) => {}
                }
            }
        });

        Requester { }
    }
}
