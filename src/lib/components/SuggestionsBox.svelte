<svelte:options accessors={true}/>

<script type="ts">
  import { IS_WINDOWS } from "../config/config"
  import { getDynamicValues } from "../suggestions/GetDynamicValues"
  import { COMMANDS } from "../suggestions/library/commands"

  let script: string = ''
  let lastWord = ''
  let currentSuggestions = []
  let visibleSuggestions = []
  let suggestionsCarrier = [ [COMMANDS] ]
  $: hasSuggestionCandidates = visibleSuggestions.length > 0
  export let isVisibleBox = true

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
            selected = candidate
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

    suggestionsCarrier[script.length] = selected['next']()
  }

  export const updateSuggestions = async (newCmdInput: string, promptContext: object) => {
    isVisibleBox = true
    await processSuggestions(newCmdInput, promptContext)
    currentSuggestions = suggestionsCarrier[script.length]

    visibleSuggestions = []
    for (let wrp of currentSuggestions) {
      let newWrp = {...wrp}
      if (script[script.length - 1] == " ") {
        visibleSuggestions = [...visibleSuggestions, newWrp]
        continue
      }
      newWrp['values'] = newWrp['values'].filter(sugg => isCandidate(sugg))
      if (newWrp['values'].length > 0) {
        visibleSuggestions = [...visibleSuggestions, newWrp]
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


{#if hasSuggestionCandidates && isVisibleBox}
  <div id="suggestions-box">
    {#each visibleSuggestions as wrapper}
      <div class="suggestions-wrapper">
        {#each wrapper['values'] as suggestion}            
          <div class="suggestion">
              {JSON.stringify(suggestion["names"])}
          </div>
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

  .suggestion {
    border-bottom: 1px solid rgb(0, 0, 0);
    margin: 2px;
  }

  .suggestions-wrapper {
    border-bottom: 3px solid rgb(7, 115, 3);
    margin: 2px;
  }
</style>