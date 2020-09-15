# simplestbrowser
A simple implementation of webkitGTK in ~68 Rust SLOC. Compiles to a 3.1 MB self-contained binary. 
![simplestbrowser](https://github.com/skylinecc/simplestbrowser/blob/master/data/simplestbrowser.png?raw=true)

## Using
Download the latest binary from the [releases](https://github.com/skylinecc/simplestbrowser/releases) page, or build the project from scratch.

## Building
Install dependencies:
```
# apt install libwebkit2gtk-4.0-dev
```
Build the project:
```
$ cargo build --release
```
Run the project:
```
$ ./target/debug/simplestbrowser
```
