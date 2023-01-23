  export type Group = {
    suggestions: Suggestion[]
  }

  export type Suggestion = {
    name: string;
    regex: RegExp;
    next?: () => (Group | Suggestion)[]
  }
