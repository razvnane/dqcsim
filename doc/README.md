# Documentation source files

This directory contains the source files for the documentation. **You can find
the documentation for the latest version
[here](https://qe-lab.github.io/dqcsim/index.html).**

The following tools are used to generate the documentation:

 - The main documentation pages are generated by `mdbook` (`cargo install mdbook`).
 - The Rust reference is generated by Rust/Cargo.
 - The C reference is part of the `mdbook`-based documentation, using a custom
   Python script (in the `tools` dir) to preprocess the its markdown sources.
 - The Python reference is generated by `pdoc3` (`pip3 install pdoc3`).

A `Makefile` is provided to orchestrate the process:

 - `make [all]` will build the documentation.
 - `make open` will do the above and try to open the index in your browser.
 - `make clean` will clean the temporary files generated by this script. The
   files `cargo` generated are NOT cleaned; you can run `cargo clean` for that.

The root of the generated HTML is in the `target/book` folder of this
repository (so `../target/book` relative to the directory you're in now).
