static DEFAULT_DICT: &str = include_str!("../data/dict.txt");

#[cfg(test)]
mod tests {
    use crate::DEFAULT_DICT;

    #[test]
    fn test_hello_world() {
        println!("hello world!")
    }

    #[test]
    fn test_chinese() {
        // 默认词典
        let jieba = jieba_rs::Jieba::new();
        let words = jieba.cut("我的24岁美女总裁老婆", false);
        println!("[我的24岁美女总裁老婆] 分词结果：{:?}", words);

        let words = jieba.cut("都市最强狂兵", false);
        println!("[都市最强狂兵] 分词结果：{:?}", words);

        println!();
        println!();
        // 自定义词典
        use std::io::BufReader;
        let mut default_dict = BufReader::new(DEFAULT_DICT.as_bytes());
        let custom_jieba = jieba_rs::Jieba::with_dict(&mut default_dict).unwrap();

        let words = custom_jieba.cut("我的24岁美女总裁老婆", false);
        println!("[我的24岁美女总裁老婆] 分词结果：{:?}", words);

        let words = custom_jieba.cut("都市最强狂兵", false);
        println!("[都市最强狂兵] 分词结果：{:?}", words);
    }
}
