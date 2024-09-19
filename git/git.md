## git config

Set `name` and `email`
```bash
git config --global user.name "Your Name"
git config --global user.email "youremail@yourdomain.com"
```

```bash
[user]
	name = Your Name
	email = youremail@yourdomain.com
```

Set pager `delta`
```bash
[core]
    pager = delta

[interactive]
    diffFilter = delta --color-only

[delta]
    navigate = true    # use n and N to move between diff sections

    # delta detects terminal colors automatically; set one of these to disable auto-detection
    # dark = true
    # light = true

[merge]
    conflictstyle = diff3

[diff]
    colorMoved = default
```

Change editor
```bash
git config --global core.editor "nvim"
```

```bash
[core]
    editor = nvim
```

Change pager
```bash
git config --global core.pager delta
```

Reset author
```bash
git commit --amend --reset-author
```

RRR - Reuse Recorder Resolution
(Once enabled, `rerere` will remember each side of a conflict and apply the recorded resolution next time the same conflict reappears.)
```bash
git config --global rerere.enabled true
```

autostash
```bash
git config --global core.autostash true
git config --global core.autoSquash true

git log --oneline -S "embarassing" --walk-reflog --date=local | grep commit | less
```

Prune unreachable objects:
```bash
git reflog expire --expire=now --all
git gc --prune=now --aggressive
```

Remove file from repo
```bash
git filter-branch --tree-filter 'rm -f path/to/file' HEAD
```


### rebase strategy

Note that during git rebase and git pull --rebase, ours and theirs may appear swapped; 

--ours gives the version from the branch the changes are rebased onto, while

--theirs gives the version from the branch that holds your work that is being rebased.

This is because rebase is used in a workflow that treats the history at the remote as the shared canonical one, 
and treats the work done on the branch you are rebasing as the third-party work to be integrated, 
and you are temporarily assuming the role of the keeper of the canonical history during the rebase. 
As the keeper of the canonical history, you need to view the history from the remote as ours (i.e. "our shared canonical history"), 
while what you did on your side branch as theirs (i.e. "one contributor’s work on top of it").

Example 
```powershell
git rebase -Xours origin/main
```
`-Xours` will prioritize the changes from the `origin/main`, ignoring the changes from the `feat` branch in case of conflicts.

```powershell
git rebase -Xtheirs origin/main
```
`-Xtheirs` will prioritize the changes from the `feat` branch, ignoring the changes from the `origin/main` branch in case of conflicts.

