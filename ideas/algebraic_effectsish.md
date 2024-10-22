A function would have two return types: the main return type, and an effect type.

Ex:

```
read_file: (path: String) -> String / Error<ReadFileError>
```

If a function has an effect type, it will automatically be wrapped when called.

Ex:

```
print_file_contents = (path) =>
    file = read_file(path)
    print(file)
```

is equivalent to

```
print_file_contents = (path) =>
    read_file(path, (file) =>
        print(file))
```

Becase read_file returns an effect type, the type inference will infer the parent function as also having that effect type.

Effects can be thrown. When they are, they interrupt the control flow of the calling function.

```
validate_name = (name) =>
    if name != ""
        name 
    else
        throw Error("Name cannot be empty")
```

and they are caught with `catch`, via a handler function:

```
main = () =>
    name = validate_name("John")
    print(name)
    
    catch
        name: (e, continue) => print(e) 
```

The above is equivalent to:

```
validate_name = (name, continue): Nothing | Error(String) =>
    if name != ""
        continue(name)
    else
        Error("Name cannot be empty")
        
main = () =>
    handler = (v, continue) => 
        if Error(e) = v
            print(e)
        else
            continue(v)
            
    rest = (name) => print(name)
    handler(validate_name("John", rest), rest)
```
