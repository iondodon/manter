<svelte:options accessors={true}/>

<script type="ts">
  import { afterUpdate } from 'svelte';
  import { IS_UNIX } from "../config/config"
  import { getDynamicValues } from "../suggestions/GetDynamicValues"
  import { COMMANDS } from "../suggestions/library/commands"

  export let script: string = ''
  export let lastWord = ''
  
  let candidateGroups = [ [COMMANDS] ]

  export let isVisibleSuggestionsBox = true
  
  let suggestionTaken = null
  let focusedSuggestionIndex = 0
  
  export let filteredSuggestions = []
  let totalAfterFilterSuggestions = 0

  export const bringSuggestionsToCursor = () => {
    const suggestionsBoxElement = document.getElementById('suggestions-box')
    if (!suggestionsBoxElement) {
      return
    }
    if (script.length == 0) {
      suggestionsBoxElement.style.display = 'none'
      return
    }
    const cursorElement = document.getElementsByClassName('xterm-helper-textarea')[0]
    const cursorRect = cursorElement.getBoundingClientRect()

    suggestionsBoxElement.style.display = 'block'
    
    const suggestionsBoxHeight = suggestionsBoxElement.clientHeight
    const suggestionsBoxWidth = suggestionsBoxElement.clientWidth

    if (cursorRect.bottom + suggestionsBoxHeight > window.innerHeight) {
      suggestionsBoxElement.style.top = `${cursorRect.top - suggestionsBoxHeight}px`
    } else {
      suggestionsBoxElement.style.top = `${cursorRect.bottom}px`
    }

    if (cursorRect.right + suggestionsBoxWidth > window.innerWidth) {
      suggestionsBoxElement.style.left = `${cursorRect.left - suggestionsBoxWidth}px`
    } else {
      suggestionsBoxElement.style.left = `${cursorRect.right}px`
    }
  }

  const scrollToFocusedSuggestion = () => {
    const focusedSuggestionElement = document.getElementById("focused-suggestion")
    const suggestionsBoxElement = document.getElementById("suggestions-box")

    const focusedSuggestionTop = focusedSuggestionElement.offsetTop
    const focusedSuggestionBottom = focusedSuggestionTop + focusedSuggestionElement.offsetHeight
    const suggestionsBoxTop = suggestionsBoxElement.scrollTop
    const suggestionsBoxBottom = suggestionsBoxTop + suggestionsBoxElement.offsetHeight
    if (focusedSuggestionTop < suggestionsBoxTop) {
      suggestionsBoxElement.scrollTop = focusedSuggestionTop - 21
    } else if (focusedSuggestionBottom > suggestionsBoxBottom) {
      suggestionsBoxElement.scrollTop = focusedSuggestionBottom - suggestionsBoxElement.offsetHeight + 21
    }
  }

  export const focusOnNextSuggestion = () => {
    if (!isVisibleSuggestionsBox || totalAfterFilterSuggestions <= 1) {
      return
    }
    focusedSuggestionIndex = ++focusedSuggestionIndex % totalAfterFilterSuggestions
    scrollToFocusedSuggestion()
  }

  export const focusOnPrevSuggestion = () => {
    if (!isVisibleSuggestionsBox || totalAfterFilterSuggestions <= 1) {
      return
    }
    focusedSuggestionIndex = focusedSuggestionIndex - 1 < 0 ? totalAfterFilterSuggestions - 1 : focusedSuggestionIndex - 1
    scrollToFocusedSuggestion()
  }

  export const takeFocusedSuggestion = () => {
    if (!isVisibleSuggestionsBox || totalAfterFilterSuggestions < 1) {
      return
    }
    suggestionTaken = null
    for (let suggestionsGroup of filteredSuggestions) {
      for (let suggestion of suggestionsGroup['values']) {
        if (suggestion['index'] == focusedSuggestionIndex) {
          suggestionTaken = suggestion
          break
        }
      }
      if (suggestionTaken) {
        break
      }
    }

    if (!suggestionTaken) {
      alert("Couldn't take a suggestion")
    }

    if (script[script.length - 1] == ' ') {
      return suggestionTaken['names'][0]
    }

    for (let name of suggestionTaken['names']) {
      if (name == lastWord) {
        return ''
      }
      if (name.startsWith(lastWord)) {
        return name.substring(lastWord.length)
      }
    }

    alert("Matching names not found for taken suggestion")
  }

  const matchesLastWord = (suggestion) => {
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


  const updateCandidateGroups = async (newCmdInput: string, sessionContext: object) => {
    if (newCmdInput === '\n' || newCmdInput === '\r' || newCmdInput == '\x03') {
      candidateGroups = [ [COMMANDS] ]
      script = ''
      return
    }

    if (newCmdInput === '\b' || newCmdInput === '\x7f') {
      if (script.length > 0) {
        script = script.slice(0, -1)
      }
    } else {
      script += newCmdInput
    }

    lastWord = getLastScriptWord(script)

    if (script.length == 0) {
      return
    }

    let suggestionMatchFound = null
    for (let suggestionsGroup of candidateGroups[script.length - 1]) {
      if (IS_UNIX && suggestionsGroup['postProcessor']) {
        suggestionsGroup['values'] = await getDynamicValues(suggestionsGroup, sessionContext)
      }
      
      for (const suggestion of suggestionsGroup['values']) {
        if (typeof suggestion['names'] == 'function') {
          suggestion['names'] = (suggestion['names'] as Function)()
        }

        for (const name of suggestion['names']) {
          if (!suggestionMatchFound && name == lastWord) {
            suggestionMatchFound = { ...suggestion }
            break
          }
        }
        if (suggestionMatchFound) {
          break
        }
      }

    }

    if (!suggestionMatchFound) {
      candidateGroups[script.length] = candidateGroups[script.length - 1]
      return
    }

    if (typeof suggestionMatchFound['next'] == "function") {
      suggestionMatchFound['next'] = suggestionMatchFound['next']()
    }
    candidateGroups[script.length] = suggestionMatchFound['next']
  }

  const getLastScriptWord = (script: string): string => {
    const words = script.trim().split(' ')
    return words[words.length - 1].trim()
  }

  const filterSuggestions = () => {
    filteredSuggestions = []
    totalAfterFilterSuggestions = 0

    for (let suggestionsGroup of candidateGroups[script.length]) {
      let groupSibling = {...suggestionsGroup}
      if (script[script.length - 1] == " ") {
        filteredSuggestions = [...filteredSuggestions, groupSibling]
      } else {
        groupSibling['values'] = groupSibling['values'].filter(sugg => matchesLastWord(sugg))
        if (groupSibling['values'].length > 0) {
          filteredSuggestions = [...filteredSuggestions, groupSibling]
        }
      }
      for (let sugg of groupSibling['values']) {
        sugg['index'] = totalAfterFilterSuggestions
        totalAfterFilterSuggestions++
      }
    }

    if (totalAfterFilterSuggestions == 0) {
      isVisibleSuggestionsBox = false
    }
  }


  export const updateSuggestions = async (newCmdInput: string, promptContext: object) => {
    isVisibleSuggestionsBox = true
    await updateCandidateGroups(newCmdInput, promptContext)
    focusedSuggestionIndex = 0
    filterSuggestions()
  }

  afterUpdate(() => {
    bringSuggestionsToCursor()
  })
</script>


{#if isVisibleSuggestionsBox}
  <div id="suggestions-box">
  {#if filteredSuggestions.length > 0}
    {#each filteredSuggestions as suggestionsGroup}
      <div class="suggestions-group">
        {#each suggestionsGroup['values'] as suggestion}
          {#if focusedSuggestionIndex == suggestion['index']}
            <div id="focused-suggestion">
              <div class="suggestion">
                {JSON.stringify(suggestion["names"])}
              </div>
            </div>
          {:else}
            <div class="suggestion">
              {JSON.stringify(suggestion["names"])}
            </div>
          {/if}
        {/each}
      </div>
    {/each}
  {:else}
    loading...
  {/if}
</div>
{/if}


<style type="scss">
  #suggestions-box {
    z-index: 1;
    position: absolute;
    top: 0;
    left: 0;
    width: fit-content;
    max-height: 200px;
    font-size: 1.2em;
    font-family: monospace;
    display: none;
    overflow-y: auto;
    overflow-x: hidden;
    background-color: rgb(0, 0, 0);
    border: 1px solid rgb(77, 77, 77);
  }

  #focused-suggestion {
    background-color: hsl(0, 0%, 37%);
  }

  .suggestion {
    color: hsl(0, 0%, 67%);
    border-bottom: 1px solid hsl(0, 0%, 14%);
    margin: 2px;
  }

  .suggestions-group {
    margin: 2px;
  }
</style>