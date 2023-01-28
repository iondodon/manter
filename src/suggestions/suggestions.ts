  import clis  from '../cli/library/library'

const getSuggestions = (input) => {
    let next: any = [clis];
    let words = input.trim().split(" ").filter(word => word.length > 0)
    
    for (const word of words) {

      let found = false
      for (const item of next) {

        if (item.suggestions) {
          // in a group
          for (const suggestion of item.suggestions) {
            if (suggestion.regex.test(word)) {
              if (suggestion.next)
                next = suggestion.next()
              else if(item.next) 
                next = item.next()
              else next = []
              found = true
              break
            }
          }
        } else if (item.regex) {
          // single suggestion
          const suggestion = item
          if (suggestion.regex.test(word)) {
            if (suggestion.next)
              next = suggestion.next()
            else next = []
            found = true
            break
          }
        } else {
          console.log("yoklmn")
        }

        if (found) {
          break
        }
      }

    }

    return next;
  }

  export default getSuggestions