`<c-p>` to go to project
```bash
# ~/.zshrc

# Function to select a directory and change to it
fzf-change-directory() {
    local sand
    local doutorado
    local project
    local predifined_folders
    local command

    # Find directories using fd
    sand=$(fd -t d -d 3 -a --base-directory ~/Sandbox/)
    doutorado=$(fd -t d -d 1 -a --base-directory ~/Documents/doutorado.git/)

    # Predefined directories
    predefined_folders="
$HOME/Documents/doutorado.git
$HOME/.config/nvim
$HOME/Sandbox"

    # Combine directories and use fzf for selection
    project=$(printf "%s\n%s" "$sand" "$doutorado" "${predefined_folders[@]}" | fzf --tac)

    # If a directory is selected, change to it
    if [[ -n $project ]]; then
        command="z \"$project\""
        
        # Revert the current line and insert the command
        zle -f # Force a refresh if needed
        BUFFER="$command"  # Set the command to the buffer
        zle -f # Refresh line editor
        zle accept-line # Execute the command
    fi
}

# Register the function as a widget
zle -N fzf-change-directory

# Bind Ctrl+p to the widget
bindkey '^p' fzf-change-directory

# Optional: Enable auto-completion if not already enabled
autoload -U compinit
compinit
```
