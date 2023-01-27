<script lang="ts">
  import type { Group, Suggestion } from 'src/cli/library/contract';
  import clis from '../../cli/library/library'

  export let suggestions: (Group | Suggestion)[] = []

  const DISTANCE_FROM_CURSOR_PX = 5

  export const update = () => {
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
      <li>{JSON.stringify(suggestion)}</li>
    {/each}
  </ol>
{/if}

<style lang="scss">
  #suggestions-container {
    position: absolute;
    display: none;
    width: 300px;
    background-color: rgb(160, 32, 32);
    border: 1px solid rgb(92, 24, 24);
    list-style: none;
  }
</style>