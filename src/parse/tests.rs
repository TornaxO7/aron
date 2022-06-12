// aron (c) Nikolas Wipper 2022

#[cfg(test)]
mod tests {
    use crate::parse::encodings::matches;
    use crate::parse::lexer::Lexer;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("# test comment\n.test_directive test_dir_arg\na_label:\nsome other stuff 0x00, 22".to_string());

        assert_eq!(lexer.read(), Ok("\n".to_string()));
        assert_eq!(lexer.read(), Ok(".".to_string()));
        assert_eq!(lexer.read(), Ok("test_directive".to_string()));
        assert_eq!(lexer.read(), Ok("test_dir_arg".to_string()));
        assert_eq!(lexer.read(), Ok("\n".to_string()));
        assert_eq!(lexer.read(), Ok("a_label".to_string()));
        assert_eq!(lexer.read(), Ok(":".to_string()));
        assert_eq!(lexer.read(), Ok("\n".to_string()));
        assert_eq!(lexer.read(), Ok("some".to_string()));
        assert_eq!(lexer.read(), Ok("other".to_string()));
        assert_eq!(lexer.read(), Ok("stuff".to_string()));
        assert_eq!(lexer.read(), Ok("0x00".to_string()));
        assert_eq!(lexer.read(), Ok(",".to_string()));
        assert_eq!(lexer.read(), Ok("22".to_string()));
    }

    #[test]
    fn test_assembler() {
        let instr = matches(&vec!["push".to_string(), "rbp".to_string()]);

        assert!(instr.is_some());
        let instr = instr.unwrap();
        assert_eq!(instr.get_bytes(), &vec![0x55u8]);
    }
}

