<svelte:options accessors={true}/>

<script type="ts">
  import { IS_WINDOWS } from "../config/config"
  import { getDynamicValues } from "../suggestions/GetDynamicValues"
  import { COMMANDS } from "../suggestions/library/commands"

  export let script: string = ''
  export let lastWord = ''
  export let filteredSuggestions = []
  let suggestionsCarrier = [ [COMMANDS] ]
  $: hasSuggestionCandidates = filteredSuggestions.length > 0
  export let isVisible = true
  let suggestionTaken = null
  
  let selectedSuggestionIndex = 0
  let totalSuggestions = 0

  const scrollToSelectedSuggestion = () => {
    const selectedSuggestionElement = document.getElementById("selected-suggestion")
    const suggestionsBoxElement = document.getElementById("suggestions-box")

    const selectedSuggestionTop = selectedSuggestionElement.offsetTop
    const selectedSuggestionBottom = selectedSuggestionTop + selectedSuggestionElement.offsetHeight
    const suggestionsBoxTop = suggestionsBoxElement.scrollTop
    const suggestionsBoxBottom = suggestionsBoxTop + suggestionsBoxElement.offsetHeight
    if (selectedSuggestionTop < suggestionsBoxTop) {
      suggestionsBoxElement.scrollTop = selectedSuggestionTop - 21
    } else if (selectedSuggestionBottom > suggestionsBoxBottom) {
      suggestionsBoxElement.scrollTop = selectedSuggestionBottom - suggestionsBoxElement.offsetHeight + 21
    }
  }

  export const selectNextSuggestion = () => {
    if (!isVisible) {
      return
    }
    if (totalSuggestions == 0 || totalSuggestions == 1) {
      return
    }
    selectedSuggestionIndex++
    if (selectedSuggestionIndex == totalSuggestions) {
      selectedSuggestionIndex = 0
    }
    scrollToSelectedSuggestion()
  }

  export const selectPrevSuggestion = () => {
    if (!hasSuggestionCandidates) {
      return
    }
    if (totalSuggestions == 0 || totalSuggestions == 1) {
      return
    }
    selectedSuggestionIndex--
    if (selectedSuggestionIndex < 0) {
      selectedSuggestionIndex = totalSuggestions - 1
    }
    scrollToSelectedSuggestion()
  }

  export const takeSuggestion = () => {
    if (!isVisible || !hasSuggestionCandidates || totalSuggestions < 1) {
      return
    }
    suggestionTaken = null
    for (let wrp of filteredSuggestions) {
      for (let sugg of wrp['values']) {
        if (sugg['index'] == selectedSuggestionIndex) {
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

  const isCandidate = (suggestion) => {
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

  const processSuggestions = async (newCmdInput: string, promptContext: object) => {
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

    let selected = null
    for (let candidatesWrapper of suggestionsCarrier[script.length - 1]) {
      if (!IS_WINDOWS && candidatesWrapper['postProcessor']) {
        candidatesWrapper['values'] = await getDynamicValues(candidatesWrapper, promptContext["cwd"])
      }
      
      let candidates = candidatesWrapper['values']
      for (const candidate of candidates) {
        if (typeof candidate['names'] == 'function') {
          candidate['names'] = (candidate['names'] as Function)()
        }

        for (const name of candidate['names']) {
          if (!selected && name == lastWord) {
            selected = { ...candidate }
            break
          }
        }
        if (selected) {
          break
        }
      }

    }

    if (!selected) {
      suggestionsCarrier[script.length] = suggestionsCarrier[script.length - 1]
      return
    }

    if (typeof selected['next'] == "function") {
      selected['next'] = selected['next']()
    }
    suggestionsCarrier[script.length] = selected['next']
  }

  export const updateSuggestions = async (newCmdInput: string, promptContext: object) => {
    isVisible = true
    await processSuggestions(newCmdInput, promptContext)

    filteredSuggestions = []
    totalSuggestions = 0
    selectedSuggestionIndex = 0
    for (let wrp of suggestionsCarrier[script.length]) {
      let newWrp = {...wrp}
      if (script[script.length - 1] == " ") {
        filteredSuggestions = [...filteredSuggestions, newWrp]
      } else {
        newWrp['values'] = newWrp['values'].filter(sugg => isCandidate(sugg))
        if (newWrp['values'].length > 0) {
          filteredSuggestions = [...filteredSuggestions, newWrp]
        }
      }
      for (let sugg of newWrp['values']) {
        sugg['index'] = totalSuggestions
        totalSuggestions++
      }
    }
  }

  export const bringSuggestionsToCursor = () => {
    const suggestionsElement = document.getElementById('suggestions-box')
    if (!suggestionsElement) {
      return
    }
    if (script.length == 0) {
      suggestionsElement.style.display = 'none'
      return
    }
    const cursorHtml = document.getElementsByClassName('xterm-helper-textarea')[0]
    const rect = cursorHtml.getBoundingClientRect()

    suggestionsElement.style.display = 'block'
    suggestionsElement.style.top = `${rect.top + 20}px`
    suggestionsElement.style.left = `${rect.left + 10}px`
  }
</script>


{#if isVisible}
  <div id="suggestions-box">
  {#each filteredSuggestions as wrapper}
    <div class="suggestions-wrapper">
      {#each wrapper['values'] as suggestion}
        {#if selectedSuggestionIndex == suggestion['index']}
          <div id="selected-suggestion">
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
    overflow-y: scroll;
  }

  #selected-suggestion {
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