# Open functions

A function declared as `open` can have alternative definitions in different files. The particular implementation will be decided based on the types of the arguments. This is an implementation of multiple dispatch.

Ex.:

operators.nova

```python

pub add = open fn (a: Int, b: Int) -> Int => a + b
```

```python
import file operators add 

# When extending an open function, the variable name must be fully qualified
# with the module name.
operators.add = ..add fn (a:Float, b:Float) -> Float => a + b

main = fn =>
    # Those are all legal since they all have been defined
    add(1, 2)
    add(1.0, 2.0)
```

Multiple implementations of the same function can be declared one after the other:

```python
add = 
    fn (a: Int, b: Int) -> Int => a + b
    fn (a: Float, b: Float) -> Float => a + b
```
