<script type="ts">
  export let suggestion;
  export let script;
  export let lastWord;

  const isVisible = (suggestion) => {
    if (script[script.length - 1] == " ") {
      return true
    }

    if (typeof suggestion['names'] == 'function') {
      suggestion['names'] = suggestion['names']()
    }

    for (const name of suggestion["names"]) {
      if (name == lastWord || name.startsWith(lastWord)) {
        return true
      }
    }
    return false
  }
</script>

{#if isVisible(suggestion)}
  <div class="suggestion">
    {JSON.stringify(suggestion["names"])}
  </div>
{/if}

<style lang="scss">
 .suggestion {
    border-bottom: 1px solid rgb(0, 0, 0);
    margin: 2px;
  }
</style>