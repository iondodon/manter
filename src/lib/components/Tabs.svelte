<script lang="ts">
  import {ActiveTermUUIDStore, TerminalsStore} from '../stores/stores'
  import {NIL as NIL_UUID, v4 as uuidv4} from 'uuid'

  let terminals = []
  let activeTermUUID = NIL_UUID

  TerminalsStore.subscribe(updatedTerminals => terminals = updatedTerminals)
  ActiveTermUUIDStore.subscribe(updatedActiveTerminalUUID => activeTermUUID = updatedActiveTerminalUUID)

  const addNewTerminal = () => {
    const NEW_TERM_UUID = uuidv4()
    TerminalsStore.update(terminals => {
      terminals.push({
        uuid: NEW_TERM_UUID,
        sessionContext: {
          cwd: "~",
          user: ""
        },
        terminalInterface: null,
        ptyWebSocket: null,
        fitAddon: null
      })
      return terminals
    })

    setActive(NEW_TERM_UUID)
  }

  const setActive = (newActiveTermUUID) => ActiveTermUUIDStore.update(_prevActiveTermUUID => newActiveTermUUID)

  const getTerminalByUuid = (termUuid) => {
    for (const term of terminals) {
      if (term['uuid'] == termUuid) {
        return term
      }
    }
    throw `Terminal with UUID=${termUuid} not found`
  }

  const closeTerminal = async (termUUID) => {
    const isSure = await confirm("Are you sure you want to close this terminal?")
    if(!isSure) {
      return
    } 
    let termToClose = getTerminalByUuid(termUUID)
    termToClose['ptyWebSocket'].close()
    TerminalsStore.update(terminals => terminals.filter(term => term['uuid'] != termUUID))
    ActiveTermUUIDStore.update(_prevActiveTermUUID => terminals[0]['uuid'])
  }
</script>

<ol id="tabs">
  {#each terminals as terminal, index}
    <li class="tab" on:click={() => setActive(terminal['uuid'])}>
      {#if terminal['uuid'] === activeTermUUID}
        <div id="selected-tab">
          <span class="tab-text">Terminal {index}</span>
          <span class="close-tab-button" on:click={() => closeTerminal(terminal['uuid'])}>X</span>
        </div>
      {:else}
        <span class="tab-text">Terminal {index}</span>
        <span class="close-tab-button" on:click={() => closeTerminal(terminal['uuid'])}>X</span>
      {/if}
    </li>
  {/each}
  <li class="tab" on:click={addNewTerminal}><span>+</span></li>
</ol>

<style lang="scss">
  #tabs {
    display: flex;
    flex-direction: row;
    list-style-type: none;
    margin: 0;
    padding: 0;
    overflow: hidden;
    background-color: #333;
  }

  .close-tab-button {
    background-color: hsl(270, 54%, 32%);
    margin-left: 0.5em;
    margin-right: 0.5em;
  }

  .close-tab-button:hover {
    background-color: rgb(71, 71, 237);
  }

  #selected-tab {
    background-color: rebeccapurple;
  }

  .tab {
    float: left;
    outline: none;
    cursor: pointer;
    transition: 0.3s;
    font-size: 2em;
    border: 1px solid rgb(136, 16, 16);
    text-align: center;
  }

  .tab:hover {
    background-color: #ddd;
  }
</style>