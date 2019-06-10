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

    pub fn get_token(&mut self) -> Token {
        for c in self.input.chars() {
            if !c.is_whitespace() {
                break;
            }

            self.index += 1
        }

        println!("INDEX {:?}", self.index);

        let candidates: Vec<(&'static str, usize, usize, &'a str)> = self
            .rules
            .iter()
            .map(|(token_type, re)| (token_type, re.find(&self.input[self.index..])))
            .filter(|(_token_type, res)| res.is_some())
            .map(|(token_type, res)| {
                let res = res.unwrap();
                let start = res.start() + self.index;
                let end = res.end() + self.index;
                (*token_type, start, end, &self.input[start..=end])
            })
            .collect();

        //println!("candidates {:?}", candidates);

        let candidate = candidates
            .into_iter()
            .max_by_key(|(_token_type, _start, end, _s)| *end);
        println!("THE candidate {:?}", candidate);

        //TODO proper error habdling strategy
        if candidate.is_none() {
            panic!("FUYUU");
        }

        let candidate = candidate.unwrap();

        Token {
            kind: candidate.0,
            start: candidate.1,
            end: candidate.2,
            lexeme: candidate.3,
        }
    }
}

fn main() {
    let mut lexer = Lexer::new("   myVariable123 123 if");
    let token = lexer.get_token();

    println!("asdasd {:?}", token);

    //let res = Regex::new(r"^[[:alpha:]]\w*").unwrap().find(&" mivariable"[1..] );
    //println!("test {:?}", res);
}
