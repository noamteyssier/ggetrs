# Autocomplete

This is used to generate autocomplete information for your terminal shell.

## Arguments

| Name | Short | Long | Description |
|------|-------|------|-------------|
| Shell | `-s` | `--shell` | Shell to generate autocompletions for [possible values: bash, elvish, fish, powershell, zsh]|

## Command Line Interface

```bash
# generate autocompletions for the fish shell
ggetrs autocomplete -s fish

# write autocomplete directly to fish shell config
ggetrs autocomplete -s fish > ~/.config/fish/completions/ggetrs.fish
```
