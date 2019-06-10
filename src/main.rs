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
        return Lexer {
            index: 0,
            input,
            rules: vec![
                ("ID", r"^[[:alpha:]]\w*"),
                ("NUM", r"^(\d+|\d+.\d+)"),
                ("IF", r"^if"),
            ]
            .into_iter()
            .map(|(token_type, raw_re)| (token_type, Regex::new(raw_re).unwrap()))
            .collect(),
        };
    }

    pub fn get_token(&mut self) -> Option<Token> {
        for c in self.input.chars().skip(self.index) {
            if !c.is_whitespace() {
                break;
            }

            self.index += 1
        }

        println!("INDEX {:?}", self.index);

        if self.index >= self.input.len() {
            return None;
        }

        let candidates: Vec<(&'static str, usize, usize, &'a str)> = self
            .rules
            .iter()
            .map(|(token_type, re)| (token_type, re.find(&self.input[self.index..])))
            .filter(|(_token_type, res)| res.is_some())
            .map(|(token_type, res)| {
                let res = res.unwrap();
                let start = res.start() + self.index;
                let end = res.end() + self.index;
                (*token_type, start, end, &self.input[start..end])
            })
            .collect();

        //println!("candidates {:?}", candidates);

        let candidate = candidates
            .into_iter()
            .max_by_key(|(_token_type, _start, end, _s)| *end);
        //println!("THE candidate {:?}", candidate);

        let ret = candidate.map(|(kind, start, end, lexeme)| {
            self.index = end;
            Token {
                kind,
                start,
                end,
                lexeme,
            }
        });

        println!("LEXER {:?}", self);

        return ret;
    }
}

fn main() {
    let input = "   myVariable123 123 if";
    for (i, c) in input.chars().enumerate() {
        println!("{:?} {:?}", i, c);
    }

    let mut lexer = Lexer::new("   myVariable123 123 if");

    while let Some(token) = lexer.get_token() {
        println!("Token {:?}", token);
    }

    //let res = Regex::new(r"^[[:alpha:]]\w*").unwrap().find(&" mivariable"[1..] );
    //println!("test {:?}", res);
}
