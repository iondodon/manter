import { IS_WINDOWS } from "../../config/config"

const files = {
  values: [
    {
      names: ['files...'],
      description: 'list of files',
      next: () => { return [files] }
    }
  ],
  script: "ls -a",
  postProcessor: (line) => {
    return {
      names: [line],
      description: 'description of first file',
      next: () => { return [files] }
    }
  }
}


const lsOptions = {
  values: [
    {
      names: () => { return IS_WINDOWS ? ['-a'] : ['-a', '--all'] },
      description: 'ls all - description',
      next: () => { return [lsOptions, files] }
    }
  ]
}

export const COMMANDS = {
  values: [
    {
      names: ['ls'],
      description: "ls description",
      next: () => { return [lsOptions, files] }
    },
    {
      names: ['sudo'],
      description: "super user do",
      next: () => { return [] }
    }
  ]
}