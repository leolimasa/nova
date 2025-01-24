* All comma delimited lists can be replaced with a block

Ex.:

fn (a, b)

can be replaced with

fn
  a
  b
  
record (a: Int, b: Int)

can be replaced with

record
  a: Int
  b: Int
  
oneof (a: Int, b: Int)

can be replaced with

oneof
  a: Int
  b: Int
 
* New line is the same as a comma

my_arr = [1, 2, 3]
my_arr = [
    1
    2
    3
]

my_rec = {a = 1, b = 2}

my_rec = {
    a = 1
    b = 2
}

* Destructuring, for both types and values, always start with ..

Ex.:

record
  a: Int
  b: Int
  ..SomeType
  
oneof (a: Int, b: Int, ..SomeOtherType)

my_record = {
    a = 1
    b = 2
    ..some_var
}

* `:` are always for type annotations, and `=` are always for assignment
* `=>` means a block of code follows
* `{}` is always a record
* `[]` is always a list
