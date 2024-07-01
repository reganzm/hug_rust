# A test code for kvmsample
# 程序入口标号为_start，代码段为16位
.globl _start
    .code16
_start:
    # 初始化ax寄存器为0
    xorw %ax, %ax
# 循环    
loop1:
    # 将ax的值输出到0x10端口，即标准输出
    out %ax, $0x10
    # ax中的值+1
    inc %ax
    # 跳转到循环处继续执行
    jmp loop1

