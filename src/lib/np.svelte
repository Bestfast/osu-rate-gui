

<script>
  import { listen } from '@tauri-apps/api/event'
  // Import Swiper Svelte components
  
  let metadata;
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

  const unlisten = listen("poll", async (event) => {
    metadata = event.payload;
  })
  
  $: {
    if (metadata) {
      document.getElementById("np").innerHTML = `${metadata.artist} - ${metadata.title} [${metadata.difficulty}] by ${metadata.mapper}`;
    }
  }
  
</script>

<div>
  <p id="np"></p>

</div>

