The biggest obstacle to compile to Go is the lack of TCO. Turns out we may be able to avoid ridiculously large stack frames by unrolling into a loop, and, if that fails, using trampolines. 

See the dragon book and SICP on how to detect a tail call.

## Detect tail call (from AI studio - double check)

Last Operation: The call to the function (let's call it F) must be the very last operation performed in the calling function (let's call it G) before G returns. There can be no operations performed on the return value of F after it returns to G.

Direct Return: G must return the value returned by F directly. No modifications, calculations, or any other operations can be applied to the returned value before G returns it.

No Pending State: There should be no local variables or other state in G that are needed after the call to F returns. All necessary state should be passed as arguments to F.

## Convert to loop

Update Arguments: Replace the current arguments of G with the arguments of F.

Jump to Start: Jump to the beginning of the function G. This effectively turns the recursive call into a loop iteration.
