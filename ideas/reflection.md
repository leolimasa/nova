All types are subtypes of Any, but Any **is not** a subtype of any type.
To convert **Any** to an usable type, one must call `any_to_type(T)`, where T is a type. `any_to_type(T)` has a result type that returns `Err.IncorrectType` if the type cannot be converted.

Any type:

```
Any = oneof
    Int(Int)
    Float(Float)
    String(String)
    Boolean(Boolean)
    Array(Array(Any))
    Record(Option(String) Map(String Any))
    OneOf(Option(String) String Map(String Any))
```

```

# Serialize json
to_json = fn x: Any -> Result(String Err(_)) =>
    fields_to_str = fn f => 
        map(fields(f), fn k v => k + ":" + to_json(v))
        | join(",")
        
    when x is
        Any.Int(i) => to_string(i)
        Any.Float(f) => to_string(i)
        Any.String(s) => s
        Any.Boolean(b) => to_string(b)
        Any.Record(name, r) =>
            "{"
            + fields_to_str(f)
            + "}"
        Any.OneOf(name, tag, f) => 
            "{tag: " + tag 
            + ", fields: "
            + fields_to_str(f)
            + "}"
        _ => Err(TypeNotSupported)
        

# Deserialize json
from_json = fn x: Json -> Any =>
    when x is
        Json.Int(i) => i
        Json.Float(f) => f 
        Json.String(s) => s
        Json.Record(r) => r
        Json.OneOf(o) => o
        
        
main = fn =>
    person = http.get("/person")
        | json_response
        | from_json
        | any_to_type(Person)
    
    if Err(e) = person then
        print("Error! " + e)
        
     

```
