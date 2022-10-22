# reindent

A small tool for reindenting files. Useful if you don't want to pull in (or heck, implement!)
heavyweight formatters and instead keep everything nice and simple.

## Usage

```sh
# Reindent input.txt to 4 spaces
$ reindent -T4 input.txt
# Reindent input.txt to tabs
$ reindent -Ttab input.txt
# Reindent input.txt to 3 spaces, hinting that existing indentation uses tabs
$ reindent -Ftab -T3 input.txt
# Once you're happy with the results, you can tell reindent to perform the operation in-place:
$ reindent -Ftab -T3 input.txt -i
```

## Compiling

```sh
$ cargo build --release
# or, to install to PATH:
$ cargo install --path .
```
