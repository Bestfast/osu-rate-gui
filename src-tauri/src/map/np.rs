use gosumemory_helper::Gosumemory;
use tauri::Window;
pub async fn get_np(data: Gosumemory) -> String {
        data.menu.bm.metadata.title
    }