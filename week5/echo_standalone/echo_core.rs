pub fn echo_text(words:&[String])->String{
    format!("{}\n", words.join(" "))
}
#[cfg(test)]
mod tests{
    use super::echo_text;
    #[test]
    fn empty_input_returns_newline(){
        assert_eq!(echo_text(&[]),"\n");
    }
    #[test]
    fn prints_one_word(){
        let words = vec![String::from("hello")];
        assert_eq!(echo_text(&words),"hello\n");
    }
    #[test]
    fn joins_words_with_spaces(){
        let words = vec![
            String::from("hello"),
            String::from("Rust"),
        ];
        assert_eq!(echo_text(&words), "hello Rust\n");   }
    #[test]
    fn preserves_spaces_inside_an_argument(){
        let words = vec![String::from("hello rust")];
        assert_eq!(echo_text(&words),"hello rust\n")
    }
}