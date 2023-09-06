<script>
import { emit, listen } from '@tauri-apps/api/event'
import { onMount } from 'svelte';
  let now_playing = "";

    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    
    const unlisten = listen("np", async (event) => {
      now_playing = event.payload
    })
</script>

<div>
  <p>{now_playing}</p>
</div>

