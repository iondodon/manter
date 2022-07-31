<script type="ts">
  import Suggestion from  "./Suggestion.svelte"

  export let wrapper;
  export let script;
  export let lastWord;
</script>

<div class="suggestions-wrapper">
  {#each wrapper['values'] as suggestion}
    <Suggestion suggestion={suggestion} script={script} lastWord={lastWord} />
  {/each}
</div>

<style lang="scss">
  .suggestions-wrapper {
    border-bottom: 3px solid rgb(7, 115, 3);
    margin: 2px;
  }
</style>