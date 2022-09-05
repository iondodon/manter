<svelte:options accessors={true}/>

<script type="ts">
  import { afterUpdate } from 'svelte';
  import { IS_UNIX } from "../config/config"
  import { getByScript } from "../suggestions/GetByScript"
  import { COMMANDS } from "../suggestions/cmd-library/src/commands"
  import type { NamesProvider } from '../suggestions/cmd-library/src/contract/contract';

  export let script: string = ''
  export let lastWord = ''
  
  let candidateGroups = [ [COMMANDS] ]

  export let isVisibleSuggestionsBox = true
  
  let suggestionTaken = null
  let focusedSuggestionIndex = 0
  let focusedSuggestion = null
  
  export let filteredSuggestions = []
  let totalAfterFilterSuggestions = 0

  let isBoxReversed = false

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
      isBoxReversed = true
      suggestionsBoxElement.style.top = `${cursorRect.top - suggestionsBoxHeight - DISTANCE_FROM_CURSOR_PX}px`
    } else {
      isBoxReversed = false
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

    const DELTA = 15

    if (focusedSuggestionTop < suggestionsListTop) {
      suggestionsListElement.scrollTop = focusedSuggestionTop - DELTA
    } else if (focusedSuggestionBottom > suggestionsListBottom) {
      suggestionsListElement.scrollTop = focusedSuggestionBottom - suggestionsListElement.offsetHeight + DELTA
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
      for (let suggestion of suggestionsGroup['suggestions']) {
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


  const updateCandidateGroups = async (newCmdInput: string, sessionContext: object, copyPrevSuggestion) => {
    // arrows
    if (newCmdInput == '\u001b[D' || newCmdInput == '\u001b[C' || newCmdInput == "\u001b[A" || newCmdInput == "\u001b[B") {
      return
    }
    // arrows
    if (newCmdInput == '\u001bOD' || newCmdInput == '\u001bOC' || newCmdInput == "\u001bOA" || newCmdInput == "\u001bOB") {
      return
    }
    // esc
    if (newCmdInput == '\u001b') {
      return
    }
    // enter or ctrl+c
    if (newCmdInput === '\n' || newCmdInput === '\r' || newCmdInput == '\u0003') {
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

    if (copyPrevSuggestion) {
      candidateGroups[script.length] = candidateGroups[script.length - 1]
      return
    }

    let suggestionMatchFound = null
    let groupMatchFound = null
    for (let suggestionsGroup of candidateGroups[script.length - 1]) {
      if (IS_UNIX && suggestionsGroup['postProcessor']) {
        suggestionsGroup['suggestions'] = await getByScript(suggestionsGroup, sessionContext)
      }
      
      for (const suggestion of suggestionsGroup['suggestions']) {
        if (typeof suggestion['names'] == 'function') {
          suggestion['names'] = (suggestion['names'] as NamesProvider)()
        }

        for (const name of suggestion['names']) {
          if (!suggestionMatchFound && name == lastWord) {
            suggestionMatchFound = { ...suggestion }
            groupMatchFound = suggestionsGroup
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

    if (suggestionMatchFound['next']) {
      if (typeof suggestionMatchFound['next'] == "function") {
        suggestionMatchFound['next'] = suggestionMatchFound['next']()
      }
      candidateGroups[script.length] = suggestionMatchFound['next']
      return
    }
    if (groupMatchFound['next']) {
      if (typeof groupMatchFound['next'] == "function") {
        groupMatchFound['next'] = groupMatchFound['next']()
      }
      candidateGroups[script.length] = groupMatchFound['next']
      return
    }

    candidateGroups[script.length] = []
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
        groupSibling['suggestions'] = groupSibling['suggestions'].filter(sugg => matchesLastWord(sugg))
        if (groupSibling['suggestions'].length > 0) {
          filteredSuggestions = [...filteredSuggestions, groupSibling]
        }
      }
      for (let sugg of groupSibling['suggestions']) {
        sugg['index'] = totalAfterFilterSuggestions
        totalAfterFilterSuggestions++
      }
    }

    if (totalAfterFilterSuggestions == 0) {
      isVisibleSuggestionsBox = false
    }
  }


  export const updateSuggestions = async (newCmdInput: string, promptContext: object, copyPrevSuggestion) => {
    isVisibleSuggestionsBox = true
    await updateCandidateGroups(newCmdInput, promptContext, copyPrevSuggestion)
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
    focusedSuggestion = suggestion
  }
</script>


{#if isVisibleSuggestionsBox}
  <div id="suggestions-box">

    {#if isBoxReversed}
      {#if focusedSuggestion}
        <div class="suggestion-description">
          {#if focusedSuggestion['details'] && focusedSuggestion['details']['description']}
            {focusedSuggestion['details']['description']}
          {:else}
            {focusedSuggestion['names']}
          {/if}
        </div>
      {/if}
    {/if}

    <div id="suggestions-list">
      {#if filteredSuggestions.length > 0}
        {#each filteredSuggestions as suggestionsGroup}
          <div class="suggestions-group">
            {#each suggestionsGroup['suggestions'] as suggestion}
              {#if focusedSuggestionIndex == suggestion['index']}
                <div id="focused-suggestion">
                  <div class="suggestion">
                    {(() => { 
                      setFocusedSuggestion(suggestion)
                      return formatNames(suggestion['names']) 
                    })()}
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
        <div class="loading">
          loading...
        </div>
      {/if}
    </div>

    {#if !isBoxReversed}
      {#if focusedSuggestion}
        <div class="suggestion-description">
          {#if focusedSuggestion['details'] && focusedSuggestion['details']['description']}
            {focusedSuggestion['details']['description']}
          {:else}
            {focusedSuggestion['names']}
          {/if}
        </div>
      {/if}
    {/if}

    <div class="controls-info">
      <kbd>↑</kbd> and <kbd>↓</kbd> to navigate 
      <br/>
      <kbd>Tab</kbd> to take
      <br/>
      <kbd>Esc</kbd> to hide
    </div>

  </div>
{/if}


<style type="scss">
  #suggestions-box {
    display: flex;
    flex-direction: column;
    z-index: 4;
    width: 300px;
    position: absolute;
    top: 0;
    left: 0;
    background-color: rgb(160, 32, 32);
    border: 1px solid rgb(92, 24, 24);
  }

  #suggestions-list {
    max-height: 100px;
    width: 100%;
    overflow-y: auto;
    overflow-x: hidden;
    background-color: rgb(0, 0, 0);
    border: 2px solid rgb(77, 77, 77);
  }

  .controls-info {
    color: hsl(0, 0%, 67%);
    max-height: 70px;
    overflow-y: auto;
    background-color: hsl(0, 0%, 7%);
    border: 1px solid hsla(0, 0%, 45%, 0.697);
    width: 100%;
  }

  .suggestion-description {
    color: hsl(0, 0%, 67%);
    width: 100%;
    max-height: 70px;
    overflow-y: auto;
    border: 1px solid hsla(0, 0%, 36%, 0.697);
    background-color: hsl(0, 0%, 22%);
  }

  .loading {
    color: white;
  }

  #focused-suggestion {
    background-color: hsl(0, 0%, 37%);
  }

  .suggestion {
    color: hsl(0, 0%, 67%);
    border-bottom: 1px solid hsl(0, 0%, 14%);
    width: 100%;
    max-height: 70px;
    overflow-y: auto;
  }
</style>