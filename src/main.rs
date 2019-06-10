use regex::Regex;

#[derive(Debug)]
struct Token<'a> {
    kind: &'static str,
    lexeme: &'a str,
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Lexer<'a> {
    index: usize,
    input: &'a str,
    rules: Vec<(&'static str, Regex)>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer {
            index: 0,
            input,
            rules: vec![
                ("ID", r"^[[:alpha:]]\w*"),
                ("NUM", r"^(\d+|\d+.\d+)"),
                ("IF", r"^if"),
            ]
            .into_iter()
            .map(|(kind, raw_re)| (kind, Regex::new(raw_re).expect("A valid regex")))
            .collect(),
        }
    }

    pub fn get_token(&mut self) -> Option<Token> {
        for c in self.input.chars().skip(self.index) {
            if !c.is_whitespace() {
                break;
            }

            self.index += 1
        }

        if self.index >= self.input.len() {
            return None;
        }

        let candidates: Vec<Token> = self
            .rules
            .iter()
            .map(|(kind, re)| (kind, re.find(&self.input[self.index..])))
            .filter(|(_kind, res)| res.is_some())
            .map(|(kind, res)| {
                let res = res.unwrap();
                let start = res.start() + self.index;
                let end = res.end() + self.index;
                Token {
                    kind,
                    start,
                    end,
                    lexeme: &self.input[start..end],
                }
            })
            .collect();

        candidates
            .into_iter()
            .max_by_key(|token| token.end)
            .map(|token| {
                self.index = token.end;
                token
            })
    }
}

fn main() {
    let input = "   myVariable123 123 if";
    for (i, c) in input.chars().enumerate() {
        println!("{:?} {:?}", i, c);
    }

    let mut lexer = Lexer::new("   myVariable123 123 if");

    while let Some(token) = lexer.get_token() {
        println!("{:?}", token);
    }

    //let res = Regex::new(r"^[[:alpha:]]\w*").unwrap().find(&" mivariable"[1..] );
    //println!("test {:?}", res);
}
