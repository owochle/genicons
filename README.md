# Genicons
A simple and fast utility to generate manifests and favicons.

## Installation
Run `cargo install genicons` or build it from source.

## Usage
```
A quick utility to generate favicons

Usage: genicons [OPTIONS] <MASTER_IMAGE> <APP_NAME>

Arguments:
  <MASTER_IMAGE>  Your master image from which all favicons will be generated
  <APP_NAME>      Your app name

Options:
  -o, --output-dir <OUTPUT_DIR>  [default: ./favicons]
  -c, --no-html-copy             Do not copy html code in clipboard at the end of the program
      --short-name <SHORT_NAME>  Optional short name of your app
      --app-color <APP_COLOR>    Your app color [default: #FFFFFF]
      --start-url <START_URL>    Your app start url [default: .]
  -p, --path <PATH>              The path where the favicons will be hosted [default: /]
  -s, --silent                   Removes all prints (except on errors)
  -h, --help                     Print help
  -V, --version                  Print version
```

## License
MIT License Copyright © 2025 OwOchlé