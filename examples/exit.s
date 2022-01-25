#
# this will execute the exit syscall with exit code 42.
#

mov $60, %rax   # set syscall number.
mov $42, %rdi   # set exit code.
syscall