# nova
Easy to write, easy to read, hard to break. A new programming language.

Loose thread: while writing the test_arithmetic_ops, it looks like the order of operations for mult and add is wrong. Check that. Also come up with helper functions to generate ast enums a bit easier.

## IR

* [X] Write spec
* [.] Minimum Viable Compiler: simple expressions, functions, export, extern
  * [.] Parser
    * [X] Expressions (literals (except arrays, struct, strings)) + operators (except dot))
    * [ ] Block (expression + return)
    * [ ] Function declaration
    * [ ] Export
    * [ ] Module 
  * [ ] WASM compiler
    * [ ] Expressions (number + operators)
    * [ ] Block
    * [ ] Functions
    * [ ] Export
    * [ ] Module
* [ ] Strings
    * [ ] Parser
    * [ ] WASM
* [ ] Assignment (minus arrays and structs) and Identifiers
    * [ ] Parser: assignment expression
    * [ ] WASM
* [ ] Function calls
    * [ ] Parser: function call expression
    * [ ] WASM
* [ ] If statement
    * [ ] parser
    * [ ] WASM
* [ ] Arrays
    * [ ] Parser: array literals
    * [ ] Parser: array access
    * [ ] Parser: array assignment
    * [ ] WASM: array literals
    * [ ] WASM: array access
    * [ ] WASM: array assignment
* [ ] Structs
    * [ ] Parser: struct declaration
    * [ ] Parser: struct access (dot operator)
    * [ ] Parser: struct assignment
    * [ ] WASM: struct declaration
    * [ ] WASM: struct access
    * [ ] WASM: struct assignment

## Main language

* [ ] Write spec

## Basic Types

* [ ] HM type inference
* [ ] Generics
* [ ] Row types
* [ ] Polymorphic variants

## Advanced types

* [ ] Lexically scoped Algebraic effects
* [ ] `handle` keyword
* [ ] Overloading, maybe?

## Future

* [ ] stdlib (hashmaps, lists, etc)
* [ ] executable compiler (using cranelift or wasmtime somehow)

