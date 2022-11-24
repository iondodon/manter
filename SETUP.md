## Envirnoment variables

### Common Unix

```bash

PROMPT_COMMAND='echo -en "\033]0; [manter]
                  {
                    \"cwd\": \"$(pwd)\",
                    \"git\": {
                      \"currentBranch\" : \"$(git rev-parse --abbrev-ref HEAD 2> /dev/null )\"
                    }
                  }
                \a" '

TERM=xterm-256color

```

```bash
source ~/.bashrc
```

### MacOS

```bash

prmptcmd() { eval "$PROMPT_COMMAND" }

precmd_functions=(prmptcmd)

```

```bash
source ~/.profile
source ~/.zshenv
```
