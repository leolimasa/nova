* To pass context down: have a `ctx` varaible always `untyped`, or type it if you want safety
* To pass context up: have a rec type with `{ val, ctx }` always `untyped`

Ex.:


```
log = (msg: String, logs: { content: List(String), to_stdout: Bool }) -> List(String) => {
    if logs.to_stdout {
        print(msg) 
    }
    logs.content += logs.content + [msg]
}

setup_log = (to_stdout: Bool) => {
    content: [],
    to_stdout
}

request_page = (url, ctx) => {
    ctx.logs = log(ctx.logs, "Requesting page")
    req = request(url)
    
    if req == Error(e) {
        ctx.logs = log(ctx.logs, "Error requestiog page: " + to_str(e)) 
        return { req, ctx }
    }
    
    if req = None {
        ctx = log(ctx, "Empty request")
        return { req, ctx }
    }
    
    to_str(req) 
}

main = () => {
    ctx = {
        logs: setup_log(False) 
    }
    { page1, ctx } = request_page("https://www.google.com", ctx)
    { page2, ctx } = request_page("https://www.yahoo.com", ctx)
    for_each(logs, (log) => print(log))
}

```
