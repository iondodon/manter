<script type="ts">
  import BottomBar from './lib/components/BottomBar.svelte'
  import Terminal from './lib/components/Terminal.svelte'
  import Tabs from './lib/components/Tabs.svelte';
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/tauri'
  import { TerminalsStore } from './lib/stores/stores'
  import { ActiveTermIdStore } from './lib/stores/stores'

  let terminals = []
  let activeTerminalId = 0
  let activeTerminal = terminals[0]

  TerminalsStore.subscribe(updatedTerminals => terminals = updatedTerminals)
  ActiveTermIdStore.subscribe(updatedTerminalId => {
    activeTerminalId = updatedTerminalId
    activeTerminal = terminals[updatedTerminalId]
  })

  onMount(async () => {
    await invoke('update_usage_counter')
  })
</script>

<main>
  <Tabs/>
  {#each terminals as terminal}
    {#if terminal.id == activeTerminalId}
      <Terminal
        bind:sessionContext={activeTerminal['sessionContext']}
        bind:terminalInterface={activeTerminal['terminalInterface']} 
        bind:ptyWebSocket={activeTerminal['ptyWebSocket']}
        bind:fitAddon={activeTerminal['fitAddon']}
      />
    {/if}
  {/each}
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