# render-viewer
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Simple rust program to create png image and then show it on screen.

Well now png rendering is declared via json and supports adding images (png with alpha), rendering text (limited length and fixed font size) and geometric primitives like lines and polygons.

And for some reason FLTK shows image poorly on my Win10 machine...

# requirements

Rust and CMake.

# to run

First render image

```
cargo run
```

Then start viewer part:

```
cargo run viewer
```