rjit is a toy runtime assembler written in Rust.
By using this, you can test the assembly directly without linking process.

### restriction
* input is one assembly(.s) file.
* no libc support.
  * but I'm thinking of adding my own implementation for libc that doesn't need the linking process. (ex: printf, malloc, etc...)

### supported assembly syntax
wip...
- [ ] nop
- [ ] add
- [ ] sub
- [ ] mov

## ref
* [Intel SDM](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-instruction-set-reference-manual-325383.pdf)