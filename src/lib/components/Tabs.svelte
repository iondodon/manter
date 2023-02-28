<script lang="ts">
  import {ActiveTermUUIDStore, ActiveSessionContextStore, TerminalsStore} from '../stores/stores'
  import {NIL as NIL_UUID, v4 as uuidv4} from 'uuid'
  import { afterUpdate } from 'svelte';

  let terminals = []
  let activeTermUUID = NIL_UUID
  let sessionContext = {}

  TerminalsStore.subscribe(updatedTerminals => terminals = updatedTerminals)
  ActiveTermUUIDStore.subscribe(updatedActiveTerminalUUID => activeTermUUID = updatedActiveTerminalUUID)
  ActiveSessionContextStore.subscribe(newSessionContext => sessionContext = newSessionContext)

  const handleChangeTabName = () => {
    const tabsElements = document.querySelectorAll('.tab');
    tabsElements.forEach(tabElement => {
      tabElement.addEventListener('dblclick', () => {
        const tabNameElement = tabElement.querySelector('.tab-name')

        if (!tabNameElement) {
          return
        }

        const tabName = tabNameElement.textContent

        const tabNameInput = document.createElement('input')
        tabNameInput.value = tabName
        tabNameInput.style.width = '100%'
        tabNameInput.style.height = '100%'
        tabNameInput.style.border = 'none'
        tabNameInput.style.outline = 'none'
        tabNameInput.style.fontSize = '14px'
        tabNameInput.style.color = '#fff'
        tabNameInput.style.backgroundColor = 'transparent'
        tabNameInput.style.borderBottom = '1px solid #fff'

        tabNameElement.replaceWith(tabNameInput)
        tabNameInput.focus()

        const replaceInputWithName = () => {
          const newTabName = tabNameInput.value
          tabNameInput.replaceWith(tabNameElement)
          tabNameElement.textContent = newTabName
        }

        tabNameInput.addEventListener('keydown', (event) => {
          if (event.key === 'Enter' || event.key === 'Escape') {
            replaceInputWithName()
          }
        })

        tabNameInput.addEventListener('blur', () => {
          replaceInputWithName()
        })
      })
    })
  }

  afterUpdate(() => {
    handleChangeTabName()
  })

  const addNewTerminal = () => {
    const NEW_TERM_UUID = uuidv4()
    TerminalsStore.update(terminals => {
      terminals.push({
        uuid: NEW_TERM_UUID,
        sessionContext: {
          user: "",
          suggestions: [],
          filteredSuggestions: [],
          prompt_command_result: {},
          suggestionsContainer: null,
          lineText: "",
          searchIsOn: false,
        },
        terminalInterface: null,
        ptyWebSocket: null,
        addons: {}
      })
      return terminals
    })

    setActive(NEW_TERM_UUID)
  }

  const setActive = (newActiveTermUUID) => {
    ActiveTermUUIDStore.update(_prevActiveTermUUID => newActiveTermUUID)

    const newActiveTerm = getTerminalByUuid(newActiveTermUUID)
    ActiveSessionContextStore.update(_prevSessionContext => newActiveTerm['sessionContext'])
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
    
    await TerminalsStore.update(terminals => terminals.filter(term => term['uuid'] != termUUID))
    
    for(const term of terminals) {
      if (term['uuid']) {
        setActive(term['uuid'])
        return
      }
    }
  }
</script>

<ol id="tabs">
  {#each terminals as terminal, _index}
    <li class="tab" id="{terminal['uuid'] === activeTermUUID? 'selected-tab' : ''}" 
        on:click={() => setActive(terminal['uuid'])}
        on:keypress={() => setActive(terminal['uuid'])}
    >
      <div class="tab-name">New Session</div>
      <div class="close-tab-button" 
        on:click={() => closeTerminal(terminal['uuid'])} 
        on:keypress={() => closeTerminal(terminal['uuid'])} 
      >
        x
      </div>
    </li>
  {/each}
  <li class="tab" on:click={addNewTerminal} on:keypress={addNewTerminal}>
    <span>+</span>
  </li>
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

  .tab-name {
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