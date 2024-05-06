trait File {
    fn read(&self);
}

struct TxtFile {
    file_name: String,
}

impl TxtFile {
    fn load_from_dist(&self) {
        println!("加载文件:{}", &self.file_name);
    }
    fn new(file_name: String) -> Self {
        let file = TxtFile { file_name };
        file.load_from_dist();
        file
    }
}
impl File for TxtFile {
    fn read(&self) {
        println!("读取文件:{}", &self.file_name);
    }
}

struct ProxyFile {
    txt_file: TxtFile,
    file_name: String,
}
impl ProxyFile {
    fn new(file_name: String) -> Self {
        Self {
            txt_file: TxtFile::new(file_name.clone()),
            file_name,
        }
    }
}
impl File for ProxyFile {
    fn read(&self) {
        println!("-----------通过代理读取文件开始-----------");
        self.txt_file.read();
        println!("-----------通过代理读取文件结束-----------")
    }
}

fn main() {
    let proxy_file = ProxyFile::new("拥抱未来语言Rust".to_string());
    proxy_file.read();
}
