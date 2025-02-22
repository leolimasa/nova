# Nova Spec Draft

## Syntax

### Operators

* Algebraic, comparison and logic: `+`, `-`, `*`, `/`, `%`, `==`, `!=`, `>`, `>=`, `<`, `<=`, `and`, `or`, `not`
* Dot operator: `point.x`
* Pipe operator: `point | map(x => x + 1)`

### Functions

* Declaration: `(arg1 String, arg2 Int) -> String => arg1 + arg2`
    * Types are optional:  `(arg1, arg2) -> => arg1 + arg2`
* Body
    * Assignment: `x = 1`
    * Accessor: `x[1]`
    * Conditional: `if x > 0 then x else -x`
    * When: 
      
      ```
      when x
        is 1 => True
        is 2 => False
        default => True
      ```
    * Return: `return x`
    * Function call: `f(x)`
    * Partial application: `f(x, _)`
    * Last line is returned by default
* `use` keyword for callbacks

### Annotations

```
Person = {
    name String @json("person_name")
    age Int @json("person_age")
}
```

### Records

* Type declaration: `{ x Int, y Int }`
* Instantiation: `{ x: 1, y: 2 }`
* Access: `point.x`
* Spreading: `{ x: 1, y: 2, ...point }`

### Pattern matching

TODO

### Literals

* Integers: `1`, `2`, `3`, ...
* Floats: `1.0`, `2.0`, `3.0`, ...
* Hex: `0x01`, `0x02`, `0x03`, ...
* Strings: `"hello"`, `"world"`, ...
* Template strings: `t"hello ${name}"`
* Multiline strings: `"""hello"""`
* Template multiline strings: `t"""hello ${name}"""`

### Open functions (multiple dispatch)


```
add = open (x a, y b) -> a
add += (a Int, b Int) -> Int -> int.sum(a,b)
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
* Exports TODO

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
    * Record types: `record`
    * Union types: `oneof`
    * `(a) -> x` (function)
    * `Map`
    * `Channel`

### Generics

TODO

### GADT

TODO

### Mutability

TODO

### Single-use types (linear types)


### Polymorphic records (row polymorphism)

TODO

### Polymorphic unions (polymorphic variants)

TODO

### Type constraints (dependent types)

TODO

### Type inference

TODO

## Macros

* `comptime` keyword

## Compiler

* Compile to Go and Js
* Import go/js libs:
    * `import go math/rand`
        * Go types are automatically detected by the compiler
        * All go types are mutable by default
    * `import js crypto { randomUUID (Int, Int) -> Int }`
        * JS needs type definitions

## Standard lib

* `os`
* `io`
* `string`
* `convert`
* `time`
* `math`
* `concur`: concurrency

