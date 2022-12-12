# Rust-Universal-Machine

Design Doc: https://docs.google.com/document/d/180tMh6rOxbS1wKEbjOogs6xyyTm2t2SH49-BSq6nmmM/edit?usp=sharing

## How to compile
`RUSTFLAGS='-C target-cpu=native' cargo build --release`

## How to run
`./target/release/rum ./umbinaries/midmark.um`

## Runtimes
10/11/20 No code refactoring, just "cheap tricks" such as target-cpu=native, some other compiler flags: 

-Midmark 0.42s
-Sandmark 11.74s

## Ideas for optimization
1. Compiler flags
2. B-tree
3. Static variables
4. Minimizing function calls
5. All code in one file