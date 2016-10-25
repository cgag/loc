## 0.3.0 (2016-10-24)

- @svenstaro added OpenGL Shading Language (GLSL)

### Bugs
  - Fixed counting for python, which was extremely inaccurate due to an assumption that the
    start of comment character is different from the end of comment character, but we treat
    ''' as both starting and ending multiline comments for Python.
