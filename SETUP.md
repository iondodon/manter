### MacOS

On MacOS the following commands should be executed in the terminal before starting the application.
These two are required to runt the COMMAND_PROMPT script each time the prompt appears in the terminal.
The COMMAND_PROMPT script by default provides useful information about the current git branch, the current user the current directory.
The script can be modified in the sourcecode. In the future will make an option for the user to defined the script.

Add the following two command in ~/.profile or ~/.zshenv (or both - not sure). The run source them.

```bash

prmptcmd() { eval "$PROMPT_COMMAND" }

precmd_functions=(prmptcmd)

```

```bash
source ~/.profile
source ~/.zshenv
```
