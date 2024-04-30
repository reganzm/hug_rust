// 零件
trait Part {
    fn run(&self);
}

// 喇叭
struct Horn;
// 屏幕
struct Screen;
// 网络
struct Net;
// 变压器
struct Transformer;
impl Part for Horn {
    fn run(&self) {
        println!("喇叭已就位...")
    }
}
impl Part for Screen {
    fn run(&self) {
        println!("屏幕已就位...")
    }
}

impl Part for Net {
    fn run(&self) {
        println!("网络已就位")
    }
}
impl Part for Transformer {
    fn run(&self) {
        println!("变压器已就位...")
    }
}
// 创建外观
struct TvFacade {
    horn: Horn,
    screen: Screen,
    net: Net,
    transformer: Transformer,
}

impl TvFacade {
    fn init(&self) {
        self.horn.run();
        self.screen.run();
        self.net.run();
        self.transformer.run();
        println!("初始化完毕....\r\n")
    }
}

fn main() {
    let tv = TvFacade {
        horn: Horn,
        screen: Screen,
        net: Net,
        transformer: Transformer,
    };
    tv.init()
}
