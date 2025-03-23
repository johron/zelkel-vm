# Zelkel Runtime
- Virtual machine runtime for the [Zelkel programming language](https://github.com/johron/zelkel)

## Documentation
- `@function:`: Defines a function, '@entry' is the entry point.
- `.label:`: Defines a label for a section of code
- `alc *buffer, size`: Allocates a buffer of the specified size.
- `fre *buffer`: Frees a buffer or variable.
- `psh value`: Pushes a value onto the stack.
- `len`: Pushes the length of the top item on the stack without popping it.
- `rot`: Rotates the top three items on the stack.
- `dup`: Duplicates the top item on the stack.
- `sys`: Executes a system call with the arguments on the stack.
- `pop $variable`: Pops the top item from the stack into a variable, names '\$' and '\$_' are ignored.
- `typ type`: Converts the top item on the stack to the specified type [str, int, float, bool].
- `sub`: Subtracts the top two items on the stack.
- `add`: Adds the top two items on the stack.
- `mul`: Multiplies the top two items on the stack.
- `div`: Divides the top two items on the stack.
- `mod`: Modulus of the top two items on the stack.
- `jmp .label`: Jumps to a label.
- `jnz .label`: Jumps to a label if the top item on the stack is not zero.
- `jzr .label`: Jumps to a label if the top item on the stack is zero.
- `run @function`: Run a function, requires ret to end it
- `cmp`: Compares the top two items on the stack.
- `ret`: Returns from a function.
- `<path:line:column>`: Defines a source location for debugging.

## License
Licensed under the MIT License; please see the [license file](LICENSE.md) for terms.
