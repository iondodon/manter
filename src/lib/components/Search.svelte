<script lang="ts">
  import { ActiveTermUUIDStore, TerminalsStore } from '../stores/stores';

  let input = ''
  let terminals = []
  let activeTermUUID = null
  let activeTerminal = null

  TerminalsStore.subscribe(updatesTerminals => terminals = updatesTerminals)
  ActiveTermUUIDStore.subscribe(updatesActiveTermUUID => {
    activeTermUUID = updatesActiveTermUUID
    activeTerminal = terminals.find(term => term.uuid === activeTermUUID)
  })

  const search = (evt) => {
    if (evt.key !== 'Enter' || !activeTerminal) return

    const { addons } = activeTerminal
    const { searchAddon } = addons

    searchAddon.findNext(input)
  }

</script>

<div id="seach-control" >
  <input type="text" bind:value={input} on:keyup={search} />
</div>

<style lang="scss">
  #seach-control {
    margin-left: auto;
  }

  input {
    border: none;
    outline: none;
    font-size: 14px;
    color: #fff;
    background-color: transparent;
    border-bottom: 1px solid #fff;
    margin: 10px;
  }
</style>