#[macro_use]
extern crate lazy_static;

use std::env;
use std::fmt;
use std::collections::{HashMap, VecDeque};

lazy_static!{
    static ref op_priority: HashMap<char, u32> = {
        let mut m = HashMap::new();
        m.insert('+', 1);
        m.insert('-', 1);
        m.insert('*', 2);
        m.insert('/', 2);
        m.insert('(', 0);
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
    INTEGER(i64),
    UNARY(char),
}

struct Interpreter{
    text: String,
    tokens: Vec<Token>,
    idx: usize,
}


struct TreeNode{
    OP: Token,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn get_digits(v_slice: &Vec<char>, idx: &mut usize) -> u64{
    let mut ret: u64 = v_slice[*idx] as u64 - '0' as u64;
    (*idx) += 1;
    while *idx < v_slice.len(){
        match v_slice[*idx]{
            '0'..='9' => {ret = ret * 10 + (v_slice[*idx] as u64 - '0' as u64);
                          (*idx) += 1;},
            _ => return ret,
        }
    }
    ret
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
            Token::UNARY(c) => write!(f, "UNARY: {}", c)
        }
    }
}

impl Interpreter{
    pub fn from(s: &String) -> Option<Interpreter>{
        let s_vec: Vec<char> = s.clone().chars().collect();
        let mut tok: Vec<Token> = vec![];

        let mut idx: usize = 0;
        while idx < s_vec.len(){
            match s_vec[idx]{
                '0'..='9' => tok.push(Token::INTEGER(get_digits(&s_vec, &mut idx) as i64)),
                '+' | '-' => {tok.push(Token::OP1(s_vec[idx])); idx += 1;},
                '*' | '/' => {tok.push(Token::OP2(s_vec[idx])); idx += 1;},
                '(' => {tok.push(Token::LP); idx += 1;},
                ')' => {tok.push(Token::RP); idx += 1;},
                ' ' => idx += 1,
                _ => return None,
            }
        }
        tok.push(Token::EOF);

        Some(Interpreter{text: s.clone(), tokens: tok, idx: 0})
    }

    pub fn factor(&mut self) -> Box<TreeNode>{
        let token = self.tokens[self.idx].clone();
        match &self.tokens[self.idx]{
            Token::OP1(c) => {
                self.idx += 1;
                let tmp_node = Box::new(TreeNode{
                        OP: Token::UNARY(*c),
                        left: Some(self.factor()),
                        right: None,
                    });
                tmp_node
            },
            Token::INTEGER(_) => {
                self.idx += 1;
                return Box::new(TreeNode{
                    OP: token,
                    left: None,
                    right: None,
                });
            },
            Token::LP => {
                self.idx += 1;
                let node = self.expr();
                match &self.tokens[self.idx]{
                    Token::RP => {self.idx += 1},
                    e => {
                        println!("\n idx: {}, token: {:?}\n", self.idx, e);
                        panic!(format!("error in fn factor! idx"))},
                }
                return node;
            },
            e => {
                    println!("\n idx: {}, token: {:?}\n", self.idx, e);
                    panic!(format!("error in fn factor! idx"))
                },
        }
    }

    pub fn term(&mut self) -> Box<TreeNode>{
        let mut node = self.factor();
        loop{
            match self.tokens[self.idx]{
                Token::OP2(c) => {
                        self.idx += 1;
                        node = Box::new(TreeNode{
                                        OP: Token::OP2(c), 
                                        left: Some(node), 
                                        right: Some(self.factor()),
                                    });
                    },
                _ => return node,
            }
        }
    }

    pub fn expr(&mut self) -> Box<TreeNode>{
        let mut node = self.term();
        loop{
            match self.tokens[self.idx]{
                Token::OP1(c) => {
                        self.idx += 1;
                        node = Box::new(TreeNode{
                                        OP: Token::OP1(c), 
                                        left: Some(node), 
                                        right: Some(self.term()),
                                    });
                    },
                _ => return node,
            }
        }
    }
}

impl fmt::Debug for Box<TreeNode>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let mut s: VecDeque<&Box<TreeNode>> = VecDeque::new();
        let mut ret = String::from("\n");
        s.push_back(&self);
        //ret += format!("{:?}\n", self.OP).as_str();

        let mut pre_level_num:usize;
        let mut this_level_num:usize = 1;
        while !s.is_empty(){
            pre_level_num = this_level_num;
            this_level_num = 0;
            for _ in 0..pre_level_num{
                let node = s.pop_front().unwrap();
                ret += format!("{:?}    ", node.OP).as_str();
                match node.OP{
                    Token::INTEGER(_) => {},
                    Token::OP1(_) | Token::OP2(_) => {
                        s.push_back(node.left.as_ref().unwrap());
                        s.push_back(node.right.as_ref().unwrap());
                        this_level_num += 2;
                    },
                    _ => panic!("illegal node when debug: {:?}", node.OP),
                }
            }
            ret += "\n";
        }
        write!(f, "{}", ret)
    }
}

impl TreeNode{
    pub fn post_t(&self){
        let mut ret = String::new();
        match &self.OP{
            Token::INTEGER(n) => {println!("{}", n)},
            Token::OP1(c) | Token::OP2(c) => {
                TreeNode::post_r(&self.left.as_ref().unwrap(), &mut ret);
                TreeNode::post_r(&self.right.as_ref().unwrap(), &mut ret);
                ret.push(*c);
                println!("{}", ret);
            }
            _ => panic!("error OP type in TreeNode"),
        }
    }

    fn post_r(root: &Box<TreeNode>, ret: &mut String){
        match root.OP{
            Token::INTEGER(n) => {ret.push_str(format!("{}", n).as_str())},
            Token::OP1(c) | Token::OP2(c) => {
                TreeNode::post_r(root.left.as_ref().unwrap(), ret);
                TreeNode::post_r(root.right.as_ref().unwrap(), ret);
                ret.push(c);
            }
            _ => panic!("error OP type in TreeNode")
        }
    }

    fn visit(root: &Box<TreeNode>) -> i64{
        match &root.OP{
            Token::INTEGER(n) => return *n,
            Token::UNARY(c) => {
                if *c == '-'{
                    return -TreeNode::visit(root.left.as_ref().unwrap());
                }else{
                    return -TreeNode::visit(root.left.as_ref().unwrap());
                }
            },
            Token::OP1(c) | Token::OP2(c) => {
                let l = TreeNode::visit(root.left.as_ref().unwrap());
                let r = TreeNode::visit(root.right.as_ref().unwrap());
                match c{
                    '+' => return l + r,
                    '-' => return l - r,
                    '*' => return l * r,
                    '/' => return l / r,
                    _ => panic!("error in vistit_OP(x)"),
                }
            }
            _ => panic!("error in vistit AST tree"),
        }
    }
}

