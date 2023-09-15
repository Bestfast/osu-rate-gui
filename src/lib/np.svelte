<script lang="ts">
  import { listen } from '@tauri-apps/api/event'
    import { onMount } from 'svelte';
  // Import Swiper Svelte components
  
  let metadata;
  let np;
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

  const unlisten = listen("poll", async (event) => {
    metadata = event.payload;
    console.log("a");
  })

  
  $: {
    if (metadata) {
      np = `${metadata.artist} - ${metadata.title} [${metadata.difficulty}] by ${metadata.mapper}`;
    }
  }
  let rate = 1.0; // Initial value
  // Add an event listener to the slider input
  function updateSliderValue(event) {
    rate = event.target.value;
  }

  function handleInput(event) {
    const inputValue = parseFloat(event.target.value);

    // Check if the input value is greater than 0.1
    if (inputValue > 0.1) {
      rate = inputValue;
    } else {
      // If the input value is not valid, reset it to 1.0
      rate = 1.0;
    }
  }
  
</script>

<div>
  <p>{np}</p>
  <input type="range" min="0.1" max="3.0" step="0.01" bind:value={rate}>
  <p>Rate: <input type="number" min="0.1" step="0.1" inputmode="numeric" bind:value={rate} on:input={handleInput}></p>
</div>

