<script lang="ts">
  import {SessionContextStore} from "../stores/stores"

  let sessionContext = {}
  SessionContextStore.subscribe(updatesSessionContext => sessionContext = updatesSessionContext)
</script>

<div id="bottom-bar">
  <div id="context-status">
    {#if sessionContext['scriptResult'] && sessionContext['scriptResult']['user_scripts']}
      {JSON.stringify(sessionContext['scriptResult']['user_scripts'])}
    {/if}
  </div>
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
    font-size: 8px;
    line-height: 40px;
    padding: 0 20px;
  }
</style>