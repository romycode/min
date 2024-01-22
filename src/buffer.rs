use std::fmt::{Debug, Display, Formatter};

pub struct LineRange {
    start: usize,
    end: usize,
}

impl Display for LineRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {})",
            self.start,
            self.end,
        )
    }
}

impl Debug for LineRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl LineRange {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

pub struct Buffer {
    content: Vec<char>,
    lines: Vec<LineRange>,
}

impl Display for Buffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "buffer [ content: '{}', lines: '{:?}']",
            self.content.iter().collect::<String>(),
            self.lines,
        )
    }
}

impl Debug for Buffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            content: vec![],
            lines: vec![],
        }
    }
    pub fn from_str(content: &str) -> Self {
        let (content, mut last_lb, mut line, mut lines) = (content.chars().collect::<Vec<char>>(), 0, 0, vec![]);
        for (i, c) in content.iter().enumerate() {
            if '\n' == *c {
                lines[line] = LineRange::new(last_lb, i);
                line += 1;
                last_lb = i + 1;
            }
            if lines.len() == line {
                lines.push(LineRange::new(last_lb, last_lb));
            }
            lines[line].end = i;
        }
        Self { content, lines }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_buffer_from_str_with_one_line() {
        let buffer = Buffer::from_str("package main");
        assert_eq!(
            "buffer [ content: 'package main', lines: '[(0, 11)]']",
            buffer.to_string(),
        );
    }

    #[test]
    fn new_buffer_from_str_with_multiple_lines() {
        let buffer = Buffer::from_str("package main\nfunc main() {\n}\n");
        assert_eq!(
            "buffer [ content: 'package main\nfunc main() {\n}\n', lines: '[(0, 12), (13, 26), (27, 28), (29, 28)]']",
            buffer.to_string(),
        );
    }
}