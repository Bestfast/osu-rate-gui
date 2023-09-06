use gosumemory_helper::Gosumemory; 
use tungstenite::{connect, Message};
use std::time::Duration;
use tokio::spawn;
use tauri::Window;
use tokio::sync::RwLock;
use std::sync::Arc;
use std::default::Default;

#[derive(Default, Clone)]
pub struct Server {
    data: Arc<RwLock<Gosumemory>>,
}

impl Server {
    pub async fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (mut socket, _response) = connect("ws://localhost:24050/ws").expect("Can't connect");
        
        spawn({
            let data = Arc::clone(&self.data);
            async move {
                loop {
                    if let Ok(msg) = socket.read() {
                        let mem: Gosumemory = parse(msg);
                        
                        *data.write().await = mem;
                    }
                    tokio::time::sleep(Duration::from_millis(20)).await;
                }
            }
        });
        Ok(())
    }
   
    // Random example of doing something with shared data 
    pub async fn get_data(&self) -> Gosumemory {
        let mem: &Gosumemory = &*self.data.read().await;
        return mem.clone();
    }
}

fn parse(msg: Message) -> Gosumemory {
    let json_result: Gosumemory = serde_json::from_str(msg.to_text().unwrap()).unwrap();
    return json_result;
}