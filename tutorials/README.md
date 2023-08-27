#### Notes


##### Project Setup
* Compile single file

    `rustc <file_name>`

* Cargo project setup:

    Create a library: `cargo new <project_name>`
    Build: `cargo build`
    Run a program: `cargo run`
    Check for issues: `cargo check`

##### Programming

* Create variables
    `let var_name  = ...`

    Variables are immutable by default, so add `mut` for make them mutable
    `let mut var_name = ...`

* To use the other libaries 
    `use <library>::<...>` -> equivalent to `import ` in python or `#include ` C++

    for example `use std::io` for IO
