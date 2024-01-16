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

### git config

general

```
git config --global user.name "Your Name"
```

```
git config --global user.email "youremail@yourdomain.com"
```

```
git pager = `delta`
```

```
git config --global core.editor "nvim"
```

```
git config --global core.pager delta
```

```
git commit --amend --reset-author
```

RRR - Reuse Recorder Resolution
Once enabled, `rerere` will remember each side of a conflict and apply the recorded resolution next time the same conflict reappears.

```
git config rerere.enabled true
```

```
git config --global core.autostash true
git config --global core.autoSquash true

git log --oneline -S "embarassing" --walk-reflog --date=local | grep commit | less
```

REMOVE FILE FROM REPO
```
git filter-branch --tree-filter 'rm -f path/to/file' HEAD
```

%userprofile%\.gitconfig
~\.gitconfig

### git alias

```
[alias]
    i           = init
    br          = branch --format='%(HEAD) %(color:yellow)%(refname:short)%(color:reset) - %(contents:subject) %(color:green)(%(committerdate:relative)) [%(authorname)]' --sort=-committerdate
    br2         = branch --all --format='%(HEAD) %(color:yellow)%(refname:short)%(color:reset) - %(contents:subject) %(color:green)(%(committerdate:relative)) [%(authorname)]' 
    bra         = br --all
    changed     = whatchanged -n 1
    changes     = whatchanged -n 1
    cfg         = config --edit --global
    cg          = config --edit --global
    ci          = commit
    cie         = ci --allow-empty
    co          = checkout
    com         = checkout master
    compare     = diff --name-status
    comparetree = "!f() { git compare $1 | gtree;  }; f"
    coo         = checkout -
    cow         = checkout work
    fe          = fetch
    getbr       = "!f() { git co -b temp_Branch_D0nV1; git co $1; git res temp_Branch_D0nV1; git br -D temp_Branch_D0nV1; }; f"
    gettag      = "!f() { git tag -d $1; git tag $1; }; f"
    tagg        = "!f() { git tag -d $1; git tag $1; }; f"
    goto        = "!f() { git co -b temp_Branch_D0nV1; git reset --soft $1; git co $1; git br -D temp_Branch_D0nV1; }; f"
    gr          = log --color --graph --all --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset'
    ; gr          = "!f() { git-graph; }; f"
    grdate      = log --color --graph --all --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%ad) %C(bold blue)<%an>%Creset' --date=format:'%Y-%m-%d %H:%M:%S'
    l1          = log1
    last        = log -1 HEAD --stat
    lg          = log --graph --pretty=tformat:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr, %cs) %C(bold blue)<%an>%Creset' --abbrev-commit --decorate=full
    ll          = log --pretty=tformat:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr, %cs) %C(bold blue)<%an>%Creset' --abbrev-commit --decorate=full
    lla         = log --color --graph --all --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset' --abbrev-commit --
    ll1         = ll -n 1
    log1        = log -n 1
    move        = "!f() { git co -b temp_Branch_D0nV1; git reset --soft $1; git co $1; git br -D temp_Branch_D0nV1; }; f"
    purge       = reflog expire --expire-unreachable=now --all
    ref         = log --reflog --pretty=format:"%h%x09%an%x09%ad%x09%s"
    ref2        = log --reflog --oneline
    rem         = remote -v
    res         = reset --hard
    res1        = reset HEAD~1 --hard
    ress        = reset --soft
    root        = rev-parse --show-toplevel
    sf          = show --name-only
    st          = status
    staa        = stash apply
    stad        = stash drop
    stal        = stash list
    stapo       = stash pop
    stapu       = stash push
    tree        = ls-tree -r master --name-only

    reso  = "!f() { branch_name=$(git rev-parse --abbrev-ref HEAD); git reset --hard origin/$branch_name;  }; f"
    comm  = "!f() { git com; git reset --hard origin/master;  }; f"
    fem   = "!f() { git fe; git comm; git coo;  }; f"
    upm   = "!f() { git comm; git coo;  }; f"

    wt    = "!f() { git worktree $1 $2; }; f"

    ; new  = "!f() { branch_name=$(git rev-parse --abbrev-ref HEAD); git ll origin/$branch_name..HEAD;  }; f"
    ; missing  = "!f() { branch_name=$(git rev-parse --abbrev-ref HEAD); git ll HEAD..origin/$branch_name;  }; f"
    new = ll origin/develop..HEAD
    newm = ll origin/master..HEAD
    missing = ll HEAD..origin/develop
    missingm = ll HEAD..origin/master

    iter = ci -am "iter"
    cii  = ci -am "iter"
 
    am  = ci -a --am

    customcmd = "!f() { git ci .github/workflows/backend.yml -m \"testing actions $1\" && git push; }; f"
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
