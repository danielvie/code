### git alias

```bash
[alias]
    ; BRANCHES
    br          = branch --format='%(HEAD) %(color:yellow)%(refname:short)%(color:reset) - %(contents:subject) %(color:green)(%(committerdate:relative)) [%(authorname)]' --sort=-committerdate
    br2         = branch --all --format='%(HEAD) %(color:yellow)%(refname:short)%(color:reset) - %(contents:subject) %(color:green)(%(committerdate:relative)) [%(authorname)]' 
    bra         = br --all

    ; CONFIG
    cfg         = config --edit --global
    cg          = config --edit --global

    ; COMMITS
    ci          = commit
    cie         = ci --allow-empty
    cii         = ci -am "iter"
    co          = checkout
    com         = checkout main
    coo         = checkout -
    cow         = checkout work
    iter        = ci -am "iter"
    sf          = show --name-only

    ; STATUS
    changed     = whatchanged -n 1
    changes     = whatchanged -n 1
    compare     = diff --name-status
    comparetree = "!f() { git compare $1 | gtree;  }; f"
    getbr       = "!f() { git co -b temp_Branch_D0nV1; git co $1; git res temp_Branch_D0nV1; git br -D temp_Branch_D0nV1; }; f"
    gettag      = "!f() { git tag -d $1; git tag $1; }; f"
    li          = ll -n 1
    ll          = log --pretty=tformat:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr, %cs) %C(bold blue)<%an>%Creset' --abbrev-commit --decorate=full
    lla         = log --color --graph --all --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset' --abbrev-commit --
    new         = ll origin/main..HEAD
    last        = log -1 HEAD --stat
    missing     = ll HEAD..origin/main
    ref         = log --reflog --pretty=format:"%h%x09%an%x09%ad%x09%s"
    ref2        = log --reflog --oneline
    tree        = ls-tree -r main --name-only
    st          = status

    ; GRAPH
    gr          = log --color --graph --all --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset'
    grdate      = log --color --graph --all --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%ad) %C(bold blue)<%an>%Creset' --date=format:'%Y-%m-%d %H:%M:%S'

    ; REPO
    fe          = fetch
    obliterate  = reflog expire --expire-unreachable=now --all
    purge       = reflog expire --expire-unreachable=now --all
    rebo        = "!f() { branch_name=$(git rev-parse --abbrev-ref HEAD); git rebase origin/$branch_name;  }; f"
    rem         = remote -v
    res         = reset --hard
    res1        = reset HEAD~1 --hard
    reso        = "!f() { branch_name=$(git rev-parse --abbrev-ref HEAD); git reset --hard origin/$branch_name;  }; f"
    ress        = reset --soft
    root        = rev-parse --show-toplevel

    ; WORKTREE
    wt          = "!f() { git worktree $1 $2; }; f"

    ; SUBMODULES
    smu = submodule update --init --recursive
    sms = submodule status
    sma = submodule add

    ; PLAY
    customcmd = "!f() { git ci .github/workflows/backend.yml -m \"testing actions $1\" && git push; }; f"
    

```
