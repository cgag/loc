2018-03-08:
I saw a bunch of stars pop up and thought I should mention that tokei is smarter and more accurate so please give that a look and see if there are any wild discrepancies (mostly for your benefit but please let me know if so).  Tokei is linked below but it's also rust so `cargo install tokei` is all you need.  Also these benchmarks are quite old. I doubt cloc has changed but tokei probably has. 

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

`loc` should now compile on Windows, but you can also run it under Windows using linux emulation:

> You can run `loc` on Windows 10 Anniversary Update build 14393 or later using the [Windows Subsystem for Linux](https://msdn.microsoft.com/de-de/commandline/wsl/install_guide?f=255&MSPPError=-2147217396). Simply download the Linux distribution from the [releases page](https://github.com/cgag/loc/releases), and run it in `bash` using a WSL-compatible path (e.g. `/mnt/c/Users/Foo/Repo/` instead of `C:\Users\Foo\Repo`).

### Usage

By default, `loc` will count lines of code in a target directory:

``` shell
$ loc
--------------------------------------------------------------------------------
 Language             Files        Lines        Blank      Comment         Code
--------------------------------------------------------------------------------
 Lua                      2       387088        24193       193544       169351
 Rust                     4         1172          111           31         1030
 C                        4          700           75          155          470
 Markdown                 2          249           39            0          210
 Bourne Shell             4          228           41           27          160
 Ada                      2           53           12            9           32
 Toml                     1           26            4            2           20
 Gherkin                  1           12            2            2            8
 OCaml                    1           13            4            6            3
 Ruby                     1            4            0            2            2
 Handlebars               1            4            0            2            2
--------------------------------------------------------------------------------
 Total                   23       389549        24481       193780       171288
--------------------------------------------------------------------------------

```

You can also pass one or many targets for it to inspect

``` shell
$ loc ci benches
--------------------------------------------------------------------------------
 Language             Files        Lines        Blank      Comment         Code
--------------------------------------------------------------------------------
 Bourne Shell             4          228           41           27          160
 Rust                     1           17            4            0           13
--------------------------------------------------------------------------------
 Total                    5          245           45           27          173
--------------------------------------------------------------------------------
```

To see stats for *each file* parsed, pass the `--files` flag:

```sh
$ loc --files src
--------------------------------------------------------------------------------
 Language             Files        Lines        Blank      Comment         Code
--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
 Rust                     2         1028           88           29          911
--------------------------------------------------------------------------------
|src/lib.rs                         677           54           19          604
|src/main.rs                        351           34           10          307
```

By default, the columns will be sorted by `Code` counted in descending order. You can select a different column to sort
using the `--sort` flag:

``` shell
$ loc --files --sort Comment ci
--------------------------------------------------------------------------------
 Language             Files        Lines        Blank      Comment         Code
--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
 Bourne Shell             4          228           41           27          160
--------------------------------------------------------------------------------
|ci/before_deploy.sh                 68           15           13           40
|ci/install.sh                       60           13            6           41
|ci/script.sh                        41            8            8           25
|ci/utils.sh                         59            5            0           54

```

`loc` can also be called with regexes to match and/or exclude files.

``` shell
$ loc --include 'count'
--------------------------------------------------------------------------------
 Language             Files        Lines        Blank      Comment         Code
--------------------------------------------------------------------------------
 Rust                     2          144           23            2          119
--------------------------------------------------------------------------------
 Total                    2          144           23            2          119
```

``` shell
loc --exclude 'sh$'
--------------------------------------------------------------------------------
 Language             Files        Lines        Blank      Comment         Code
--------------------------------------------------------------------------------
 Lua                      2       387088        24193       193544       169351
 Rust                     4         1172          111           31         1030
 C                        4          700           75          155          470
 Markdown                 2          275           38            0          237
 Ada                      2           53           12            9           32
 Toml                     1           26            4            2           20
 Gherkin                  1           12            2            2            8
 OCaml                    1           13            4            6            3
 Handlebars               1            4            0            2            2
 Ruby                     1            4            0            2            2
--------------------------------------------------------------------------------
 Total                   19       389347        24439       193753       171155
--------------------------------------------------------------------------------
```


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

Ignored and hidden files:

By default, loc respects .gitignore/.ignore files, and ignores hidden files and directories.  You can count disregard
ignore files with `loc -u`, and include hidden files/dirs with `loc -uu`.

### Supported Languages

- ActionScript
- Ada
- Agda
- ASP
- ASP.NET
- Assembly
- Autoconf
- Awk
- Batch
- Bourne Shell
- C
- C Shell
- C/C++ Header
- C#
- C++
- Clojure
- CoffeeScript
- ColdFusion
- ColdFusionScript
- Coq
- CSS
- CUDA
- CUDA Header
- D
- Dart
- DeviceTree
- Erlang
- Forth
- FORTRAN Legacy
- FORTRAN Modern
- F# (Fsharp)
- GLSL
- Go
- Groovy
- Handlebars
- Haskell
- Hex
- HTML
- Idris
- INI
- Intel Hex
- Isabelle
- Jai
- Java
- JavaScript
- JSON
- Jsx
- Julia
- Kotlin
- Lean
- Less
- LinkerScript
- Lisp
- Lua
- Make
- Makefile
- Markdown
- Mustache
- Nim
- Nix
- Objective-C
- Objective-C++
- OCaml
- OpenCL
- Oz
- Pascal
- Perl
- PHP
- Plain Text
- Polly
- Prolog
- Protobuf
- Pyret
- Python
- Qcl
- QML
- R
- Razor
- reStructuredText
- Ruby
- RubyHtml
- Rust
- SaltStack
- Sass
- Scala
- SML
- SQL
- Stylus
- Swift
- Tcl
- Terraform
- TeX
- Toml
- TypeScript
- Tsx
- UnrealScript
- VimL
- Wolfram
- XML
- Yacc
- YAML
- Zig
- Z Shell

## Attributions

This project contains code from [Tokei](https://github.com/Aaronepower/tokei) by [Aaronepower](https://github.com/Aaronepower) and [ripgrep](https://github.com/BurntSushi/ripgrep) by [BurntSushi](https://github.com/BurntSushi).

### Contributors
