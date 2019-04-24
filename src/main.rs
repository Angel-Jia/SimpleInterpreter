#[macro_use]
extern crate lazy_static;


use std::env;
use std::fmt;
use std::collections::{HashMap, VecDeque};

lazy_static!{
    static ref KeyWord: HashMap<&'static str, Token> = {
        let mut m = HashMap::new();
        m.insert("BEGIN", Token::BEGIN);
        m.insert("END", Token::END);
        m
    };
}

#[derive(Clone)]
enum Token{
    EOF,
    OP1(char),
    OP2(char),
    LP,
    RP,
    INTEGER(u64),
    UNARY(char),
    BEGIN,
    END,
    DOT,
    ASSIGN,
    SEMI,
    ID(String),
}

struct Interpreter{
    text: Vec<char>,
    idx: usize,
}


struct TreeNode{
    OP: Token,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl fmt::Debug for Token{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Token::INTEGER(n) => write!(f, "INTEGER: {}", n),
            Token::OP1(c) => write!(f, "operation: {}", c),
            Token::OP2(c) => write!(f, "operation: {}", c),
            Token::EOF => write!(f, "EOF"),
            Token::LP => write!(f, "("),
            Token::RP => write!(f, ")"),
            Token::UNARY(c) => write!(f, "UNARY: {}", c),
            Token::BEGIN => write!(f, "BEGIN"),
            Token::END => write!(f, "END"),
            Token::DOT => write!(f, "DOT"),
            Token::ID(s) => write!(f, "variable: {}", s),
            Token::ASSIGN => write!(f, "ASSIGN"),
            Token::SEMI => write!(f, "SEMI"),
        }
    }
}

impl Interpreter{
    pub fn from(s: &String) -> Self{
        Interpreter{text: s.chars().collect(), idx: 0}
    }

    pub fn _id(&mut self) -> Token{
        let mut s: String = String::new();

        while self.idx < self.text.len(){
            match self.text[self.idx]{
                c @ 'a'...'z' | c @ 'A'...'Z' | c @ '0'...'9' => {
                    s.push(c);
                    self.idx += 1;
                },
                _ => break,
            }
        }
        match KeyWord.get(s.as_str()){
            Some(value) => value.clone(),
            None => Token::ID(s),
        }
    }

    pub fn get_digits(&mut self) -> u64{
        let mut ret: u64 = self.text[self.idx] as u64 - '0' as u64;
        self.idx += 1;
        while self.idx < self.text.len(){
            match self.text[self.idx]{
                c @ '0' ... '9' => {
                    ret = ret * 10 + c as u64 - '0' as u64;
                    self.idx += 1;
                },
                _ => break,
            }
        }
        ret
    }

    pub fn get_next_token(&mut self) -> Token{
        if self.idx >= self.text.len() {return Token::EOF;}
        match self.text[self.idx]{
            'a' ... 'z' | 'A' ... 'Z' => self._id(),
            ':' => {
                if self.idx + 1 < self.text.len() && self.text[self.idx + 1] == '='{
                    self.idx += 2;
                    Token::ASSIGN
                }else{
                    panic!("error in with char ':', missing '=' ");
                }
            },
            ';' => {
                self.idx += 1;
                Token::SEMI
            },
            '.' => {
                self.idx += 1;
                return Token::DOT;
            },
            '0'..='9' => Token::INTEGER(self.get_digits()),
            c @ '+' | c @ '-' => {self.idx += 1; Token::OP1(c)},
            c @ '*' | c @ '/' => {self.idx += 1; Token::OP2(c)},
            '(' => {self.idx += 1; Token::LP},
            ')' => {self.idx += 1; Token::RP},
            ' ' | '\n' => {
                self.idx += 1;
                while self.idx < self.text.len(){
                    match self.text[self.idx]{
                        ' ' | '\n' => self.idx += 1,
                        _ => break,
                    }
                }
                self.get_next_token()
            },
            e => panic!(format!("error in chars {}({})", e, self.idx)),
        }
    }

    // pub fn factor(&mut self) -> Box<TreeNode>{
        
    // }

    // pub fn term(&mut self) -> Box<TreeNode>{
        
    // }

    // pub fn expr(&mut self) -> Box<TreeNode>{
    
    // }
        
}


 

fn main() {
    let input = "BEGIN\n".to_string() + "    BEGIN\n" + "        number := 2;\n"
    + "        a := number;\n" + "        b := 10 * a + 10 * number / 4;\n" + "        c := a - - b\n"
    + "    END;\n" + "    x := 11;\n" + "END.";

    let mut inp = Interpreter::from(&input);

    let mut token = inp.get_next_token();
    loop{
        match token{
            Token::EOF => break,
            _ => {
                println!("{:?}", token);
                token = inp.get_next_token();
            },
        }
    }

    //node.post_t();

    //println!("{:?}", node);

    //println!("\nret: {}\n", TreeNode::visit(&node));
}
