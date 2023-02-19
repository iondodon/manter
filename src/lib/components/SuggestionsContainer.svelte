<script lang="ts">
  import { resolveDynamicGroups }  from '../../suggestions/suggestions';
  import clis from '../../cli/library/library'
  import { IS_UNIX } from '../config/config';
  import { afterUpdate } from 'svelte';

  const DISTANCE_FROM_CURSOR_PX = 5

  export let sessionContext
  export let selectedIndex = 0
  
  let totalSuggestions = 0
  let filteredSuggestions = []
  let lineText = ''

  $: (async () => {
    if (IS_UNIX && sessionContext['lineText'].endsWith(' ')) {
      await resolveDynamicGroups(sessionContext)
    }

    filteredSuggestions = sessionContext['filteredSuggestions']
    lineText = sessionContext['lineText']

    selectedIndex = 0
    setIndexes()
  })()

  afterUpdate(() => {
    updateContainerDisplay()
  })

  document.addEventListener("keydown", (event) => {
    if (event.key === "ArrowDown") {
      selectedIndex = selectedIndex + 1 > totalSuggestions - 1 ? 0 : selectedIndex + 1
    }
    if (event.key === "ArrowUp") {
      selectedIndex = selectedIndex - 1 < 0 ? totalSuggestions - 1 : selectedIndex - 1
    }
  })

  const setIndexes = () => {
    let cumulativeIndex = -1

    for (const suggestion of filteredSuggestions) {
      if (suggestion.suggestions) {
        for (const subSuggestion of suggestion.suggestions) {
          cumulativeIndex++
          subSuggestion['index'] = cumulativeIndex
        }
      } else {
        cumulativeIndex++
        suggestion['index'] = cumulativeIndex
      }
    }

    totalSuggestions = cumulativeIndex + 1
  }

  const updateContainerDisplay = () => {
    const suggestionsContainerElement = document.getElementById('suggestions-container')

    if (!suggestionsContainerElement) {
      return
    }

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

    const selectedSuggestionElement = document.querySelector('.selected-suggestion');
    if (selectedSuggestionElement) {
      selectedSuggestionElement.scrollIntoView({block: "nearest"});
    }
  }
</script>

{#if filteredSuggestions.length > 0 && filteredSuggestions[0] !== clis}
  <ol id="suggestions-container">
    {#each filteredSuggestions as suggestion}
      {#if suggestion.suggestions}
        <li>
          <ol class="suggestions-group">
            {#each suggestion.suggestions as subSuggestion}
              <li class={subSuggestion['index'] == selectedIndex ? "selected-suggestion" : "suggestion"} >
                <span>{subSuggestion.name}</span>
              </li>
            {/each}
          </ol>
        </li>
      {:else}
        <li class={suggestion['index'] == selectedIndex ? "selected-suggestion" : "suggestion"}>
          <span>{suggestion.name}</span>
        </li>
      {/if}
    {/each}
    <li class="suggestion">
      <span>Esc. to close</span>
    </li>
  </ol>
{/if}

<style lang="scss">
  #suggestions-container {
    position: absolute;
    max-width: 300px;
    max-height: 120px;
    overflow-y: auto;
    overflow-x: hidden;
    list-style: none;
    z-index: 3;
    border: 1px solid rgb(84, 84, 84);
    background-color: rgb(36, 36, 36);
  }

  ::-webkit-scrollbar-track {
    background-color: rgb(36, 36, 36);
  }

  .suggestions-group {
    border-left: 2px solid rgb(0, 139, 28);
    margin-bottom: 10px;
    list-style: none;
  }

  .suggestion {
    border: 1px solid rgb(74, 74, 74);
    padding-left: 5px;
    color: bisque;
  }

  .selected-suggestion {
    padding-left: 5px;
    border: 1px solid rgb(74, 74, 74);
    background-color: rgb(126, 126, 126);
    color: white;
  }
</style>