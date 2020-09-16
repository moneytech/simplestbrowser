<p align="center"><img src="https://github.com/skylinecc/simplestbrowser/blob/master/data/logo.png?raw=true" width="200" height="200"></p>
<h1 align="center">simplestbrowser</h1>
<p align="center">A simple implementation of webkitGTK in simple Rust. Compiles to a 238KB self-contained binary.</p>
<p align="center"><img src="https://github.com/skylinecc/simplestbrowser/blob/master/data/simplestbrowser.png?raw=true"></p>

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
