<script lang="ts">
  import { resolveDynamicGroups }  from '../../suggestions/suggestions';
  import { afterUpdate } from 'svelte';
  import clis from '../../cli/library/library'
  import { IS_UNIX } from '../config/config';

  export let suggestions = []
  export let lineText = ''
  export let sessionContext: object

  afterUpdate(async () => {
    if (IS_UNIX) {
      await resolveDynamicGroups(suggestions, sessionContext)
    }
    updateDisplyMode()
  })

  const DISTANCE_FROM_CURSOR_PX = 5

  const updateDisplyMode = () => {
    const suggestionsContainerElement = document.getElementById('suggestions-container')

    if (!suggestionsContainerElement) {
      return
    }
    
    if (false) { // TODO: special cases
      suggestionsContainerElement.style.display = 'none'
    }

    suggestionsContainerElement.style.display = 'block'

    const cursorElement = document.getElementsByClassName('xterm-helper-textarea')[0]

    if (!cursorElement) {
      return
    }

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

{#if suggestions.length > 0 && suggestions[0] !== clis && lineText.endsWith(' ')}
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
    max-height: 120px;
    overflow-y: scroll;
    overflow-x: hidden;
    list-style: none;
    z-index: 3;
    border: 1px solid rgb(84, 84, 84);
    background-color: rgba(0, 0, 0, 0.608);
  }

  .suggestions-group {
    list-style: none;
  }

  .suggestion {
    border: 1px solid rgb(74, 74, 74);
    color: bisque;
  }
</style>