# 10fastprogramme.rs

a typing speed trainer based on monkeytype built with Rust and EGui;

pick a language and type generated code snippets that follow its syntax

## avaiable languages

```
rust, python, javascript, go, C
```

you can also make your own language and put it inside the program's folder, following the json syntax.

example:
```json
{
  "identifiers": [
    "value", "result", "index"
  ],
  "templates": [
    "int {id} = 0;",
    "int {id} = {id};",
    "char *{id} = NULL;",
  ]
}
```

## build

requires the Rust toolchain
on Linux you may also need:

```bash
# arch
sudo pacman -S libxcb libxkbcommon

# debian/ubuntu
sudo apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev
```
then

```bash
cargo run --release
```

## controls

| key | action |
|-----|--------|
| esc | back to menu |
| tab | restart test |

