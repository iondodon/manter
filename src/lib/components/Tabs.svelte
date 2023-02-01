<script lang="ts">
  import {ActiveTermUUIDStore, SessionContextStore, TerminalsStore} from '../stores/stores'
  import {NIL as NIL_UUID, v4 as uuidv4} from 'uuid'

  let terminals = []
  let activeTermUUID = NIL_UUID
  let sessionContext = {}

  TerminalsStore.subscribe(updatedTerminals => terminals = updatedTerminals)
  ActiveTermUUIDStore.subscribe(updatedActiveTerminalUUID => activeTermUUID = updatedActiveTerminalUUID)
  SessionContextStore.subscribe(newSessionContext => sessionContext = newSessionContext)

  const addNewTerminal = () => {
    const NEW_TERM_UUID = uuidv4()
    TerminalsStore.update(terminals => {
      terminals.push({
        uuid: NEW_TERM_UUID,
        sessionContext: {
          user: "",
          suggestions: [],
          scriptResult: {},
          suggestionsContainer: null,
          lineText: ""
        },
        terminalInterface: null,
        ptyWebSocket: null,
        fitAddon: null
      })
      return terminals
    })

    setActive(NEW_TERM_UUID)
  }

  const setActive = (newActiveTermUUID) => {
    ActiveTermUUIDStore.update(_prevActiveTermUUID => newActiveTermUUID)

    const newActiveTerm = getTerminalByUuid(newActiveTermUUID)
    SessionContextStore.update(_prevSessionContext => newActiveTerm['sessionContext'])
  }

  const getTerminalByUuid = (termUuid) => {
    for (const term of terminals) {
      if (term['uuid'] == termUuid) {
        return term
      }
    }
    throw `Terminal with UUID=${termUuid} not found`
  }

  const closeTerminal = async (termUUID) => {
    const isSure = await confirm("Are you sure you want to close this tab?")
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
    <li class="tab" id="{terminal['uuid'] === activeTermUUID? 'selected-tab' : ''}" on:click={() => setActive(terminal['uuid'])}>
      <div class="tab-text">Terminal {index}</div>
      <div class="close-tab-button" on:click={() => closeTerminal(terminal['uuid'])}>x</div>
    </li>
  {/each}
  <li class="tab" on:click={addNewTerminal}><span>+</span></li>
</ol>

<style lang="scss">
  
  #tabs {
    display: flex;
    list-style: none;
    margin: 0;
    padding: 0;
    background-color: #333;
    color: #fff;
    background: linear-gradient(to top, #000, #444);
  }

  .tab {
    display: flex;
    align-items: center;
    height: 40px;
    padding: 0 20px;
    cursor: pointer;
    transition: all 1s;
  }

  .tab:hover {
    background: linear-gradient(to top, #000, rgb(18, 76, 103));
    color: #fff;
  }

  .tab#selected-tab {
    background: linear-gradient(to top, #000, rgb(23, 101, 137));
    color: #fff;
  }

  .tab-text {
    flex: 1;
    font-size: 14px;
    line-height: 40px;
  }

  .close-tab-button {
    margin-left: 10px;
    font-size: 14px;
    font-weight: bold;
    color: #fff;
    cursor: pointer;
    border: none;
    background-color: transparent;
    outline: none;
    padding: 0 10px;
    border-radius: 4px;
    transition: all 0.2s ease-in-out;
    visibility: hidden;
  }

  .tab:hover .close-tab-button {
    visibility: visible;
  }

  .close-tab-button:hover {
    background-color: #333;
    color: #ff000060;
  }

  .close-tab-button:active {
    background-color: #333;
  }

  .tab span {
    font-size: 24px;
    line-height: 40px;
  }


</style>