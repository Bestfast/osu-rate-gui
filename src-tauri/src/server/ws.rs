use gosumemory_helper::Gosumemory;
use std::default::Default;
use std::sync::Arc;
use std::time::Duration;
use tokio::spawn;
use tokio::sync::RwLock;
use tokio::time::sleep;
use tungstenite::{connect, Message};
use spdlog::prelude::*;

const UPDATE_SLEEP: Duration = Duration::from_millis(100);

#[derive(Default, Clone)]
pub struct Server {
    data: Arc<RwLock<Gosumemory>>,
}

impl Server {
    // inits the Server struct by connecting to Gosumemory and polling through the socket messages.
    // If the Message is different from the previous message, it will update the Gosumemory struct.
    pub async fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (mut socket, _response) =
            connect("ws://localhost:24050/ws").expect("Can't connect to gosumemory");
        info!("Connected to gosumemory successfully");
        spawn({
            let data = Arc::clone(&self.data);
            async move {
                let mut tmp = Message::Text("".to_string());
                loop {
                    if let Ok(msg) = socket.read() {
                        if tmp != msg {
                            trace!("Received message from gosumemory that's different from the previous one, parsing it", );
                            let mem: Gosumemory = parse(&msg);
                            *data.write().await = mem;
                            tmp = msg;
                        }
                    }
                    sleep(UPDATE_SLEEP).await;
                }
            }
        });
        Ok(())
    }

    pub async fn get_struct(&self) -> Gosumemory {
        let mem: &Gosumemory = &*self.data.read().await;
        return mem.clone();
    }
}

fn parse(msg: &Message) -> Gosumemory {
    serde_json::from_str(msg.to_text().unwrap()).unwrap()
}
