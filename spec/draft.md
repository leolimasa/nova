# Nova Spec Draft

## Syntax

### Operators

* Algebraic, comparison and logic: `+`, `-`, `*`, `/`, `%`, `==`, `!=`, `>`, `>=`, `<`, `<=`, `and`, `or`, `not`
* Dot operator: `point.x`
* Pipe operator: `point |> map(x => x + 1)`

### Literals

* Integers: `1`, `2`, `3`, ...
* Floats: `1.0`, `2.0`, `3.0`, ...
* Hex: `0x01`, `0x02`, `0x03`, ...
* Strings: `"hello"`, `"world"`, ...
* Template strings: `t"hello ${name}"`
* Multiline strings: `"""hello"""`
* Template multiline strings: `t"""hello ${name}"""`

### Functions

* Declaration: `(arg1 String, arg2 Int) -> String => arg1 + arg2`
* Types are optional:  `(arg1, arg2) => arg1 + arg2`
* Type parameters with `<>`: `<Type1,Type2>(arg1, arg2) => ...`

### Expressions

These are valid in an expression:

* Variables
* Literals
* Operators
* Function declaration
* Parenthesis grouping
* Function call: `f(x)`
* Partial application: `f(x, _)`
* Accessor: `x[0]`
* Conditional: `if x > 0 then x else -x`
* Blocks
* When: 
  
  ```
  when x
    is 1 => True
    is 2 => False
    default => True
  ```
* `use`: wraps continuations automatically
  

### Blocks

* Delimited by indentation 
    * WHY: using indentation means we can keep using brackets for objects, so there is no ambiguity. 
* Is an expression
* Last statement is the value of the expression
* Assignment: `x = 1`
* Accessor assignment: `x[1] = 0`
* Dot assignment: `x.a = 0`
* Return: `return x`
* Expressions

### Annotations

```
Person = type {
    name String @json("person_name")
    age Int @json("person_age")
}
```

### Commas multiline

Terms that are separated by commas (ex: func args, imports, oneof...) can also be declared on multiple lines without the commas:

```
some_fn = (arg1, arg2) => ...

# equivalent to

some_fn =
    arg1
    arg2
=>
```

```
Bool = type oneof (True, False)

# equivalent to

Bool = type oneof
    True
    False
```

```
import std (io, concurrent concur)

# equivalent to

import std
    io
    concurrent concur
```
### Comments

* Use hashtags for comments

### Records

* Type declaration: `{ x Int, y Int }`
* Instantiation: `{ x: 1, y: 2 }`
* Access: `point.x`
* Spreading: `{ x: 1, y: 2, ...point }`

### Pattern matching

TODO


### Open functions (multiple dispatch)


```
add = open (x a, y b) -> a
add += (a Int, b Int) -> Int => int.sum(a,b)
```

### Operator overloading

* Algebraic: TODO
* Comparison: TODO
* Assignment: TODO
* Accessor: TODO
* Pipe: TODO

### Module

* Imports
    * File: `import file ./file.nova`
    * Assign to variable: `import file ./file.nova f`
    * Import variables: `import file ./file.nova f { open, close }`
* Declarations TODO
* Pub declarations and imports TODO

## Type System

* Primitives
    * `Bool`
    * `Int`, `Int8`, `Int16`, `Int32`, `Int64`
    * `UInt`, `UInt8`, `UInt16`, `UInt32`, `UInt64`
    * `Float`, `Float32`, `Float64`
    * `Complex64`, `Complex128`
    * `String`
* Composite
    * `Array(x)`
    * Record types: `{}`
    * Union types: `oneof`
    * `(a) -> x` (function)
    * `Map`
    * `Channel`

Whether to use tagged unions/rows: will need to implement a simpler version of the language with traditional tagged unions, and then implement row polymorphism on top of that. Then evaluate how to layout the type system.

### Type expressions 

* Specify the type of an identifier anywhere in the code

```
# Union
oneof (Red, Green, Blue)

# Record
{
    name String
    favorite_color Rgb
}

# Function
(a String, b String) -> Int
```

### Generics

* Type parameters/generics are specified with angled brackets `< >`

```
Error = type <a> 
```

### Mutability

* Mutable vars: "mut x = { a: 1, b: 2 }"
* Mutable vars cannot be used in type constraints

### Single-use types (linear types)


### Polymorphic records (row polymorphism)

TODO

### Polymorphic unions (polymorphic variants)

TODO

### Type constraints (dependent types)

TODO

### Type inference

TODO

### Effects

One-shot continuations based on if-statements. See [example file](../examples/algebraic_fx.nova)
## Macros

* `comptime` keyword

## Compiler

* Compile to Go and TS (at first. Then probably Python and C.)
    * When compiling to Go, can use interfaces to support row polymorphism
    * If a function has only one instance, then don't use interface
* Arithmetic operators on primitive types are optimized to NOT use multiple dispatch
* Import go/js libs:
    * `import extern math/rand { randomUUID (Int, Int) -> Int }`
        * All go types are mutable by default
        * Types are mandatory
    * A utility automatically reads go and typescript code and generates the types as separate files
* Tail call optimization by converting to while loop or trampolines
* Generics/row polymorphism are implemented by multiple definitions with differing type signatures (function instantiation)
* Compiles into a procedural IR first, then to go/js

## Standard lib

* `os`
* `io`
* `net`
* `path`
* `string`
* `convert`
* `time`
* `math`
* `test`
* `bitwise`
* `concur`: concurrency
* `op`: operators

## Package management

* Import lib from url: `pub import lib https://somelib.com somelib `
* Specify permissions: `pub import lib https://somelib.com somelib grant io, os, extern`
* Store a lib.lock file
* Libs are used by importing a lib.nova file: `import file ./lib `

TODO

## LSP

* Automcomplete
* Go to definition
* Find references
* Rename symbol
* Diagnostics
* Hover
* Inline type signature inference
