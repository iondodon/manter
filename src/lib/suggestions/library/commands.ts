import { IS_WINDOWS } from "../../config/config"

const files = {
  values: [
    {
      'names': ['files...'],
      'description': 'list of files',
      'getNext': function() { return [files] }
    }
  ],
  script: "ls -a",
  processor: function (line) {
    return {
      'names': [line],
      'description': 'description of first file',
      'getNext': function() { return [files] }
    }
  }
}


const lsOptions = {
  values: [
    {
      'names': function() { return IS_WINDOWS ? ['-a'] : ['-a', '--all'] },
      'description': 'ls all - description',
      'getNext': function() { return [lsOptions, files] }
    }
  ]
}

export const COMMANDS = {
  values: [
    {
      'names': ['ls'],
      'description': "ls description",
      'getNext': function() { return [lsOptions, files] }
    },
    {
      'names': ['sudo'],
      'description': "super user do",
      'getNext': function() { return [] }
    }
  ]
}