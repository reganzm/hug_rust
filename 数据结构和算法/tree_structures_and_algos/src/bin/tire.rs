//! 字典树

#[derive(Default, Debug)]
struct Trie {
    root: Box<Node>, // 字典树的根节点
}

#[derive(Default, Debug)]
struct Node {
    mark: bool,                      // 标识位：用于标识是否从根节点到此节点组成一个单词
    childs: [Option<Box<Node>>; 26], //26个英文字母，只需要26个槽位
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    /// 向字典树插入单词
    fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;
        //逐个字符插入
        for c in word.as_bytes() {
            // 字母在当前的childs的索引
            let index = (c - b'a') as usize;
            let child = &mut current_node.childs[index];
            // 如果child存在则插入，否则新建
            current_node = child.get_or_insert_with(Box::<Node>::default);
        }
        // 将current_node的mark标记为true，表示从根节点到此节点组成一个单词
        current_node.mark = true;
    }

    fn contains(&self, word: &str) -> bool {
        self.word_node(word).map_or(false, |node| node.mark)
    }

    fn start_with(&self, prifix: &str) -> bool {
        self.word_node(prifix).is_some()
    }

    fn word_node(&self, word: &str) -> Option<&Node> {
        let mut current_node = &self.root;
        for c in word.as_bytes() {
            let index = (c - b'a') as usize;
            match &current_node.childs[index] {
                None => return None,
                Some(node) => current_node = node,
            }
        }
        Some(&current_node)
    }
}

fn main() {
    let mut trie = Trie::new();

    trie.insert("rust");
    trie.insert("hug");
    trie.insert("hello");
    trie.insert("hugrust");

    println!("hello in Trie:{}", trie.contains("hello"));
    println!("huga in Trie:{}", trie.contains("huga"));
    println!("Trie start with hella:{}", trie.start_with("hella"));
    println!("Trie start with rust:{}", trie.start_with("rust"));
}
