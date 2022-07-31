export const ab2str = (buf: ArrayBuffer) => {
  return String.fromCharCode.apply(null, new Uint8Array(buf))
}

export const isVisibleSuggestion = (suggestion, script, lastWord) => {
  if (script[script.length - 1] == " ") {
    return true
  }

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