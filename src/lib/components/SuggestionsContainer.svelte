<script lang="ts">
  import { afterUpdate, beforeUpdate, onMount } from 'svelte';
  import clis from '../../cli/library/library'

  export let suggestions = []

  afterUpdate(() => {
    updateDisplyMode()
  })

  const DISTANCE_FROM_CURSOR_PX = 5

  export const updateDisplyMode = () => {
    const suggestionsContainerElement = document.getElementById('suggestions-container')

    if (!suggestionsContainerElement) {
      return
    }
    
    if (false) { // TODO: special cases
      suggestionsContainerElement.style.display = 'none'
    }

    suggestionsContainerElement.style.display = 'block'

    const cursorElement = document.getElementsByClassName('xterm-helper-textarea')[0]
    const cursorRect = cursorElement.getBoundingClientRect()
  
    const suggestionsContainerHeight = suggestionsContainerElement.clientHeight
    const suggestionsContainerWidth = suggestionsContainerElement.clientWidth

    if (cursorRect.bottom + suggestionsContainerHeight > window.innerHeight) {
      suggestionsContainerElement.style.top = `${cursorRect.top - suggestionsContainerHeight - DISTANCE_FROM_CURSOR_PX}px`
    } else {
      suggestionsContainerElement.style.top = `${cursorRect.bottom + DISTANCE_FROM_CURSOR_PX}px`
    }
    
    if (cursorRect.right + suggestionsContainerWidth > window.innerWidth) {
      suggestionsContainerElement.style.left = `${cursorRect.left - suggestionsContainerWidth - DISTANCE_FROM_CURSOR_PX}px`
    } else {
      suggestionsContainerElement.style.left = `${cursorRect.right + DISTANCE_FROM_CURSOR_PX}px`
    }
  }
</script>

{#if suggestions.length > 0 && suggestions[0] !== clis}
  <ol id="suggestions-container">
    {#each suggestions as suggestion}
      {#if suggestion.suggestions}
        <li>
          <ol class="suggestions-group">
            {#each suggestion.suggestions as subSuggestion}
              <li class="suggestion">
                <span>{subSuggestion.name}</span>
              </li>
            {/each}
          </ol>
        </li>
      {:else}
        <li class="suggestion">
          <span>{suggestion.name}</span>
        </li>
      {/if}
    {/each}
  </ol>
{/if}

<style lang="scss">
  #suggestions-container {
    position: absolute;
    display: none;
    max-width: 300px;
    background-color: rgb(160, 32, 32);
    border: 1px solid rgb(92, 24, 24);
    list-style: none;
  }

  .suggestions-group {
    list-style: none;
    color: aqua;
  }

  .suggestion {
    cursor: pointer;
    border: 1px solid rgb(92, 24, 24);
  }
</style>