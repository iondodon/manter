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


const containersIds = {
  values: [
    {
      names: ['ids'],
      description: 'ides description',
      next: []
    }
  ],
  script: "docker container ps -a",
  postProcessor: (line) => {
    return {
      names: [line],
      description: 'container id',
      next: () => [containersIds]
    }
  }
}

const dockerOptions = {
  values: [
    {
      names: ['rm'],
      description: 'remove container',
      next: [containersIds]
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
    },
    {
      names: ['docker'],
      description: "docker description",
      next: [dockerOptions]
    }
  ]
}