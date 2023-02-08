<script type="ts">
  import BottomBar from './lib/components/BottomBar.svelte'
  import Terminal from './lib/components/Terminal.svelte'
  import Tabs from './lib/components/Tabs.svelte'
  import {onMount} from 'svelte'
  import {ActiveTermUUIDStore, TerminalsStore} from './lib/stores/stores'
  import {NIL as NIL_UUID} from 'uuid'
  import Home from './lib/components/Home.svelte';

  let terminals = []
  let activeTerminalUUID = NIL_UUID

  TerminalsStore.subscribe(updatedTerminals => terminals = updatedTerminals)
  ActiveTermUUIDStore.subscribe(updatedTerminalUUID => activeTerminalUUID = updatedTerminalUUID)

  onMount(async () => {

  })
</script>

<main>
  <Tabs/>

  {#if terminals.length === 0}
    <Home/>
  {:else}
    {#each terminals as terminal}
      {#if terminal['uuid'] === activeTerminalUUID}
        <Terminal
          bind:sessionContext={terminal['sessionContext']}
          bind:terminalInterface={terminal['terminalInterface']}
          bind:ptyWebSocket={terminal['ptyWebSocket']}
          bind:fitAddon={terminal['fitAddon']}
        />
      {/if}
    {/each}
  {/if}

  <BottomBar/>
</main>

<style lang="scss">
  :root {
    font-family: monospace;
    background-color: black;
  }

  main {
    height: 100%;
    width: 100%;
  }
</style>