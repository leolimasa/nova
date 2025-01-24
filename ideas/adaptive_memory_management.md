Adaptive memory management chooses how to manage memory depending on how variables are used within the program.

* Variable never leaves current thread and:
    * is used in a single function: use stack
    * is never moved outside its original ownership: use stack and clean on last function that uses it
    * is moved outside its original ownership: use refcount
    * has cycles: use garbage collector
* Variable leaves current thread:
    * use refcount or garbage collector depending on cycles
    * if mutable
        * variable MUST be marked as **linear** so that it can be moved between threads safely
