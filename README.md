`loc` is a for counting lines of code. It's a rust implementation of [cloc](http://cloc.sourceforge.net/).  It's more 100x faster than cloc. There's another rust line counting tool called [tokei](https://github.com/Aaronepower/tokei). loc is ~5-10x faster than tokei, though a bit less featureful.

I can count my 400k file `src` directory (thanks node) in just under 7 seconds with loc, in a 1m14s with tokei, and it's completely untenable with cloc.

### Installation

I've added a binary for your average non-musl linux system to the releases page. For anyone familiar with Rust there's `cargo install loc`.

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
