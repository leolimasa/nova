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

#### Function call 

Functions are called by writing the function name followed by the arguments in parentheses. There is no support for currying or partial application.

#### Literals

Literals are values that are directly represented in the code. The following literals are supported:

* Integers: `1`, `2`, `3`, ...
* Booleans: `true`, `false`
* Strings: `"hello"`, `"world"`, ...
* Floating point: `1.0`, `2.0`, `3.0`, ...
* Arrays: `[1, 2, 3]`, `[true, false, true]`, ...
* Structs: `Point { x: 1, y: 2 }`, `Person { name: "Alice", age: 30 }`, ...

Strings are UTF-8 encoded. Arrays must have the same type for all elements. Structs are defined by the name of the struct and a list of field names and values.

#### Identifiers 

Identifiers are names that can refer to a variable, extern, or function.

#### Array access

Arrays are accessed by using the square brackets. The array is followed by the index in square brackets. Example:

```
x = [1, 2, 3]
x[0]
```

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

The `if` expression is used to conditionally execute a block of code. The `else` is optional and behaves as one would expect. There is no else if.

```
if (condition) {
    # code
} else {
    # code
}
```

#### return and tail_return

`return` will stop the function evaluation. If it is provided with an expression, it will return the value of the expression as the result of the function application. If the return is a tail call, then the `tail_return` keyword should be used.

#### Expressions

Any expression can be a statement. The result of the expression is discarded.

#### Assignment

Assignments are used to bind a value to a variable. The values are always expressions. The variable name is followed by `=` and the expression.

```
x = some_function_call() + 10
```

##### Array assignments

Array assignments are done with the square brackets. The array is followed by the index in square brackets, then the assignment operator, and the expression. Example:

```
x = [1, 2, 3]
x[0] = 10
```

##### Struct assignments

Struct assignments are done with the dot operator. The variable containing the struct is followed by the field name, then the assignment operator, and the expression. Example:

```
point.x = 10
```

## Functions

All functions are named. There is no support for anonymous functions. All functions must be declared at the module level. Closures are not supported.
The `fn` keyword is used, followed by the function name, arguments, return type, and the function body. Example:

```
fn add (a: i32, b: i32): i32 {
    return a + b
}
```

## Types

### Primitive types

The IR supports the following primitive types:

* Integers: i32, i64,  
* Booleans: boolean
* Strings: string
* Floating point: f32, f64
* Arrays: array<T>

### Structs

Defined with the `struct` keyword, followed by the name. The fields are defined by the field name and the type, each on a separate line. Example:

```
struct Point {
    x: i32
    y: i32
}
```

### Arrays

Are initalized by assignment with a list of values. The type must be specified. Example:

```
x : array<i32> = [1, 2, 3]
```

### Type annotations

All variables are expected to have a type annotation, both in function arguments, assignments, and function return types.

## Modules

The entire program is represented in one tree, called a module. The IR does not support importing other modules. The module supports these top-level constructs:

* Assignments to literals
* User defined types
* Functions

### Extern and Export 

The `extern` keyword is used to declare a function that is implemented in another language. The `export` keyword is used to declare a function that can be called from another language. Example:

```
extern fn print(s: string)
export fn add(a: i32, b: i32): i32
```
