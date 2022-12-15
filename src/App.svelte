<script type="ts">
  import BottomBar from './lib/components/BottomBar.svelte'
  import Terminal from './lib/components/Terminal.svelte'
  import Tabs from './lib/components/Tabs.svelte'
  import {onMount} from 'svelte'
  import {ActiveTermUUIDStore, TerminalsStore} from './lib/stores/stores'
  import {NIL as NIL_UUID} from 'uuid'
  import { IS_MACINTOSH } from './lib/config/config'

  let terminals = []
  let activeTerminalUUID = NIL_UUID

  TerminalsStore.subscribe(updatedTerminals => terminals = updatedTerminals)
  ActiveTermUUIDStore.subscribe(updatedTerminalUUID => activeTerminalUUID = updatedTerminalUUID)

  onMount(async () => {
    if (IS_MACINTOSH) {
      alert("Run this two commands after the terminal is open: \n prmptcmd() { eval '$PROMPT_COMMAND' } \nprecmd_functions=(prmptcmd)")
    }
  })
</script>

<main>
  <Tabs/>
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
  <BottomBar/>
</main>

<style lang="scss">
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