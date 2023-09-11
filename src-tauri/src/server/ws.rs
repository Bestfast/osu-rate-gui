use futures::StreamExt;
use gosumemory_helper::{Gosumemory, Metadata};
use std::default::Default;
use std::sync::Arc;
use tokio::spawn;
use tokio::sync::{RwLock, watch};
use tokio::time::{sleep, Duration};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use spdlog::prelude::*;

const UPDATE_SLEEP: Duration = Duration::from_millis(100);


#[derive(Default, Clone)]
pub struct Server {
    data: Arc<RwLock<Gosumemory>>,
}
   
impl Server {
    // inits the Server struct by connecting to Gosumemory and polling through the socket messages.
    // If the Message is different from the previous message, it will update the Gosumemory struct.
    pub async fn init(&mut self, tx: watch::Sender<()>) -> Result<(), Box<dyn std::error::Error>> {
        let (ws_stream, _response) =
            connect_async("ws://localhost:24050/ws").await.expect("Can't connect to gosumemory");
        info!("Connected to gosumemory successfully");
        let (_, socket) = ws_stream.split();
        spawn({
            let data = Arc::clone(&self.data);
            let tmp: Arc<RwLock<Metadata>> = Arc::new(RwLock::new(Gosumemory::default().menu.bm.metadata));
            async move {
                let read_future = socket.for_each(|msg| async {
                    trace!("Received message from gosumemory");
                    if let Ok(msg) = msg {
                        let mem: Gosumemory = parse(&msg);
                        trace!("parsed the message");
                        if *tmp.read().await != mem.menu.bm.metadata {
                            info!("Received message from gosumemory that's different from the previous one, parsing it");
                            *tmp.write().await = mem.menu.bm.metadata.clone();
                            *data.write().await = mem;
                            let _ = tx.send(());
                        }
                    }
                });
                    read_future.await;
            
            }
        });
        Ok(())
    }

    pub async fn get_struct(&self) -> Gosumemory {
        trace!("Requesting Gosumemory struct");
        let mem: &Gosumemory = &*self.data.read().await;
        return mem.clone();
    }

}

fn parse(msg: &Message) -> Gosumemory {
    serde_json::from_str(msg.to_text().unwrap()).unwrap()
}
