# reindent

A small tool for reindenting files. Useful if you don't want to pull in (or heck, implement!)
heavyweight formatters and instead keep everything nice and simple.

(Right now) reindent **does not support** fixing broken indentation in files with mixed
spaces and tabs, because it assumes one type of indentation per file. It's only for changing the
indentation of correctly indented files.

## Usage

```sh
# Reindent input.txt to 4 spaces
$ reindent -T4 input.txt
# Reindent input.txt to tabs
$ reindent -Ttab input.txt
# Reindent input.txt to 3 spaces, hinting that existing indentation uses tabs
$ reindent -Ftab -T3 input.txt
# Once you're happy with the results, you can tell reindent to perform the operation in place:
$ reindent -Ftab -T3 input.txt -i
```

By default, reindent does a dry run to stdout so that you can see how everything will look.
Once you're ready to perform the operation on the actual files, pass `-i` to modify them in place.

reindent will output logs to stderr letting you know the progress of the operation, which files
were affected, which files could not be read, and most importantly, the detected indent levels for
each one. If you want to see only those stats, you can redirect stdout to `/dev/null`.

## Compiling

```sh
$ cargo build --release
# or, to install to PATH:
$ cargo install --path .
```
