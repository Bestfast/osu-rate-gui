use gosumemory_helper::Gosumemory;

pub async fn get_np(data: Gosumemory) -> String {
        data.menu.bm.metadata.title
    }