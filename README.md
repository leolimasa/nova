# nova
Easy to write, easy to read, hard to break. A new programming language.

Loose thread: the bool test for expression is failing. Fix it.

## IR

* [X] Write spec
* [o] Minimum Viable Compiler: simple expressions, functions, export, extern
  * [X] Parser
    * [X] Expressions (literals (except arrays, struct, strings)) + operators (except dot))
    * [X] Block (expression + return)
    * [X] Function declaration
    * [X] Export
    * [X] Module 
  * [.] WASM compiler
    * [X] Expressions (number + operators)
    * [ ] Block
    * [ ] Functions
    * [ ] Export
    * [ ] Module
* [ ] Strings
    * [ ] Parser
    * [ ] WASM
* [ ] Assignment (minus arrays and structs) and expression variables
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
* [ ] Bool expr short circuiting
* [ ] Remove int to float autocast

## Main language

* [ ] Write spec
* [ ] Parser

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

* Heading
* Vertical Speed
* Altitude
* Speed
* Radio
* Map zoom
