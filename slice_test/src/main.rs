

fn main() {
    let s = String::from("hello world");
    //let word = first_word(&s);
    let word2 = first_word(&s[0..6]);
    println!("{}", word2);
}

// &str字符串切片类型

/* 
函数接受由空格分隔的单词字符串
并返回在该字符串中找到的第一个单词
如果函数在字符串中没有找到空格
则整个字符串必须是一个单词
因此应返回整个字符串
*/

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}




