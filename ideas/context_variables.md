## Definition

Context variables are variables that are transparently passed through the scope of different functions. They can be input or output. Useful for passing data between functions without having to add a new argument to every function in the call stack, or having to change the return signature.


## Input context

They are declared with the `context` keyword. Example:

```

print_value_context = fn () =>
    context in value
    print(value)
   
do_print = fn () =>
    print_value_context()
    
main = fn () =>
    context in value = "Hello, world!"
    do_print() # Prints "Hellow, world!"
```

Context can be overriden by changing the stack value:

```

print_value_context = fn () =>
    context value
    print(value)
   
do_print = fn () =>
    context in value = "Goodbye, world!"
    print_value_context()
    
main = fn () =>
    context in value = "Hello, world!"
    do_print() # Prints "Goodbye, world!"

```

When a function declares an input context, a new `ctx` variable is added as an argument. The variable name is a field in `ctx`. Due to row polymorphism, this variable can now be declared safely in all functions that call a context function and passed down accordingly.

## Output context

When declared, the function is transparently converted to a tuple with the first element being the return value and the second element being the context. Example:


```
import std context

log = fn (msg) =>
    context out logs = [ Log(msg) ]

add = fn (a, b) =>
    log("Adding " + a + " and " + b)
    a + b
    
LogList = type List(Log(String))
    
context.join = ..context.join fn (a: LogList, b: LogList) =>
    a + b
    
main = fn () =>
    context out logs
    
    # Join is called behind the scenes to join the context out of
    # both add functions and store it in `logs`
    print(add(1, 2)) # Prints 3
    print(add(3, 4)) # Prints 7
    
    print(logs) # Prints [ Log("Adding 1 and 2"), Log("Adding 3 and 4") ]
```

## Continuations with output context

The `use` keyword can be used in combination with an out context to control continuations

```


```






