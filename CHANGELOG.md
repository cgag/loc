## 0.3.3
- support for including only files that match a regex (can be combined with exclude) (suggested by alekratz)
- output is now 80 characters instead of 81 (gross) (issue raised by chneukirchen)
- fixed support for multiple targets (issue raised by horacekmi, great catch)
- zackshuster fixed compilation on windows by removing a broken dependency.
- cypressious added instructions for running under the windows linux sub-system
- pthariensflame synced the supported languages in the readme and also made it prettier
- gebner added Lean support
- AaronePower gave credit where it's due to Tokei and ripgrep.
- AlexanderThaller added saltstack support
- sunbubble added CUDA support
- Switched to MIT license

## 0.3.2 (2016-10-25)
- Add a license file.
- dignati added additional extensions for clojure (.cljs, .cljc)
- pthariensflame fixed the display names for a bunch of languages
- mclehmen fixed my broken wording in the readme
- little-dude added TCL support
- glennpratt fixed things so github correctly detects this as a Rust project

## 0.3.1 (2016-10-25)
There was no 0.3.1. You saw nothing.

## 0.3.0 (2016-10-24)

- @svenstaro added OpenGL Shading Language (GLSL)

### Bugs
  - Fixed counting for python, which was extremely inaccurate due to an assumption that the
    start of comment character is different from the end of comment character, but we treat
    ''' as both starting and ending multiline comments for Python.
