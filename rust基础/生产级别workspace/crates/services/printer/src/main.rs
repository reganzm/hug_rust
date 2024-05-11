// 在包里面引用其它的包
use lib_core::core1::core1_test;
use lib_core::core2::core_test2;
use lib_core::core3::fn1::fn1;
use lib_core::core3::fn2::fn2;
use lib_utils::util1::util1;
use lib_utils::util2::util2;
// 使用as命名别名解决命名冲突
use lib_utils::utils3::fn1::fn1 as util_fn1;
use lib_utils::utils3::fn2::fn2 as util_fn2;

mod testx;

fn main() {
    core1_test();
    core_test2();
    fn1();
    fn2();
    util1();
    util2();
    util_fn1();
    util_fn2();
}
