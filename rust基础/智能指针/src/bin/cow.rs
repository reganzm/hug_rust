use std::borrow::Cow;

fn main() {
    fn delete_space<'a>(src: &'a str) -> Cow<'a, str> {
        if src.contains(' ') {
            let mut dest = String::with_capacity(src.len());
            for c in src.chars() {
                if ' ' != c {
                    dest.push(c);
                }
            }
            return Cow::Owned(dest);
        }
        return Cow::Borrowed(src);
    }
    let a = "我想买 U8";
    println!("a:{a}");
    let b = delete_space(a);
    println!("b:{b}");
}
