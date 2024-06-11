//! mini virtual machine 迷你虚拟机
#[derive(Debug)]
struct Instruct {
    // 指令
    code: IP,
    // 执行指令的条件
    cond: Condition,
    // 指令的参数指针
    p1: i32,
    p2: i32,
}
// 指令枚举
#[derive(Debug)]
enum IP {
    IADD,  // 加法
    ISUB,  // 减法
    ICMP,  // 判断
    IJMP,  // 跳转
    IMOV,  // 赋值
    ISTIP, // 保存ip
    ILDIP, // 加载ip(跳转)
    ILD,   // 加载一个立即数
    IOUT,  // 输出
    ISTOP, // 挂起虚拟机
}
// 条件枚举
#[derive(Debug, PartialEq)]
enum Condition {
    FNA, // 任何状态下都执行
    FEQ, // 状态为相等时执行
    FNE, // 状态不相等时执行
}
// 保存状态的结构体
#[derive(Debug)]
struct VMState<'a> {
    ip: i32,                  // 指令指针，指向具体的指令Instruct
    flag: Condition,          //判断标志
    code_ptr: &'a [Instruct], // 代码段地址
    data_ptr: &'a mut [i32],  //数据段地址
}

impl<'a> VMState<'a> {
    // 使用代码段和数据段初始化构建一个虚拟机
    fn build(code_ptr: &'a [Instruct], data_ptr: &'a mut [i32]) -> Self {
        VMState {
            ip: 0,
            flag: Condition::FNA,
            code_ptr,
            data_ptr,
        }
    }
}

struct VMMachine;
impl VMMachine {
    fn execute(vm_state: &mut VMState) {
        loop {
            // 一直循环知道挂起为止
            let current_index = vm_state.ip as usize;
            // 指令自动后移
            vm_state.ip += 1;
            let current_instruct = &vm_state.code_ptr[current_index];
            if current_instruct.cond != Condition::FNA && current_instruct.cond != vm_state.flag {
                continue;
            }
            match current_instruct.code {
                IP::IADD => {
                    vm_state.data_ptr[current_instruct.p1 as usize] +=
                        vm_state.data_ptr[current_instruct.p2 as usize];
                }
                IP::ISUB => {
                    vm_state.data_ptr[current_instruct.p1 as usize] -=
                        vm_state.data_ptr[current_instruct.p2 as usize];
                }
                IP::ICMP => {
                    // 比较p1和p2指向的数据
                    if vm_state.data_ptr[current_instruct.p1 as usize]
                        == vm_state.data_ptr[current_instruct.p2 as usize]
                    {
                        vm_state.flag = Condition::FEQ;
                    } else {
                        vm_state.flag = Condition::FNE;
                    }
                }
                IP::IJMP => {
                    // 根据p1进行偏移
                    vm_state.ip += current_instruct.p1;
                }
                IP::IMOV => {
                    // 将p1指向的数据设置为p2指向的数据
                    vm_state.data_ptr[current_instruct.p1 as usize] =
                        vm_state.data_ptr[current_instruct.p2 as usize];
                }
                IP::ISTIP => {
                    // 将IP保存到p1指向的数据
                    vm_state.data_ptr[current_instruct.p1 as usize] = vm_state.ip;
                }
                IP::ILDIP => {
                    // 将ip设置为p1指向的数据
                    vm_state.ip = vm_state.data_ptr[current_instruct.p1 as usize];
                }
                IP::ILD => {
                    // 将立即数p2 加载到p1指向的数据
                    vm_state.data_ptr[current_instruct.p1 as usize] = current_instruct.p2;
                }
                IP::IOUT => {
                    // 输出p1指向的数据
                    println!("result:{}", vm_state.data_ptr[current_instruct.p1 as usize]);
                }
                IP::ISTOP => {
                    return;
                }
            }
        }
    }
}

fn main() {
    //下面的指令是
    // ```
    // let mut sum = 0;
    // for i in 0..101{
    //   sum += i
    // }
    // ```
    // 编译成的指令
    //
    let codes = [
        Instruct {code: IP::ILD,  cond: Condition::FNA,p1: 2, p2: 101},
        Instruct {code: IP::ILD,  cond: Condition::FNA,p1: 3, p2: 1},
        Instruct {code: IP::ILD,  cond: Condition::FNA,p1: 1, p2: 1},
        Instruct {code: IP::ILD,  cond: Condition::FNA,p1: 0, p2: 0},
        Instruct {code: IP::ICMP, cond: Condition::FNA,p1: 1, p2: 2},
        Instruct {code: IP::IJMP, cond: Condition::FEQ,p1: 3, p2: 0},
        Instruct {code: IP::IADD, cond: Condition::FNA,p1: 0, p2: 1},
        Instruct {code: IP::IADD, cond: Condition::FNA,p1: 1, p2: 3},
        Instruct {code: IP::IJMP, cond: Condition::FNA,p1: -5,p2: 0},
        Instruct {code: IP::IOUT, cond: Condition::FNA,p1: 0, p2: 0},
        Instruct {code: IP::ISTOP,cond: Condition::FNA,p1: 0, p2: 0}
    ];
    let mut datas = [0; 16];
    let mut vm_state: VMState = VMState::build(&codes, &mut datas);
    VMMachine::execute(&mut vm_state);
}
