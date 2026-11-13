#[derive(Debug, PartialEq, Eq)]
pub struct Counts {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
}

pub fn count_text(text: &str) -> Counts {
    Counts {
        lines: text.bytes().filter(|byte| *byte == b'\n').count(),
        words: text.split_whitespace().count(),
        bytes: text.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_empty_text() {
        assert_eq!(
            count_text(""),
            Counts {
                lines: 0,
                words: 0,
                bytes: 0,
            }
        );
    }

    #[test]
    fn counts_normal_text() {
        assert_eq!(
            count_text("hello world\nRust\n"),
            Counts {
                lines: 2,
                words: 3,
                bytes: 17,
            }
        );
    }

    #[test]
    fn counts_only_newline_characters_as_lines() {
        assert_eq!(
            count_text("hello"),
            Counts {
                lines: 0,
                words: 1,
                bytes: 5,
            }
        );
    }

    #[test]
    fn handles_whitespace_and_utf8() {
        assert_eq!(
            count_text("hi  café\n"),
            Counts {
                lines: 1,
                words: 2,
                bytes: 10,
            }
        );
    }
}