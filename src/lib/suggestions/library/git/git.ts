import { highLevelCommands } from "./high-level-commands"
import { lowLevelCommands } from "./low-level-commands"

const command = {
  values: [
    ...highLevelCommands,
    ...lowLevelCommands
  ]
}

export const git = {
  names: ['git'],
  description: "git tool",
  next: [command]
}