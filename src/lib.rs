#[allow(dead_code)]
mod sql_parser {
    use regex::Regex;
    // ----- TYPES -----
    const RESERVED_WORDS: [&str; 4] = ["SELECT", "INSERT INTO", "DELETE FROM", "UPDATE"];

    enum Step {
        Type,
        SelectField,
        SelectComma,
        SelectFrom,
        SelectFromTable,
        Where,
    }

    pub struct Query {
        query_type: Option<String>,
        table_name: Option<String>, 
        conditions: Option<Vec<String>>,
        inserts: Option<Vec<String>>,
        fields: Option<Vec<String>>,
    }

    pub struct Parser {
        sql: String,
        query: Query,
        i: usize,
        step: Step,
    }

    // ----- METHODS -----
    impl Parser {
        pub fn new(s: String) -> Parser {
            Parser {
                sql: s,
                query: Query {
                    query_type: None,
                    table_name: None,
                    conditions: None,
                    inserts: None,
                    fields: None,
                },
                i: 0,
                step: Step::Type,
            }
        }

        // creates query struct from parser
        pub fn parse(&mut self) -> Result<&Query, String> {
            while self.i < self.sql.len() {
                let peeked: String = self.peek();
                match self.step {
                    Step::Type => match peeked.as_str() {
                        "SELECT" => {
                            self.query.query_type = Some(peeked);
                            self.step = Step::SelectComma;
                        }
                        "INSERT" => {
                            self.query.query_type = Some(peeked);
                        }
                        "DELETE" => {
                            self.query.query_type = Some(peeked);
                        }
                        "UPDATE" => {
                            self.query.query_type = Some(peeked);
                        }
                        _ => {
                            return Err("not a valid CRUD operation".to_string());
                        }
                    },
                    Step::SelectField => todo!(),
                    Step::SelectComma => todo!(),
                    Step::SelectFrom => todo!(),
                    Step::SelectFromTable => todo!(),
                    Step::Where => todo!(),
                }
            }
            Ok(&self.query)
        }

        // returns the next token to parse
        fn peek(&self) -> String {
            let (peeked, _) = self.peek_with_length();
            peeked
        }

        fn pop(&mut self) -> String {
            let (peeked, length) = self.peek_with_length();
            self.i += length;
            self.pop_whitespace();
            peeked
        }

        fn peek_with_length(&self) -> (String, usize) {
            // why you need here?
            if self.i >= self.sql.len() {
                return (String::new(), 0);
            }
            for word in RESERVED_WORDS.iter() {
                // why u need min here?
                let token: String = self.sql[self.i..std::cmp::min(self.sql.len(), self.i+word.len())].to_uppercase();
                if &token.as_str() == word {
                    let length: usize = token.len();
                    return (token, length);
                }
            }
            if self.sql.as_bytes()[self.i] == b'\'' {
                return self.peek_quoted_string_with_length();
            }
            self.peek_identifier_with_length()
        }

        // skip to next character following whitespace
        fn pop_whitespace(&mut self) {
            while self.i < self.sql.len() && self.sql.as_bytes()[self.i] == b' ' {
                println!("{}", self.i);
                self.i += 1;
            }
        }

        fn peek_identifier_with_length(&self) -> (String, usize) {
            let reg = Regex::new(r"[a-zA-Z0-9_*]").unwrap();
            for i in self.i..self.sql.len() {
                if !reg.is_match(&self.sql[i..i+1]) {
                    return (self.sql[self.i..i].to_string(), self.sql[self.i..i].len());
                } 
            }
            (self.sql[self.i..].to_string(), self.sql[self.i..].len())
        }

        fn peek_quoted_string_with_length(&self) -> (String, usize) {
            // why? 2nd or is in case it does not start with string
            if self.sql.len() < self.i || self.sql.as_bytes()[self.i] != b'\'' {
                return (String::new(), 0);
            }
            for i in self.i+1..self.sql.len() {
                // why \\?
                if self.sql.as_bytes()[i] == b'\'' && self.sql.as_bytes()[i] != b'\\' {
                    return (self.sql[self.i+1..i].to_string(), self.sql[self.i+1..i].len() + 2);
                } 
            }
            // WHY
            (String::new(), 0)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::sql_parser::*;

        #[test]
        fn test_new() {
            let parser: Parser = Parser::new(String::from("SELECT * FROM"));
            assert_eq!(parser.sql.as_str(), "SELECT * FROM");
            assert_eq!(parser.query.query_type, None); 
            assert_eq!(parser.i, 0);
            assert!(matches!(parser.step, Step::Type));
        }

        #[test]
        fn test_peek() {
            let parser: Parser = Parser::new(String::from("SELECT * FROM")); 
            assert_eq!(parser.peek(), "SELECT");
        }

        fn test_pop() {
            let mut parser: Parser = Parser::new(String::from("SELECT * FROM"));
            assert_eq!(parser.pop().as_str(), "SELECT");
            assert_eq!(parser.i, 6);
        }

        #[test]
        fn test_pop_whitespace() {
            let mut parser: Parser = Parser::new(String::from("     FROM"));
            parser.pop_whitespace();
            assert_eq!(parser.i, 5);

            parser.pop_whitespace();
            assert_eq!(parser.i, 5);
        }

        #[test]
        fn test_peek_with_length() {
            let parser = Parser::new(String::from("SELECT, * FROM USERS"));
            let (word, length) = parser.peek_with_length();
            assert_eq!(word.as_str(), "SELECT");
            assert_eq!(length, 6);
        }

        #[test]
        fn test_peek_quoted_string_with_length() {
            let parser = Parser::new(String::from("'ben'"));
            let (word, length) = parser.peek_with_length();
            assert_eq!(word.as_str(), "ben");
            assert_eq!(length, 5);
        }

        #[test]
        fn test_peek_identifier_with_length() {
            let parser = Parser::new(String::from("david, is an animal"));
            let (word, length) = parser.peek_identifier_with_length();
            assert_eq!(word.as_str(), "david");
            assert_eq!(length, 5);
        }
    }
}
