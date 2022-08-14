<svelte:options accessors={true}/>

<script type="ts">
  import { IS_WINDOWS } from "../config/config"
  import { getDynamicValues } from "../suggestions/GetDynamicValues"
  import { COMMANDS } from "../suggestions/library/commands"

  export let script: string = ''
  export let lastWord = ''
  
  let suggestionsCarrier = [ [COMMANDS] ]

  export let isVisible = true
  
  let suggestionTaken = null
  let focusedSuggestionIndex = 0
  
  export let filteredSuggestions = []
  let totalAfterFilterSuggestions = 0

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
    if (!isVisible || totalAfterFilterSuggestions <= 1) {
      return
    }
    focusedSuggestionIndex = ++focusedSuggestionIndex % totalAfterFilterSuggestions
    scrollToFocusedSuggestion()
  }

  export const focusOnPrevSuggestion = () => {
    if (!isVisible || totalAfterFilterSuggestions <= 1) {
      return
    }
    focusedSuggestionIndex = focusedSuggestionIndex - 1 < 0 ? totalAfterFilterSuggestions - 1 : focusedSuggestionIndex - 1
    scrollToFocusedSuggestion()
  }

  export const takeSuggestion = () => {
    if (!isVisible || filteredSuggestions.length < 1 || totalAfterFilterSuggestions < 1) {
      return
    }
    suggestionTaken = null
    for (let wrp of filteredSuggestions) {
      for (let sugg of wrp['values']) {
        if (sugg['index'] == focusedSuggestionIndex) {
          suggestionTaken = sugg
          break
        }
      }
      if (suggestionTaken) {
        break
      }
    }

    if (!suggestionTaken) {
      alert("Should be a suggestion here")
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

    alert("matching names not found")
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

  const getLastScriptWord = (script: string): string => {
    const words = script.trim().split(' ')
    return words[words.length - 1].trim()
  }


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
    suggestionsBoxElement.style.top = `${cursorRect.top + 20}px`
    suggestionsBoxElement.style.left = `${cursorRect.left + 10}px`
  }


  const processSuggestions = async (newCmdInput: string, sessionContext: object) => {
    if (newCmdInput === '\n' || newCmdInput === '\r' || newCmdInput == '\x03') {
      suggestionsCarrier = [ [COMMANDS] ]
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

    let selectedMatched = null
    for (let candidatesWrapper of suggestionsCarrier[script.length - 1]) {
      if (!IS_WINDOWS && candidatesWrapper['postProcessor']) {
        candidatesWrapper['values'] = await getDynamicValues(candidatesWrapper, sessionContext)
      }
      
      let suggestionsCandidates = candidatesWrapper['values']
      for (const suggestionCandidate of suggestionsCandidates) {
        if (typeof suggestionCandidate['names'] == 'function') {
          suggestionCandidate['names'] = (suggestionCandidate['names'] as Function)()
        }

        for (const name of suggestionCandidate['names']) {
          if (!selectedMatched && name == lastWord) {
            selectedMatched = { ...suggestionCandidate }
            break
          }
        }
        if (selectedMatched) {
          break
        }
      }

    }

    if (!selectedMatched) {
      suggestionsCarrier[script.length] = suggestionsCarrier[script.length - 1]
      return
    }

    if (typeof selectedMatched['next'] == "function") {
      selectedMatched['next'] = selectedMatched['next']()
    }
    suggestionsCarrier[script.length] = selectedMatched['next']
  }


  export const updateSuggestions = async (newCmdInput: string, promptContext: object) => {
    isVisible = true
    await processSuggestions(newCmdInput, promptContext)

    filteredSuggestions = []
    totalAfterFilterSuggestions = 0
    focusedSuggestionIndex = 0
    for (let wrp of suggestionsCarrier[script.length]) {
      let newWrp = {...wrp}
      if (script[script.length - 1] == " ") {
        filteredSuggestions = [...filteredSuggestions, newWrp]
      } else {
        newWrp['values'] = newWrp['values'].filter(sugg => matchesLastWord(sugg))
        if (newWrp['values'].length > 0) {
          filteredSuggestions = [...filteredSuggestions, newWrp]
        }
      }
      for (let sugg of newWrp['values']) {
        sugg['index'] = totalAfterFilterSuggestions
        totalAfterFilterSuggestions++
      }
    }

    if (filteredSuggestions.length == 0) {
      isVisible = false
    }
  }
</script>


{#if isVisible}
  <div id="suggestions-box">
  {#if filteredSuggestions.length > 0}
    {#each filteredSuggestions as wrapper}
      <div class="suggestions-wrapper">
        {#each wrapper['values'] as suggestion}
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
    z-index: 1000;
    position: absolute;
    top: 0;
    left: 0;
    width: fit-content;
    max-height: 200px;
    background-color: rgb(19, 11, 127);
    color: rgb(218, 17, 17);
    font-size: 1.2em;
    font-family: monospace;
    border: 1px solid rgb(222, 21, 21);
    display: none;
    overflow-y: auto;
    overflow-x: hidden;
  }

  #focused-suggestion {
    background-color: aqua;
  }

  .suggestion {
    border-bottom: 1px solid rgb(0, 0, 0);
    margin: 2px;
  }

  .suggestions-wrapper {
    border-bottom: 3px solid rgb(7, 115, 3);
    margin: 2px;
  }
</style>