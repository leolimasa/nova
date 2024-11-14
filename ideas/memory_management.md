# Memory management

Initially i thought that we could just throw a garbage collector and call it a day. But I've been realizig that that would exclude the language from being used in performance critical applications, such as microcontrollers and games.

Turns out there is a lot of research in this area. The most fertile ground seems to be between the intersection between dependent types, linear types, and arena allocators. Therefore, I believe that nova's memory management strategy will depend highly on its type system.

The focus is then to create a language WITHOUT memory management first, and focus on type system features. Just let it leak. Then add an allocator on top of that.
