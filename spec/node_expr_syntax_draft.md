# Nova syntax (list expr version)

### Operators

* Algebraic, comparison and logic: `+`, `-`, `*`, `/`, `%`, `==`, `!=`, `>`, `>=`, `<`, `<=`, `and`, `or`, `not`
* Dot operator: `point.x`
* Function operator: `=>`
* Function type operator: `->`
* Pipe operator: `point | map(x => x + 1)`

### Functions

Single expr:
 
```
arg1:String arg2:String -> String => arg1 + arg2 
```

Multi expr:

```
arg1:String arg2:String -> String => {
    x = 1
    y = 2
    x + y
}
```

### Function body

* Assignment: `x = 1`
* Accessor: `x(1)`
* Conditional: `if x > 0 x else -x`
* Conditional multiline:

```
    if x > 0 {
        x
    } else {
        -x
    }
```
* When:
```
when x {
    1 => True
    2 => False
    default => True
}
```

* Return: `return x`
* Function call: `f x`
* Partial application: `f x _`
* Last line is returned by default
