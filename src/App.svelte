<script type="ts">
  import BottomBar from './lib/components/BottomBar.svelte';
  import Terminal from './lib/components/Terminal.svelte'
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/tauri'

  onMount(async () => {
    await invoke('update_usage_counter')
  })
</script>

<main>
    <Terminal/>
    <BottomBar/>
</main>

<style type="scss">
  :root {
    font-family: monospace;
    background-color: black;
    overflow: hidden;
  }

  main {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    margin: auto;
    overflow: hidden;
  }
</style>