<script type="ts">
  import BottomBar from './lib/components/BottomBar.svelte'
  import Terminal from './lib/components/Terminal.svelte'
  import Tabs from './lib/components/Tabs.svelte';
  import { onMount } from 'svelte'
  import { TerminalsStore } from './lib/stores/stores'
  import { ActiveTermUUIDStore } from './lib/stores/stores'
  import { NIL as NIL_UUID } from 'uuid'

  let terminals = []
  let activeTerminalUUID = NIL_UUID

  TerminalsStore.subscribe(updatedTerminals => terminals = updatedTerminals)
  ActiveTermUUIDStore.subscribe(updatedTerminalUUID => activeTerminalUUID = updatedTerminalUUID)

  onMount(async () => {
    
  })
</script>

<main>
  <Tabs/>
  {#each terminals as terminal}
    {#if terminal['uuid'] == activeTerminalUUID}
      <Terminal
        bind:sessionContext={terminal['sessionContext']}
        bind:terminalInterface={terminal['terminalInterface']} 
        bind:ptyWebSocket={terminal['ptyWebSocket']}
        bind:fitAddon={terminal['fitAddon']}
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