.global crt
crt:
    lea (%rip), %rax
    mov %rax, %rbp
    call main
    mov %rax, %rbx
    mov $1, %rax    # system call id
    mov %rbx, %rbx  # return code
    int $0x80       # exit program
