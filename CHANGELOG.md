## 0.3.2 (2016-10-25)
- Add a license file.
- dignati added additional extensions for clojure (.cljs, .cljc)
- pthariensflame fixed the display names for a bunch of languages
- mclehmen fixed my broken wording in the readme
- little-dude added TCL support

## 0.3.1 (2016-10-25)
There was no 0.3.1. You saw nothing.

## 0.3.0 (2016-10-24)

- @svenstaro added OpenGL Shading Language (GLSL)

### Bugs
  - Fixed counting for python, which was extremely inaccurate due to an assumption that the
    start of comment character is different from the end of comment character, but we treat
    ''' as both starting and ending multiline comments for Python.
