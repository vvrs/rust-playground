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

* Handling Invalid Input
    * ```rust 
        <...statement...> => {
            ok(...) => ...,
            Err(...) => {
                ... Hanle ...
            }
        }
        ```
    For example, take only numbers as input
    
    ```rust 
        
        let mut input_num = String::new();
        io::stdin()
            .read_line(&mut input_num)
            .expect("Failed to read line");

        let input_num: u32 = match input_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };
    ```
