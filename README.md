# nova
Easy to write, easy to read, hard to break, and runs everywhere.

Loose thread: started to write the block compiler. Will need to infer the types.

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
    * [.] LLVM compiler
        * [X] Expressions (literals and operators, except arrays, struct, and strings)
        * [ ] Block, with expression return
        * [ ] Function definition
        * [ ] Function calls
        * [ ] JIT compilation for tests
        * [ ] Native compilation
* [X] remove wasm rust libs
* [ ] Integrate libc / musl
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
* [ ] Records
    * [ ] Parser: struct declaration
    * [ ] Parser: struct access (dot operator)
    * [ ] Parser: struct assignment
    * [ ] LLVM: struct declaration
    * [ ] LLVM: struct access
    * [ ] LLVM: struct assignment

## Basic Types

* [ ] HM type inference and checking
* [ ] Generics

## Advanced types

* [ ] Row types / structural subtyping
* [ ] Polymorphic variants
* [ ] Multiple dispatch (by having a single function have multiple defs)
* [ ] (maybe) Dependent types
* [ ] Affine / linear types to control mutability
* [ ] (maybe) Lexically scoped Algebraic effects using coroutines

## Syntatic sugar

* [ ] Implicit env passing
* [ ] Implicit return passing
* [ ] `use` notation
* [ ] Pattern matching
* [ ] List comprehensions
* [ ] Destructuring

## Memory management

Evaluate how to leverage the type system to come up with a memory management strategy.


## Final lang spec

Given all the practical experience gained so far, write down the final language specification. Note what needs to change from the current implementation.

## Future

* [ ] stdlib (hashmaps, lists, etc)
* [ ] repl
* [ ] javascript
* [ ] WASM language interop
* [ ] package manager with WASM as common distribution for compiled sources

