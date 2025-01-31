# Terminal clock

![image](https://github.com/user-attachments/assets/5b437d77-1dbd-46fb-9937-8ffbc51dff55)

## Install
`cargo install --locked --git https://github.com/samoylovfp/tui-clock`
## Run
`clock 0.5`

The argument is the aspect ratio of the font glyph.
It is probably between 0.4 and 0.6, should be easy to try
different values to see what looks good for your terminal.

## Zellij

The clock looks nice in a zellij layout, e.g.

![screenshot](scr.png)

```
layout {
    pane command="btm" size="65%"
    pane split_direction="vertical" {
        pane {
            command "gping"
            args "192.168.50.1" "8.8.8.8" "1.1.1.1" "-n" "1" "-b" "300"
        }
        pane size="20%" {
            command "clock"
            args "0.45"
        }
    }
}
```
