### terminal utils

CLEAR terminal

linux
```
alias cls='printf "\033c"'
```

macos:
```
alias cls='printf "\e[H\e[2J\e[3J"'
```

Eza

```
brew install eza
```

```
alias ls='eza --color=always --long --git --no-filesize --icons=always --no-time --no-user --no-permissions'
```

FZF in terminal

```
brew install fzf
```

config ~/.zshrc
```bash
# Set up fzf key bindings and fuzzy completion
eval "$(fzf --zsh)"
export FZF_CTRL_T_OPTS="--preview 'bat -n --color=always --line-range :500 {}'"
export FZF_ALT_C_OPTS="--preview 'eza -tree --color=always {} | head -200'"

# Advanced customization of fzf options via _fzf_comprun function
# - The first argument to the function is the name of the command.
# - You should make sure to pass the rest of the arguments to fzf.
_fzf_comprun() {
  local command=$1
  shift

  case "$command" in
    cd)           fzf --preview 'eza --tree --color=always {} | head -200' "$@" ;;
    export|unset) fzf --preview "eval 'echo \$'{}"         "$@" ;;
    ssh)          fzf --preview 'dig {}'                   "$@" ;;
    *)            fzf --preview "bat -n --color=always --line-range :500 {}" "$@" ;;
  esac
}
```

Zoxide

```
brew install zoxide
```

in `.zshrc`

```
# -- config zoxide
eval "$(zoxide init zsh)"
alias cd="z"
```

### Compilation C++

`.clangd` example
```
CompileFlags:
  Add: [ 
    '-std=c++23',
    '-I/Users/danielvieira/Sandbox/cpp/cpp_expected/src',
  ]
```

### export methods C++
```cpp
extern "C" __declspec(dllexport) int func(int a, int b) {...}
```

### custom terminal
starship
