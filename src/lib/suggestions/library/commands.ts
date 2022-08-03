import { IS_WINDOWS } from "../../config/config"

const files = {
  values: [
    {
      names: ['files...'],
      description: 'list of files',
      next: () => [files]
    }
  ],
  script: "ls -a",
  postProcessor: (line) => {
    return {
      names: [line],
      description: 'description of first file',
      next: () => [files]
    }
  }
}


const lsOptions = {
  values: [
    {
      names: () => IS_WINDOWS ? ['-a'] : ['-a', '--all'],
      description: 'ls all - description',
      next: () => [lsOptions, files]
    }
  ]
}

export const COMMANDS = {
  values: [
    {
      names: ['ls'],
      description: "ls description",
      next: [lsOptions, files]
    },
    {
      names: ['sudo'],
      description: "super user do",
      next: () => [COMMANDS]
    }
  ]
}