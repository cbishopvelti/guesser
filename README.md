## Introduction

This is a simple tool to ask a llm to try and figure out your next command based on historic commands.

## Setup

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
