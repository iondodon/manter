<script lang="ts">
  import {SessionContextStore} from "../stores/stores"

  let sessionContext = {}
  SessionContextStore.subscribe(updatesSessionContext => sessionContext = updatesSessionContext)
</script>

<div id="bottom-bar">
  <div id="context-status">
    {JSON.stringify(sessionContext)}
  </div>
</div>

<style lang="scss">
  #bottom-bar {
    display: flex;
    flex-direction: row;
    justify-content: space-between;

    bottom: 0;
    width: 100%;
    background-color: rgb(29, 29, 29);
    color: rgb(156, 156, 156);

    display: flex;
    align-items: center;
  }

  #bottom-bar * {
    margin-left: 0.2rem;
    margin-right: 0.2rem;
  }
</style>