# Terminal cock

## Install

```
cargo install --locked --git https://github.com/immanelg/tui-cock
```

```
cargo install --locked tui-cock
```

## Run
```
tui-cock 20
```

The argument is the size of the cock. It is probably between 10 and 69, should be easy to try different values to see what looks good for your terminal.

## Zellij
The cock looks nice in a zellij layout, e.g.

layout {
    pane command="btm" size="65%"
    pane split_direction="vertical" {
        pane {
            command "gping"
            args "192.168.50.1" "8.8.8.8" "1.1.1.1" "-n" "1" "-b" "300"
        }
        pane size="20%" {
            command "tui-cock"
            args "20"
        }
    }
}
