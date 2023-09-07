use gosumemory_helper::Gosumemory; 
use tungstenite::{connect, Message};
use std::time::Duration;
use tokio::spawn;
use tokio::time::sleep;
use tokio::sync::RwLock;
use std::sync::Arc;
use std::default::Default;

const UPDATE_SLEEP: Duration = Duration::from_millis(100);

#[derive(Default, Clone)]
pub struct Server {
    data: Arc<RwLock<Gosumemory>>,
}

impl Server {

    // inits the Server struct by connecting to Gosumemory and polling through the socket messages. 
    // If the Message is different from the previous message, it will update the Gosumemory struct.
    pub async fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (mut socket, _response) = connect("ws://localhost:24050/ws").expect("Can't connect to gosumemory");
        
        spawn({
            let data = Arc::clone(&self.data);
            async move {
                let mut tmp = Message::Text("".to_string());
                loop {
                    if let Ok(msg) = socket.read() {
                        if tmp != msg {
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