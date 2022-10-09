<script lang="ts">
  import { TerminalsStore } from '../stores/stores'
  import { CurrentActiveStore } from '../stores/stores'

  let terminals
  TerminalsStore.subscribe(value => terminals = value)
  let currentActive;
  CurrentActiveStore.subscribe(value => currentActive = value)

  const addNewTerminal= () => {
    TerminalsStore.update(terminals => {
      terminals.push({
        id: terminals.length, 
        sessionContext: {
          isLoggedIn: false,
          cwd: "~",
          user: ""
        },
        terminalInterface: null,
        ptyWebSocket: null,
        fitAddon: null
      })
      return terminals
    })

    setActive(terminals.length - 1)
  }

  const setActive = (termId) => {
    CurrentActiveStore.update(oldActive => {
      return termId
    })

    const activeTerminalInterface = terminals[termId]['terminalInterface']
    if (activeTerminalInterface) {
      const terminal = document.getElementById('terminal')
      while (terminal.firstChild) {
        terminal.removeChild(terminal.firstChild)
      }

      activeTerminalInterface.open(document.getElementById('terminal'))
    }
  }
</script>

<ol id="tabs">
  {#each terminals as terminal}
    <li class="tab" on:click={() => setActive(terminal.id)}>
      <span class="tab-text">Terminal {terminal.id}</span>
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

  .tab {
    float: left;
    border: none;
    outline: none;
    cursor: pointer;
    transition: 0.3s;
    font-size: 17px;
    border: 1px solid rgb(136, 16, 16);
    text-align: center;
  }

  .tab:hover {
    background-color: #ddd;
  }
</style>