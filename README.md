# nova
Easy to write, easy to read, hard to break. A new programming language.

Loose thread: the bool test for expression is failing. Fix it.

## Language Canary

* [ ] Minimum Viable Compiler: simple expressions, functions 
    * [ ] Parser
        * [ ] Move IR to be default implementation
        * [ ] Add loc fields to AST
        * [ ] Change function declaration to function operator (=>)
        * [ ] Remove implicit type casts for float and int
        * [ ] Move annotate_types to the typing module
        * [ ] Change expr to take type as a struct field
        * [ ] Require type annotations for function arguments
        * [ ] Function calls
    * [ ] LLVM compiler
        * [ ] Expressions (literals and operators, except arrays, struct, and strings)
        * [ ] Block, with expression return
        * [ ] Function definition
        * [ ] Function calls
        * [ ] JIT compilation for tests
        * [ ] Native compilation
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

* [ ] Row types
* [ ] Operator overloading based on row types
* [ ] Polymorphic variants
* [ ] Lexically scoped Algebraic effects using coroutines

## Future

* [ ] stdlib (hashmaps, lists, etc)
* [ ] repl
* [ ] WASM language interop
* [ ] package manager with WASM as common distribution for compiled sources

