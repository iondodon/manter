<svelte:options accessors={true}/>

<script type="ts">
  import { IS_WINDOWS } from "../config/config";

  import { getDynamicValues } from "../suggestions/GetDynamicValues"
  import { COMMANDS } from "../suggestions/library/commands"
  import SuggestionsWrapper from "./SuggestionsWrapper.svelte";

  let script: string = ''
  let lastWord = ''
  let currentSuggestions = []
  let suggestionsCarrier = [ [COMMANDS] ]

  const getLastScriptWord = (script: string): string => {
    const words = script.trim().split(' ')
    return words[words.length - 1]
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
      if (!IS_WINDOWS && candidatesWrapper['processor']) {
        candidatesWrapper['values'] = await getDynamicValues(candidatesWrapper, promptContext["cwd"])
      }
      
      let candidates = candidatesWrapper['values']
      for (const candidate of candidates) {
        if (typeof candidate['names'] == 'function') {
          candidate['names'] = candidate['names']()
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

    suggestionsCarrier[script.length] = selected['getNext']()
  }

  export const updateSuggestions = async (newCmdInput: string, promptContext: object) => {
    await processSuggestions(newCmdInput, promptContext)
    currentSuggestions = suggestionsCarrier[script.length]
  }

  export const bringSuggestionsToCursor = () => {
    const suggestionsElement = document.getElementById('suggestions-box')
    
    if (suggestionsElement && script.length == 0) {
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


{#if currentSuggestions}
  <div id="suggestions-box">
      {#each currentSuggestions as wrapper}
        <SuggestionsWrapper wrapper={wrapper} script={script} lastWord={lastWord} />
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
</style>