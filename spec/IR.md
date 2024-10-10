# Intermediate Representation (IR)

The intermediate representation is a simplified subset of the language meant to be easily translateable to compiler targets. It has a textual representation for easy debugging and testing. 
Unlike nova, the IR is imperative. All variables are considered mutable and garbage collected. If statements and blocks do not have return values.

Each statement is concluded by a newline. Statements cannot span multiple lines.

This is an informal specification. For the formal grammar, see [the IR grammar](../ir/grammar.lalrpop)
## Expressions

Expressions are the building blocks of the language. They can be literals, variables, function calls, or operators. All expressions have a return value.

### Operators

The usual algebraic, comparison, and logical operators are supported:

#### Algebraic Operators

* `+`
* `-`
* `*`
* `/`
* `%`

#### Comparison Operators

* `==`
* `!=`
* `>`
* `>=`
* `<`
* `<=`

#### Logical Operators

* and
* or
* not

#### Dot Operator

The dot operator is used to access fields of a struct. The variable contained the struct is followed by dot and then the field name. It is used as follows:

```
point.x
```

#### Function application

Functions are applied by writing the function name followed by the arguments in parentheses.

### Literals

Literals are values that are directly represented in the code. The following literals are supported:

* Integers: `1`, `2`, `3`, ...
* Booleans: `true`, `false`
* Strings: `"hello"`, `"world"`, ...
* Floating point: `1.0`, `2.0`, `3.0`, ...
* Arrays: `[1, 2, 3]`, `[true, false, true]`, ...
* Structs: `Point { x: 1, y: 2 }`, `Person { name: "Alice", age: 30 }`, ...

Strings are UTF-8 encoded. Arrays must have the same type for all elements. Structs are defined by the name of the struct and a list of field names and values.

## Blocks

Blocks are used to group statements together. They are delimited by curly braces. Blocks do not have a return value.

```
{
    x = 1
    y = 2
    return x + y
}
```

### Statements

Statements are commands executed within a block. Unlike expressions, they do not have a return value.

#### if

The `if` expression is used to conditionally execute a block of code.

#### else

#### else if

#### return and tail_return

`return` will stop the function evaluation. If it is provided with an expression, it will return the value of the expression as the result of the function application. If the return is a tail call, then the `tail_return` keyword should be used.

```
```

#### Expressions

Any expression can be a statement. The result of the expression is discarded.

#### Assignment

Assignments are used to bind a value to a variable. The values are always expressions.

```
```

## Functions

Functions can be named or anonymous, and are declared with the `fn` keyword. They can take arguments and return values. All arguments and return types need a type annotation.

### Declaration

Named functions are declared by a variable assignment. The body of the function is a block.

```
# Named function
add = fn (a: Int, b: Int): Int {
    return a + b
}

# Anonymous function
some_call(fn (a: Int, b: Int): Int { return a + b })
```

### Closures

If a function is a closure, it must be declared with the `fn_closure` keyword. Closures can capture variables from the outer scope.

```

```

## Types

### Primitive types

The IR supports the following primitive types:

* Integers: i32, i64,  
* Booleans: boolean
* Strings: string
* Floating point: f32, f64
* Arrays: array

### User defined types

Structs and enum are user defined types.

### Type annotations

All variables are expected to have a type annotation, both in function arguments and assignments.

## Modules

The entire program is represented in one tree, called a module. The IR does not support importing other modules. The module supports these top-level constructs:

* Assignments to literals
* User defined types
