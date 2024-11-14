# Product unions

Note: if we go with multiple dispatch, a lot of this is moot.

Normally, we'd have tagged unions and product types declared separately. Ex:

```
SomeUnionType = A | B | C

SomeProductType = { a: int, b: string }
```

Product unions treat **all** types as tagged unions with an optional product type atteched to it. It is attached with the & operator. Ex:

```
SomeType = A | B & { a: int, b: string }
```

The above means that SomeType will always have `a` and `b` fields, but it can be either A or B. This allows dynamic dispatch to work transparently by adding fields to an existing variable:

```
implement_name_print = v => v + { print = (self) => print(v.name) }

main = () =>
    employee = { name: "John", age: 30 }
    employee = implement_name_print(employee)
    employee.print()
```

`implement_name_print` has type:

```
implement_name_print: { name: String } -> { name: String, print: () -> Nothing }
```

Primitive types are also tagged unions and can be used as such:

```
SomeType = Int | String
```

So the following is valid:

```
validate_name = (name) =>
    if name != ""
        return name 
    else
        return Error("Name cannot be empty")
```

which has type:

```
validate_name: String => String | Error<String>
```

And could be used as:

```
name = validate_name("John")

if Error(e) = name
    # throw the error
    return Error(e)

// name is a string and the type system knows that. Every if statement narrows down the type.
print(name)
```

Or, alternitively, with wrap:

```
throw_error = (v, continue) =>
    if Error(e) = v
        Error(e)
    else
        continue(v)

main = () =>
    name = wrap validate_name("John") | throw_error
    
    # name is a string
    print(name)
```

With offset wrap:

```
main = () =>
    john = validate_name("John")
    chris = validate_name("Chris")
    steven = validate_name("Steven")
    print(name)
    
    wrap 
        john, chris: throw_error
        steven: (v, continue) => 
            if Error(e) = v  
                continue("just an example of a different handler") 
            else 
                continue(v)
```

There can be multiple offset wrap sections within a block.

## Multiple wraps

Here is an example of a case with nested wraps:

```
read_file = (path: String, continue: (FileHandler) -> a): a | Error<ReadFileError> => ...

main = () =>
    read_file("file.txt", 
        (file) => 
            throw_error(File.read_lines(file), 
                (lines) => 
                    print(lines)))
        
# same as

main = () =>
    file = read_file("file.txt")
    lines = File.read_lines(file)
    print(lines)
    
    wrap:
       file
       lines: throw_error
    
```
