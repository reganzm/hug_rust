all: kvmsample test.bin test2.bin

kvmsample: main.o
	gcc main.c -o kvmsample -lpthread
# ld 链接器
# -m指定为 elf_i386 格式的可执行文件
# -oformat binary 指定为二进制文件
# -N 不启用符号表
# -e 指定程序入口标号
# -Ttext 指定代码段地址为0x10000 
# -o 指定将test.o 链接为test.bin输出文件
test.bin: test.o
	ld -m elf_i386 --oformat binary -N -e _start -Ttext 0x10000 -o test.bin test.o

test.o: test.S
	# 汇编编译为32位输出
	# -o 指定输出文件名称
	as -32 test.S -o test.o

test2.bin: test2.o
	ld -m elf_i386 --oformat binary -N -e _start -Ttext 0x10000 -o test2.bin test2.o

test2.o: test2.S
	as -32 test2.S -o test2.o
