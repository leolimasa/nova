# nova
Easy to write, easy to read, hard to break. A new programming language.

Loose thread: the bool test for expression is failing. Fix it.

## Language Canary

* [o] Minimum Viable Compiler: simple expressions, functions 
    * [O] Parser
        * [X] Move IR to be default implementation
        * [X] Add loc fields to AST
        * [ ] Change lalrpop to pass things by ref
        * [X] Change function declaration to function operator (=>)
        * [X] Move annotate_types to the typing module
        * [X] Remove implicit type casts for float and int
        * [X] Change expr to take type as a struct field
        * [X] Require type annotations for function arguments
        * [ ] Function calls
    * [ ] LLVM compiler
        * [ ] Expressions (literals and operators, except arrays, struct, and strings)
        * [ ] Block, with expression return
        * [ ] Function definition
        * [ ] Function calls
        * [ ] JIT compilation for tests
        * [ ] Native compilation
* [ ] remove wasm rust libs
* [ ] Integrate libc / musl
* [ ] Integrate boehm or another GC
* [ ] Strings
    * [ ] Parser
    * [ ] LLVM
* [ ] Assignment (minus arrays and structs) and expression variables
* [ ] If statement
* [ ] Arrays
    * [ ] Parser: array literals
    * [ ] Parser: array access
    * [ ] Parser: array assignment
    * [ ] LLVM: array literals
    * [ ] LLVM: array access
    * [ ] LLVM: array assignment
* [ ] Structs
    * [ ] Parser: struct declaration
    * [ ] Parser: struct access (dot operator)
    * [ ] Parser: struct assignment
    * [ ] LLVM: struct declaration
    * [ ] LLVM: struct access
    * [ ] LLVM: struct assignment

## Main language

* [ ] Write spec
* [ ] Parser
    * [ ] Comment support
    * [ ] Create special tokenizer for indentation

## Basic Types

* [ ] HM type inference
* [ ] Generics

## Advanced types

* [ ] Row types / structural subtyping
* [ ] Polymorphic variants
* [ ] Operator overloading based on row types
* [ ] Lexically scoped Algebraic effects using coroutines
* [ ] Affine / linear types to control mutability
* [ ] Multiple dispatch

## Future

* [ ] stdlib (hashmaps, lists, etc)
* [ ] repl
* [ ] WASM language interop
* [ ] package manager with WASM as common distribution for compiled sources

