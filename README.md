# CLI Shortener

A simple CLI tool to create local shortened links using a web server

# How to build executable locally

1. Install [Rust](https://www.rust-lang.org/tools/install) on your device.
2. Run ```cargo build -r``` to create the executable
    - The newly-created executable will be located at ```./target/release/``` and will be called cli_shortener with the appropriate extension

# Help

```
Usage: cli_shortener <COMMAND>

Commands:
  new     Create a new shortened link
  delete  Delete a shortened link
  clear   Deletes all existing shortened links
  list    Lists all active shortened links
  start   Starts the web server which redirects the shortened links
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```