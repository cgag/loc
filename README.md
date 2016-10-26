`loc` is a tool for counting lines of code. It's a rust implementation of [cloc](http://cloc.sourceforge.net/), but it's more than 100x faster. There's another rust code counting tool called [tokei](https://github.com/Aaronepower/tokei), loc is ~2-10x faster than tokei, depending on how many files are being counted.

I can count my 400k file `src` directory (thanks npm) in just under 7 seconds with loc, in a 1m14s with tokei, and I'm not even willing to try with cloc.

Counting just the dragonflybsd codebase (~9 million lines):
  - loc: 1.09 seconds
  - tokei: 5.3 seconds
  - cloc: 1 minute, 50 seconds

### Installation

There are binaries available on the [releases page](https://github.com/cgag/loc/releases), thanks to the wonderful rust-everywhere project and travisci. For anyone familiar with Rust there's `cargo install loc`.
If you want to install Rust/Cargo, this is probably the easiest way: [https://www.rustup.rs/](https://www.rustup.rs/).

#### Windows

You can run `loc` on Windows 10 Anniversary Update build 14393 or later using the [Windows Subsystem for Linux](https://msdn.microsoft.com/de-de/commandline/wsl/install_guide?f=255&MSPPError=-2147217396). Simply download the Linux distribution from the [releases page](https://github.com/cgag/loc/releases), and run it in `bash` using a WSL-compatible path (e.g. `/mnt/c/Users/Foo/Repo/` instead of `C:\Users\Foo\Repo`).

### Known Issues
Fortran has a rule that comments must start with the first character of a line. I only check if it's the first non-whitespace character of a line. I don't know
how often this is a problem in real code.  I would think not often.

Comments inside string literals: You can get incorrect counts if your code has something like this:

```
x = "/* I haven't slept \ 
for 10 days \
because that would be too long \
*/";
```

loc counts the first line and last lines correctly as code, but the middle
lines will be incorrectly counted as comments.

### Supported Languages

```
ActionScript
Ada
Asp
AspNet
Assembly
Autoconf
Awk
Batch
BourneShell
C
CCppHeader
CSharp
CShell
Clojure
CoffeeScript
ColdFusion
ColdFusionScript
Coq
Cpp
Css
CUDA
D
Dart
DeviceTree
Erlang
Forth
FortranLegacy
FortranModern
GLSL
Go
Handlebars
Haskell
Hex
Html
INI
Idris
IntelHex
Isabelle
Jai
Java
JavaScript
Json
Jsx
Julia
Kotlin
Less
LinkerScript
Lisp
Lua
Make
Makefile
Markdown
Mustache
Nim
OCaml
ObjectiveC
ObjectiveCpp
Oz
Pascal
Perl
Php
Polly
Prolog
Protobuf
Python
Qcl
R
Razor
ReStructuredText
Ruby
RubyHtml
Rust
Sass
Scala
Sml
Sql
Swift
Tex
Text
Toml
TypeScript
UnrealScript
VimScript
Wolfram
XML
Yacc
Yaml
Zsh
```

## Attributions

This project contains code from [Tokei](https://github.com/Aaronepower/tokei) by [Aaronepower](https://github.com/Aaronepower) and [ripgrep](https://github.com/BurntSushi/ripgrep) by [BurntSushi](https://github.com/BurntSushi).
