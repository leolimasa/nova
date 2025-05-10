# Macros

Macros are functions where the first argument is an AST and it returns another AST or compile error.

```
select = (b: AST) -> oneof(AST, Error(String)) => {
    if b != ast.Block {
        return Error("select must be a block") 
    }
    selects = map_error(b.children, item => {
        if item != ast.Arrow {
            return Error("Invalid syntax for select statement.")
        }
        ...
    })
    if selects == Error {
        return selects
    }
}
```
