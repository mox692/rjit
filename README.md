rjit is a toy runtime assembler written in Rust.  
By using this, you can run or test the assembly directly without linking process.

## Feature
* x86_64 assembly (**NOT** 32bit)
* [AT＆T](https://csiflabs.cs.ucdavis.edu/~ssdavis/50/att-syntax.htm) syntax support.


## Quick Start
```
$ cargo build --release

// exit.s will exit with code 42.
$ ./target/release/rjit --file=./examples/exit.s

// check exit code.
$ echo $?
    -> 42
```


## Debug
If you want to check internal process , use gdb.

## Supported Assembly Syntax
- [x] nop
- [ ] add
- [ ] sub
- [x] mov
- [x] ret
- [x] int
- [x] syscall

also, please check examples directory.

## Restriction
* input is one assembly(.s) file.
* no libc support.
  * but I'm thinking of adding my own implementation for libc that doesn't need the linking process. (ex: printf, malloc, etc...)


## Reference
* [Intel SDM](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-instruction-set-reference-manual-325383.pdf)
* [Online x86 / x64 Assembler and Disassembler](https://defuse.ca/online-x86-assembler.htm)
* [X86 Opcode and Instruction Reference](http://ref.x86asm.net/coder64.html)
* [x86_64 機械語入門](https://tanakamura.github.io/pllp/docs/x8664_language.html)