<script lang="ts">
  import { TerminalsStore } from '../stores/stores'
  import { CurrentActiveStoreTermId } from '../stores/stores'

  let terminals
  TerminalsStore.subscribe(value => terminals = value)
  let currentActiveId;
  CurrentActiveStoreTermId.subscribe(value => currentActiveId = value)

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
    const terminalInterface = terminals[termId]['terminalInterface']
    if (terminalInterface) {
      const terminal = document.getElementById('terminal')
      while (terminal.firstChild) {
        terminal.removeChild(terminal.firstChild)
      }
      terminalInterface.open(document.getElementById('terminal'))
    }

    CurrentActiveStoreTermId.update(oldActive => {
      return termId
    })
  }
</script>

<ol id="tabs">
  {#each terminals as terminal}
    <li class="tab" on:click={() => setActive(terminal.id)}>
      {#if terminal.id == currentActiveId}
        <div id="selected-tab">
          <span class="tab-text">Terminal {terminal.id}</span>
        </div>
      {:else}
        <span class="tab-text">Terminal {terminal.id}</span>
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

  #selected-tab {
    background-color: rebeccapurple;
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