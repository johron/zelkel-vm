# Zelkel Runtime
- Virtual machine runtime for Zelkel programming language

## Instructions
- `add` - Add two values on the stack
- `sub` - Subtract two values on the stack
- `mul` - Multiply two values on the stack
- `div` - Divide two values on the stack
- `mod` - Modulo two values on the stack
- `cmp` - Compare two values on the stack
- `dup` - Duplicate the top value on the stack
- `pop` - Pop a value off the stack
- `psh` - Push a value onto the stack
- `rot` - Rotate the top three values on the stack
- `prt` - Print the top value on the stack
- `inp` - Read a value from the user and push it onto the stack
- `jmp` - Jump to a specific label
- `jnz` - Jump to a specific label if the top value on the stack is not zero
- `jzr` - Jump to a specific label if the top value on the stack is zero
- `typ` - Convert the top value on the stack from a string to a different type

- sys
- 

ret burde ikke stoppe programmet, det er return for funksjon

endre sjekking for .entry section sånn at det heller jumper til .entry label i evaluator.rs. Fjerne sjekking i parser for .entry label, eller kanskje ha sånn 
at han passer på at ved slutten av parsing så har han en .entry label. må legge til funksjoner som jeg kan kjøre med cal

## License
Licensed under the MIT License; please see the [license file](LICENSE.md) for terms.
