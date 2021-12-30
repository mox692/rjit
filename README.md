rjit is a toy x86-64 JIT Compiler written in Rust.

### current plan
* rjit reads x86 assembly as a input.
* While running rjit, it reads each function of asm and tranlate it to x86 binary code.
* Then, switching to execution of binary code until encounter not compiled code.
* Saving the context, switching to jit again.