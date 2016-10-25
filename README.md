`loc` is a for counting lines of code. It's a rust implementation of [cloc](http://cloc.sourceforge.net/), but it's more than 100x faster. There's another rust line counting tool called [tokei](https://github.com/Aaronepower/tokei). loc is ~5-10x faster than tokei, though a bit less featureful.

I can count my 400k file `src` directory (thanks npm) in just under 7 seconds with loc, in a 1m14s with tokei, and I'm not even willing to try with cloc.

Counting just the dragonflybsd codebase (~9 million lines):
  - loc: 1.09 seconds
  - tokei: 5.3 seconds
  - cloc: 1 minute, 50 seconds

### Installation

There are binaries available on the releases page, thanks to the wonderful rust-everywhere project and travisci. For anyone familiar with Rust there's `cargo install loc`.
If you want to install Rust/Cargo, this is probably the easiest way: [https://www.rustup.rs/](https://www.rustup.rs/).

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
