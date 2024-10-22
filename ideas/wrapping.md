# Wrapping 

Wrapping literally wraps the remainder of the code under a continuation. This is the same as gleams `use` operator. Ex:


```
name = wrap validate_name("John") | throw_error
print(name)
```

is equivalent to:

```
 throw_error(validate_name("John"), (name) =>
     print(name)
 )
```

There can be multiple offset wrap sections within a block.


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


## Nested wraps

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
    
    wrap
       file
       lines: throw_error
    
```
