Ideally, function syntax would allow specifying a multitude of types, specially named ones. A few alternatives:

```python

set_name = 
    fn
        # The object to be named
        obj: Nameable { 
          name: String, 
          age: Int where age > 0 
        }

        # The name to be applied
        name: String

        # Whether to check if the name is valid
        name_check: Boolean

        -> Nameable | Error(InvalidName)
    =>
    
    obj.name = name
    obj
```

Function calls can specify the field name:

```python
set_name obj name="John" name_check=True
```

Function calls with no params can be called with `()`:

```python
set_name ()
```

If a field has a None variant, or a default value, it is optional:

```python
set_name = 
    fn
        obj: Nameable { 
            name: String, 
            age: Int where age > 0 
        }

        name: String = "John"
        name_check: Boolean | None
        age: Int = 0

        -> Nameable | InvalidName
    =>
  
    obj.name = name
    obj.age = age
    if name_check == True and check_name(name) == False
        return Error(InvalidName)
    obj
```

Functions can be defined on a single line:

```python
set_name = fn obj, name => obj.name = name; obj
```

Empty parameters are denoted by `()`:

```python
say_hello = fn () => "Hello"
```

Python style \*\*args is defined by having the last argument be an array:

```python
sum = fn nums: Int[] -> Int => nums.reduce (a, b) => a + b
```
