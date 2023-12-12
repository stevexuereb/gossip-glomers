use async_trait::async_trait;
use maelstrom::protocol::Message;
use maelstrom::{done, Node, Result, Runtime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub(crate) fn main() -> Result<()> {
    Runtime::init(try_main())
}

async fn try_main() -> Result<()> {
    let handler = Arc::new(Handler::default());
    Runtime::new().with_handler(handler).run().await
}

#[derive(Default)]
struct Handler {
    broadcast_msgs: Mutex<Vec<u64>>,
}

#[async_trait]
impl Node for Handler {
    async fn process(&self, runtime: Runtime, req: Message) -> Result<()> {
        let msg: Result<Request> = req.body.as_obj();
        match msg {
            Ok(Request::Read {}) => {
                let msg = Request::ReadOk {
                    messages: self.broadcast_msgs.lock().unwrap().to_vec(),
                };

                return runtime.reply(req, msg).await;
            }
            Ok(Request::Broadcast { message: element }) => {
                self.add_mesage(element);

                return runtime.reply_ok(req).await;
            }
            Ok(Request::Topology { .. }) => {
                return runtime.reply_ok(req).await;
            }
            _ => done(runtime, req),
        }
    }
}

impl Handler {
    fn add_mesage(&self, val: u64) {
        let mut g = self.broadcast_msgs.lock().unwrap();
        g.push(val)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
enum Request {
    Read {},
    ReadOk {
        messages: Vec<u64>,
    },
    Broadcast {
        message: u64,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
}
