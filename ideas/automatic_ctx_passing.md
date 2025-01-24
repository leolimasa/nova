# Automatic ctx passing

* Function may or may not have an `ctx` argument
* If an `ctx` argument is present, then calling functions will automatically fill it with the current `ctx` variable in scope.
* If an `ctx` argument is not present, but there are functions calls that have `ctx` arguments in the function body, an `ctx` argument is automatically created.
* If an `ctx` argument is not present, and there are variables declared ctx in the function body, an `ctx` argument is automatically created.

Because of row types, parent ctxs will automatically have fields of the children.

This allows state to be passed transparently between functions. It also enables multiple dispatch cleanly. See [multiple_dispatch](multiple_dispatch.md).

Note: I think the multiple dispatch portion of this is a bad idea. Though I forgot the reason why right now... Write here when you remember.

## Example

```python

setup_ctx = fn => 
    {
        operators: {
            add:
                fn a: Foo, b: Foo -> Foo => sum_foos(a, b) 
                fn a: Bar, b: Bar -> Foo => sum_bars(a, b)
                ..ctx.operators.add
            ..ctx.operators
        }
        ..ctx
    }

main = fn () =>
    ctx = setup_ctx()
    sum_foos()

sum_foos = fn () =>
    foo1 = Foo()
    foo2 = Foo()
    foo1 + foo2
```

Is equivalent to:

``` python
setup_ctx = fn (ctx) => 
    {
        operators: {
            add:
                fn a: Foo, b: Foo -> Foo => sum_foos(a, b) 
                fn a: Bar, b: Bar -> Foo => sum_bars(a, b)
                ..ctx.operators.add
            ..ctx.operators
        }
        ..ctx
    }

main = fn (ctx) =>
    ctx = setup_ctx(ctx)
    sum_foos(ctx)

sum_foos = fn (ctx) =>
    foo1 = Foo()
    foo2 = Foo()
    ctx.operators.add(foo1, foo2)
``` 

## ctx default fields

Some example of possible default ctx fields:

* ctx.proc:
    * args: array of command line arguments
    * pwd: current working directory
* ctx.os:
    * env:
        * set()
        * get()
    * user:
        * uid()
        * gid()
        * groups()
    * file
        * open
        * read
        * write
        * copy
    * dir
        * list
        * create
        * delete
    * process
        * 
