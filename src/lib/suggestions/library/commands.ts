import { IS_WINDOWS } from "../../config/config"

const files = {
  values: [
    {
      names: ['files...'],
      description: 'list of files',
      next: function() { return [files] }
    }
  ],
  script: "ls -a",
  postProcessor: function (line) {
    return {
      names: [line],
      description: 'description of first file',
      next: function() { return [files] }
    }
  }
}


const lsOptions = {
  values: [
    {
      names: function() { return IS_WINDOWS ? ['-a'] : ['-a', '--all'] },
      description: 'ls all - description',
      next: function() { return [lsOptions, files] }
    }
  ]
}

export const COMMANDS = {
  values: [
    {
      names: ['ls'],
      description: "ls description",
      next: function() { return [lsOptions, files] }
    },
    {
      names: ['sudo'],
      description: "super user do",
      next: function() { return [] }
    }
  ]
}