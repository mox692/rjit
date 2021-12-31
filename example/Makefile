CC = gcc
CFLAGS = -fno-pic -static -fno-builtin -fno-strict-aliasing -O2 -Wall -MD -ggdb -Werror -fno-omit-frame-pointer
CFLAGS += $(shell $(CC) -fno-stack-protector -E -x c /dev/null >/dev/null 2>&1 && echo -fno-stack-protector)
LD = ld

OBJS = 1.o

c: 1.c crt.s
	$(CC) $(CFLAGS) -c -o 1.o 1.c
	$(CC) $(CFLAGS) -c -o crt.o crt.s
	$(LD) -N -e crt -o 1.bin 1.o crt.o

b:;
	gcc -c -o 1.o 1.s

clean:;
	rm -f *.o *.bin *.d