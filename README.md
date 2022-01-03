rjit is a toy runtime assembler written in Rust.
I created this for learning x86 machine-code and concept of runtime compile, ex: JIT.

### current plan
* rjit reads x86 assembly as a input.
* While running rjit, it reads each function of asm and tranlate it to x86 binary code.
* Then, switching to execution of binary code until encounter not compiled code.
* Saving the context, switching to jit again.

### supported assembly syntax
- [ ] nop
- [ ] add
- [ ] sub