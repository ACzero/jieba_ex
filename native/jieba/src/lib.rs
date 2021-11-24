use jieba_rs::Jieba;
use lazy_static::lazy_static;
use std::env;
use std::fs::File;
use std::io::BufReader;

lazy_static! {
    static ref JIEBA: Jieba = Jieba::with_dict(&mut BufReader::new(
        File::open(env::var("JIEBA_DICT_DIR").unwrap()).unwrap()
    ))
    .unwrap();
}

#[rustler::nif]
pub fn cut(txt: String) -> Vec<String> {
    let words = JIEBA.cut(&txt, false);
    words.iter().map(|n| n.to_string()).collect()
}

rustler::init!("Elixir.Jieba", [cut]);
