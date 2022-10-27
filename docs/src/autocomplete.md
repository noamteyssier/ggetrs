# Autocomplete

This is used to generate autocomplete information for your terminal shell.

## Help

```text
Set up autocomplete for various shells

Usage: ggetrs autocomplete --shell <SHELL>

Options:
  -s, --shell <SHELL>  Shell to generate autocompletions for [possible values: bash, elvish, fish, powershell, zsh]
  -h, --help           Print help information
  -V, --version        Print version information
```

## Usage

```bash
# generate autocompletions for the fish shell
ggetrs autocomplete -s fish

# write autocomplete directly to fish shell config
ggetrs autocomplete -s fish > ~/.config/fish/completions/ggetrs.fish
```
