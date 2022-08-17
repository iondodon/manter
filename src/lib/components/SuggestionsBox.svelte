<svelte:options accessors={true}/>

<script type="ts">
  import { afterUpdate } from 'svelte';
  import { IS_UNIX } from "../config/config"
  import { getDynamicValues } from "../suggestions/GetDynamicValues"
  import { COMMANDS } from "../suggestions/cmd-library/src/commands"

  export let script: string = ''
  export let lastWord = ''
  
  let candidateGroups = [ [COMMANDS] ]

  export let isVisibleSuggestionsBox = true
  
  let suggestionTaken = null
  let focusedSuggestionIndex = 0
  let focusedSuggestion = null
  
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
    const DISTANCE_FROM_CURSOR_PX = 5

    if (cursorRect.bottom + suggestionsBoxHeight > window.innerHeight) {
      suggestionsBoxElement.style.top = `${cursorRect.top - suggestionsBoxHeight - DISTANCE_FROM_CURSOR_PX}px`
    } else {
      suggestionsBoxElement.style.top = `${cursorRect.bottom + DISTANCE_FROM_CURSOR_PX}px`
    }

    if (cursorRect.right + suggestionsBoxWidth > window.innerWidth) {
      suggestionsBoxElement.style.left = `${cursorRect.left - suggestionsBoxWidth - DISTANCE_FROM_CURSOR_PX}px`
    } else {
      suggestionsBoxElement.style.left = `${cursorRect.right + DISTANCE_FROM_CURSOR_PX}px`
    }
  }

  const scrollToFocusedSuggestion = () => {
    const focusedSuggestionElement = document.getElementById("focused-suggestion")
    const suggestionsListElement = document.getElementById("suggestions-list")

    const focusedSuggestionTop = focusedSuggestionElement.offsetTop
    const focusedSuggestionBottom = focusedSuggestionTop + focusedSuggestionElement.offsetHeight
    const suggestionsListTop = suggestionsListElement.scrollTop
    const suggestionsListBottom = suggestionsListTop + suggestionsListElement.offsetHeight
    if (focusedSuggestionTop < suggestionsListTop) {
      suggestionsListElement.scrollTop = focusedSuggestionTop - 21
    } else if (focusedSuggestionBottom > suggestionsListBottom) {
      suggestionsListElement.scrollTop = focusedSuggestionBottom - suggestionsListElement.offsetHeight + 21
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

  const formatNames = (namesArr) => {
    return namesArr.join(', ')
  }

  const setFocusedSuggestion = (suggestion) => {
    if (suggestion['details'] && suggestion['details']['description']) {
      focusedSuggestion = suggestion
      return
    }
    focusedSuggestion = suggestion
  }
</script>


{#if isVisibleSuggestionsBox}
  <div id="suggestions-box">
    <div id="suggestions-list">
      {#if filteredSuggestions.length > 0}
        {#each filteredSuggestions as suggestionsGroup}
          <div class="suggestions-group">
            {#each suggestionsGroup['values'] as suggestion}
              {#if focusedSuggestionIndex == suggestion['index']}
                {setFocusedSuggestion(suggestion)}
                <div id="focused-suggestion">
                  <div class="suggestion">
                    {formatNames(suggestion['names'])}
                  </div>
                </div>
              {:else}
                <div class="suggestion">
                  {formatNames(suggestion['names'])}  
                </div>
              {/if}
            {/each}
          </div>
        {/each}
      {:else}
        loading...
      {/if}
    </div>
    <div class="suggestion">
      <kbd>↑</kbd> and <kbd>↓</kbd> to navigate 
      <br/>
      <kbd>Tab</kbd> to take
      <br/>
      <kbd>Esc</kbd> to hide
    </div>
    {#if focusedSuggestion}
      {focusedSuggestion['details']['description']}
    {/if}
  </div>
{/if}


<style type="scss">
  #suggestions-box {
    z-index: 1;
    position: absolute;
    top: 0;
    left: 0;
    font-size: 1rem;
    background-color: rgb(160, 32, 32);
    border: 1px solid rgb(34, 0, 158);
  }

  #suggestions-list {
    font-size: 1rem;
    overflow-y: auto;
    overflow-x: hidden;
    max-height: 100px;
    max-width: 200px;
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