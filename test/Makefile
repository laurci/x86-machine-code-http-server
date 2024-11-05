clean:
	rm -f *.bin
	rm -f *.o

test: test.c
	cc -o test.bin test.c
	./test.bin

server: server.c
	cc -o server.bin server.c
	./server.bin

server_raw: server_raw.c
	# compile 32-bit binary, no libc
	cc -m32 -nostdlib -O0 -g -o server_raw.bin server_raw.c
	./server_raw.bin

server_asm: server_asm.S
	nasm -f elf32 server_asm.S -o server_asm.o
	ld -m elf_i386 server_asm.o -o server_asm.bin
	./server_asm.bin