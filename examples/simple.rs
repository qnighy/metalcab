extern crate metalcab;
use metalcab::compat::*;
use std::ffi::CStr;

fn main() {
    let input = "太郎は次郎が持っている本を花子に渡した。";

    // Create tagger object
    let mut mecab = Tagger::create2(CStr::from_bytes_with_nul(b"\0").unwrap()).unwrap();

    // Gets tagged result in string.
    {
        let result = mecab.parse(input).unwrap();
        println!("INPUT: {}", input);
        print!("RESULT:\n{}", result);
    }

    {
        let mut optnode = Some(mecab.parseToNode(input).unwrap());
        println!("INPUT: {}", input);
        while let Some(node) = optnode {
            print!("/{}", node.surface().unwrap());
            optnode = node.next();
        }
        println!("");
    }
}
