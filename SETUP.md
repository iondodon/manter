### MacOS

On MacOS the following commands should be executed in the terminal.
These two are required to runt the PROMPT_COMMAND script each time the prompt appears.
The PROMPT_COMMAND script by default provides useful information about the current git branch, the current user and the current directory.
The script can be modified in the sourcecode. In the future will make an option for the user to defined the script to provide the information they want.

Add the following two command in ~/.profile or ~/.zshenv (or both - not sure). The run source them. 
Or execute them in Manter after the app has started and the user is logged in. 

```bash

prmptcmd() { eval "$PROMPT_COMMAND" }

precmd_functions=(prmptcmd)

```

```bash
source ~/.profile
source ~/.zshenv
```
