const gitAddOptions = {
  values: [
    {
      names: ['--verbose', '-v'],
      description: "Be verbose",
      next: []
    },
    {
      names: ['--dry-run', '-n'],
      description: "Don't actually add the file(s), just show if they exist and/or will be ignored.",
      next: []
    },
    {
      names: ['--force', '-f'],
      description: "Allow adding otherwise ignored files.",
      next: []
    }
  ]
}

export const highLevelCommands = [
  {
    names: ['add'],
    description: "add files",
    next: [gitAddOptions]
  }
]