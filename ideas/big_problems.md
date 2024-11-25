The current big problems in programming languages are:

* Safe mutability: How to protect variables shared between threads in an efficient way without cognitive burden
    * actor model
    * affine types for mutable variables only
    * substructural type systems
* Easy effects: How to handle type safe side effects without introducing too much boilerplate
    * algebraic effects 
* Expression problem: How to make programs that are at the same time expressive and safe
    * multiple dispatch
    * I think multiple dispatch can be implemented with something like algebraic effects
* Memory management: How to make memory management safe and efficient and runnable on constrained devices
    * linear types for mutability
    * automated lifetime tracking
