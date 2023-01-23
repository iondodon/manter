import type { Group, Suggestion } from "./contract"

const filesOrFolders: Suggestion = {
  name: 'file or foler',
  regex: /^.*$/,
  next: () => [filesOrFolders]
}


const lsOptions: Group = {
  suggestions: [
    {
      name: '-a',
      regex: /^-a$/
    },
    {
      name: '-l',
      regex: /^-l$/
    }
  ],
  next: () => [lsOptions, filesOrFolders]
}

const clis: Group = {
  suggestions: [
    {
      name: 'ls',
      regex: /^ls$/,
      next: () => [lsOptions, filesOrFolders]
    }
  ]
}

export default clis