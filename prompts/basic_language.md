This project is the implementation of a programming language compiler, and has two main files:

src/parser/ast.rs: defines the AST for the language
src/parser/parse.rs: defines the parser, which has functions that take a string and return an AST.

It uses the lalrpop library with a grammar defined in src/parser/grammar.lalrpop.

With that in mind, change ast.rs, parse.rs and the grammar so that it fulfills the following specifications:

- Expressions should support the following operators:
    * Algebraic, comparison and logic: `+`, `-`, `*`, `/`, `%`, `==`, `!=`, `>`, `>=`, `<`, `<=`, `and`, `or`, `not`
    * Dot operator: `point.x`
    * Pipe operator: `point |> map(x => x + 1)`
- Expressions should support the following literals:
    * Integers: `1`, `2`, `3`, ...
    * Floats: `1.0`, `2.0`, `3.0`, ...
    * Hex: `0x01`, `0x02`, `0x03`, ...
    * Strings: `"hello"`, `"world"`, ...
    * Template strings: `t"hello ${name}"`
    * Multiline strings: `"""hello"""`
    * Template multiline strings: `t"""hello ${name}"""`
- Function declarations should follow this general format: `(arg1 String, arg2 Int) -> String => arg1 + arg2`

To compile the project, you just have to run `cargo build`
