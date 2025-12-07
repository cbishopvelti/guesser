## Introduction

This is a simple tool to ask a llm to try and figure out your next command based on historic commands.

## Setup

requires protobuf
```
brew install protobuf
```

### .bashrc

Add to ~/.bashrc:

```bash
_rust_gusser() {
    # CHANGE THIS to the absolute path of your Cargo.toml
    local MANIFEST="/home/chris/playground/guesser/Cargo.toml"

    # We use 'cargo run -q' instead of the binary directly.
    # '--manifest-path' lets us run this from anywhere in the system.
    # '< /dev/tty' forces cargo to accept keyboard input even though we capture stdout.
    local selected=$(history -a && cargo run -q --manifest-path "$MANIFEST" --release -- "$READLINE_LINE" < /dev/tty)

    if [ -n "$selected" ]; then
        READLINE_LINE="$selected"
        READLINE_POINT=${#selected}
    fi
}

bind -x '"\C-g": _rust_gusser'
```

## lancedb data viewer
```
docker run --rm -p 8080:8080 \
  -v ~/home/chris/playground/guesser/data/dev:/data:ro \
  ghcr.io/gordonmurray/lance-data-viewer:lancedb-0.24.3
```

### .zshrc

```zsh
_rust_gusser_widget() {
    # CHANGE THIS to the absolute path of your Cargo.toml
    local MANIFEST="/Users/chris/playground/guesser/Cargo.toml"

    fc -AI
    local selected=$(cargo run -q --manifest-path "$MANIFEST" -- "$READLINE_LINE" < /dev/tty)
    # local selected=$(guesser < /dev/tty)

    if [ -n "$selected" ]; then
        BUFFER="$selected"
        CURSOR=${#selected}
    fi
    zle reset-prompt
}

zle -N _rust_gusser_widget
bindkey '^g' _rust_gusser_widget

export OPENAI_BASE_URL="http://192.168.2.227:1234/v1"
setopt INC_APPEND_HISTORY
```

## build
```bash
cargo build --bins --release
```
