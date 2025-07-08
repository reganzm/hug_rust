//! 组合模式

// 组件接口
trait Component{
    fn name(&self)->&'static str;
    fn search(&self,keyword:&str);
}

// 文件(Leaf)
struct File{
    name:&'static str
}
impl File{
    fn new(name:&'static str)->Self{
        Self{name}
    }
}
impl Component for File{
    fn name(&self)->&'static str {
        self.name
    }
    fn search(&self,keyword:&str) {
        println!("在文件:‘{}’中搜索关键词:‘{}’",self.name,keyword);
    }
}

// 文件夹(Composite)
struct Folder{
    name:&'static str,
    childrens:Vec<Box<dyn Component>>
}
impl Folder{
    fn new(name:&'static str)->Self{
        Self{name,childrens:Vec::new()}
    }
    fn add_component(&mut self,component:impl Component+'static){
        self.childrens.push(Box::new(component));
    }
}
impl Component for Folder{
    fn name(&self)->&'static str {
        self.name
    }
    fn search(&self,keyword:&str) {
        println!("在文件夹:‘{}’下搜索关键字:‘{}’",self.name,keyword);
        self.childrens.iter().for_each(|chi|chi.search(keyword));
    }
}



fn main() {
    let file1 = File::new("组合模式.md");
    let file2 = File::new("特征.md");
    let file3 = File::new("struct.md");

    let mut folder1 = Folder::new("设计模式");
    folder1.add_component(file1);

    let mut folder2 = Folder::new("拥抱未来语言Rust");
    folder2.add_component(file2);
    folder2.add_component(file3);
    folder2.add_component(folder1);

    //      拥抱未来语言Rust
    //      /      |          \
    //   特征.md struct.md 设计模式
    //                        |
    //                      组合模式.md

    folder2.search("rust");
}
