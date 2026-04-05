-- Pull in the wezterm API
local wezterm = require("wezterm")

-- This will hold the configuration.
local config = wezterm.config_builder()

-- This is where you actually apply your config choices.

-- For example, changing the initial geometry for new windows:
config.initial_cols = 120
config.initial_rows = 28

-- Set the font family

config.font = wezterm.font("JetBrainsMono Nerd Font")

config.color_schemes = {
    ["MyTheme"] = {
        foreground = "#CBE0F0",
        background = "#1e1e1e",
        cursor_bg = "#47FF9C",
        cursor_border = "#47FF9C",
        cursor_fg = "#011423",
        selection_bg = "#033259",
        selection_fg = "#CBE0F0",
        ansi = { "#214969", "#E52E2E", "#44FFB1", "#FFE073", "#834ec3", "#a277ff", "#24EAF7", "#24EAF7" },
        brights = { "#4597db", "#E52E2E", "#44FFB1", "#FFE073", "#A277FF", "#a277ff", "#24EAF7", "#24EAF7" },
    },
}

config.font_size = 16

config.color_scheme = "MyTheme"

config.window_decorations = "TITLE | RESIZE"

-- default terminal
config.default_prog = { "pwsh.exe" }

-- config.enable_tab_bar = false
config.keys = {
    -- 'new tab' with alt+t
    {
        key = "x",
        mods = "ALT",
        action = wezterm.action.CloseCurrentPane({ confirm = true }),
    },
    {
        key = "|",
        mods = "CTRL|SHIFT",
        action = wezterm.action.SplitHorizontal({ domain = "CurrentPaneDomain" }),
    },
    {
        key = "|",
        mods = "ALT|SHIFT",
        action = wezterm.action.SplitVertical({ domain = "CurrentPaneDomain" }),
    },

    -- Move to pane on the Right
    { key = "l", mods = "CTRL", action = wezterm.action.ActivatePaneDirection("Right") },
    -- Move to pane on the Left
    { key = "h", mods = "CTRL", action = wezterm.action.ActivatePaneDirection("Left") },
    -- Move to pane Above
    { key = "k", mods = "CTRL", action = wezterm.action.ActivatePaneDirection("Up") },
    -- Move to pane Below
    { key = "j", mods = "CTRL", action = wezterm.action.ActivatePaneDirection("Down") },
}

-- Finally, return the configuration to wezterm:
return config
