import type { Suggestion } from "./contract"

const filesOrFolders: Suggestion = {
  name: 'file or foler',
  regex: /^.*$/,
  next: () => [filesOrFolders]
}


const lsOptions = {
  suggestions: [
    {
      name: '-a',
      regex: /^-a$/,
      next: () => [lsOptions, filesOrFolders]
    },
    {
      name: '-l',
      regex: /^-l$/,
      next: () => [lsOptions, filesOrFolders]
    }
  ]
}

const clis = {
  suggestions: [
    {
      name: 'ls',
      regex: /^ls$/,
      next: () => [lsOptions, filesOrFolders]
    }
  ]
}

export default clis