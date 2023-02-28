<script lang="ts">
  import {ActiveSessionContextStore} from "../stores/stores"
  import Search from "./Search.svelte"

  let sessionContext = {}
  ActiveSessionContextStore.subscribe(updatesSessionContext => sessionContext = updatesSessionContext)
</script>

<div id="bottom-bar">
  {#if sessionContext['searchIsOn']}
    <Search />
  {:else}
    <div id="context-status">
      {#if sessionContext['prompt_command_result'] && sessionContext['prompt_command_result']['user_scripts']}
        {JSON.stringify(sessionContext['prompt_command_result']['user_scripts'])}
      {/if}
    </div>
  {/if}
</div>

<style lang="scss">
  #bottom-bar {
    display: flex;
    align-items: center;
    height: 40px;
    background: linear-gradient(to bottom, #000, #333);
    color: #fff;
  }

  #context-status {
    flex: 1;
    font-size: 12px;
    line-height: 40px;
    padding: 0 20px;
  }
</style>