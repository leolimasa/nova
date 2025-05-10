# List expressions

Nova expressions are derived from lisp S-Expressions. It's meant the be easy to parse but without excessive parenthesis.


Lists are defined by declaring list **items**:

```
item1 item2 item3
```

Items can be:

* **List**: grouped of items using parenthesis or brackets
* Integer
* Float
* Hex
* String
* Identifiers

## Parenthesis

Parenthesis are used to declare sublists:

```
nodeName (grandchild1 prop1) (grandchild2 prop2) (grandchild3 prop3)
```

## Brackets

Brackets declare list of lists where each item is a single line. It's effectively sugar for `(())`. Child declarations can be continued on the same line after the closing bracket. Example:

```
child1 child2 {
    grandchild1 prop1
    grandchild2 prop2
    grandchild3 prop3
}
```

Example:

```
if a > b {
    fun_call a
} else {
    fun_call b
}

```


## Operators

Operators are desugared to nodes with the name of the operator, and the children will always be the left and right nodes. Operators are parsed AFTER the initial AST is created. Example:

```
if a > b {
    fun_call a
}
```

is desugared to:

```
if (> a b) {
    fun_call a
} else { 
    fun_call b
}
```

or, as an AST:

```
- ident: if
- list:
    - ident: >
    - ident: a
    - ident: b
- list:
    - list:
        - ident: fun_call
        - ident: a
    - ident: else
    - list:
        - ident: fun_call
        - ident: b


```

Operators are:

```
+, -, *, /, %, ==, !=, >, >=, <, <=, ., :, |, and, or, not
```

## Examples

### Function

```
fn a b {
    a + b
}
```
