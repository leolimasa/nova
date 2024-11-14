# Implicit handlers

Implicit handlers are callbacks that are transparently passed between functions. They allow functions to be defined by a parent caller. It is an implementation of algebraic effects.

Handlers are declared as a single struct named handlers and passed as an argument.

```



read_file: (path: String) -> String / Error<ReadFileError>

Handlers = type {
    read_file: (path) -> String
}

print_file_contents = (handlers, path) =>
    contents = handlers.read_file(path)
    
    if Abort(a) = contents
        return Abort(a)
        
    print(contents)
    
    
main = () => 
    handlers = {
        read_file: (path) => 
            h = fopen(path)
            if h == -1 
                Abort(Error())
    }
```

A handler can be passed


