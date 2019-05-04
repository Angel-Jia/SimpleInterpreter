#[macro_use]
extern crate lazy_static;

use std::fmt;
use std::collections::{HashMap, VecDeque};

//key word: BEGIN END INTEGER REAL DIV PROGRAM VAR

//AST node type(Token::Other): PROGRAM BLOCK VARDEC Empty COMP

lazy_static!{
    static ref KeyWord: HashMap<&'static str, Token> = {
        let mut m = HashMap::new();
        m.insert("BEGIN", Token::KEYWORD("BEGIN".to_string()));
        m.insert("END", Token::KEYWORD("END".to_string()));
        m.insert("INTEGER", Token::KEYWORD("INTEGER".to_string()));
        m.insert("REAL", Token::KEYWORD("REAL".to_string()));
        m.insert("DIV", Token::KEYWORD("DIV".to_string()));
        m.insert("PROGRAM", Token::KEYWORD("PROGRAM".to_string()));
        m.insert("VAR", Token::KEYWORD("VAR".to_string()));
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
    INTEGER_CONST(u64),
    REAL_CONST(f64),
    UNARY(char),
    KEYWORD(String),
    DOT,
    ASSIGN,
    SEMI,
    COMMA,
    COLON,
    ID(String),
    ASTNode(String),
}

enum Symbol{
    Real(f64),
    Integer(i64),
}

struct Interpreter{
    text: Vec<char>,
    current_token: Token,
    idx: usize,
}

struct TreeNode{
    node: Token,
    sub_nodes: Vec<TreeNode>,
}

#[derive(Clone)]
enum VarType{
    Integer(i64),
    Real(f64),
}

impl fmt::Debug for VarType{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            VarType::Integer(n) => write!(f, "INTEGER({})", n),
            VarType::Real(n) => write!(f, "REAL({})", n),
        }
    }
}

impl fmt::Debug for Token{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Token::INTEGER_CONST(n) => write!(f, "INTEGER: {}", n),
            Token::REAL_CONST(n) => write!(f, "REAL: {}", n),
            Token::OP1(c) => write!(f, "operation: {}", c),
            Token::OP2(c) => write!(f, "operation: {}", c),
            Token::EOF => write!(f, "EOF"),
            Token::LP => write!(f, "("),
            Token::RP => write!(f, ")"),
            Token::UNARY(c) => write!(f, "UNARY: {}", c),
            Token::KEYWORD(s) => write!(f, "KEYWORD: {}", s),
            Token::DOT => write!(f, "DOT"),
            Token::ID(s) => write!(f, "variable: {}", s),
            Token::ASSIGN => write!(f, "ASSIGN"),
            Token::SEMI => write!(f, "SEMI"),
            Token::COMMA => write!(f, "COMMA"),
            Token::COLON => write!(f, "COLON"),
            Token::ASTNode(s) => write!(f, "ASTNode: {}", s),
        }
    }
}

impl PartialEq for Token{
    fn eq(&self, other: &Token) -> bool{
        match (self, other){
            (Token::EOF, Token::EOF) => true,
            (Token::OP1(_), Token::OP1(_)) => true,
            (Token::OP2(_), Token::OP2(_)) => true,
            (Token::LP, Token::LP) => true,
            (Token::RP, Token::RP) => true,
            (Token::INTEGER_CONST(_), Token::INTEGER_CONST(_)) => true,
            (Token::REAL_CONST(_), Token::REAL_CONST(_)) => true,
            (Token::UNARY(_), Token::UNARY(_)) => true,
            (Token::KEYWORD(a), Token::KEYWORD(b)) => a == b,
            (Token::DOT, Token::DOT) => true,
            (Token::ASSIGN, Token::ASSIGN) => true,
            (Token::SEMI, Token::SEMI) => true,
            (Token::COMMA, Token::COMMA) => true,
            (Token::COLON, Token::COLON) => true,
            (Token::ID(_), Token::ID(_)) => true,
            (Token::ASTNode(a), Token::ASTNode(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialEq for VarType{
    fn eq(&self, other: &VarType) -> bool{
        match (self, other){
            (VarType::Integer(_), VarType::Integer(_)) => true,
            (VarType::Real(_), VarType::Real(_)) => true,
            _ => false,
        }
    }
}


impl Interpreter{
    pub fn from(s: &String) -> Self{
        let mut a = Interpreter{text: s.chars().collect(), current_token: Token::EOF, idx: 0};
        a.get_next_token();
        a
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
            None => Token::ID(s.clone()),
        }
    }

    pub fn get_digits(&mut self) -> Token{
        let start = self.idx;
        let length = self.text.len();
        let mut one_dot = false;
        self.idx += 1;
        loop{
            if self.idx >= length {panic!("error in fn get_digits, idx({}) over size({}).", self.idx, length)};
            match self.text[self.idx]{
                '0'...'9' => self.idx +=1,
                '.' => {
                    if !one_dot {one_dot = true; self.idx += 1;}
                    else {panic!("find more than one dot in number")}
                },
                _ => break,
            }
        }

        let num = self.text[start..self.idx].iter().collect::<String>();
        //println!("\nnum: {}\n", num);

        if one_dot{
            Token::REAL_CONST(num.parse::<f64>().unwrap())
        }else{
            Token::INTEGER_CONST(num.parse::<u64>().unwrap())
        }
    }

    pub fn get_next_token(&mut self) -> Token{
        let mut ret: Token = Token::EOF;
        if self.idx >= self.text.len() {self.current_token = ret.clone(); return ret;}
        ret = match self.text[self.idx]{
                'a' ... 'z' | 'A' ... 'Z' => self._id(),
                ':' => {
                    if self.idx + 1 < self.text.len() && self.text[self.idx + 1] == '='{
                        self.idx += 2;
                        Token::ASSIGN
                    }else{
                        self.idx += 1;
                        Token::COLON
                    }
                },
                ';' => {
                    self.idx += 1;
                    Token::SEMI
                },
                ',' => {
                    self.idx += 1;
                    Token::COMMA
                }
                '.' => {
                    self.idx += 1;
                    Token::DOT
                },
                '0'..='9' => self.get_digits(),
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
                '{' => {
                    self.skip_comment();
                    self.get_next_token()
                }
                e => panic!("error in chars {}({})", e, self.idx),
            };
        self.current_token = ret.clone();
        ret
    }

    fn skip_comment(&mut self){
        let length = self.text.len();
        while self.idx < length && self.text[self.idx] != '}'{
            self.idx += 1;
        }
        if self.idx >= length{
            panic!("error in fn skip comment, index({}) over size({}).", self.idx, length);
        }
        self.idx += 1;
    }

    fn eat(&mut self, token: Token){
        if &self.current_token == &token{
            self.get_next_token();
        }else{
            panic!("error in fn eat, current token: {:?}, token: {:?}", self.current_token, token);
        }
    }    

    fn program(&mut self) -> TreeNode{
        self.eat(Token::KEYWORD("PROGRAM".to_string()));
        let name = self.variable();
        self.eat(Token::SEMI);
        let block = self.block();
        self.eat(Token::DOT);
        TreeNode{node: Token::ASTNode("PROGRAM".to_string()), sub_nodes: vec![name, block]}
    }

    fn block(&mut self) -> TreeNode{
        let mut s_nodes = self.declarations();
        s_nodes.push(self.compound_statement());
        TreeNode{node: Token::ASTNode("BLOCK".to_string()), sub_nodes: s_nodes}
    }

    fn declarations(&mut self) -> Vec<TreeNode>{
        let mut nodes: Vec<TreeNode> = Vec::new();
        if self.current_token == Token::KEYWORD("VAR".to_string()){
            self.eat(Token::KEYWORD("VAR".to_string()));
            while &self.current_token == &Token::ID("a".to_string()){
                nodes.extend(self.variable_declaration().into_iter());
                self.eat(Token::SEMI);
            }
        }

        nodes
    }

    fn variable_declaration(&mut self) -> Vec<TreeNode>{
        let mut vars: Vec<String> = Vec::new();

        loop{
            let token = self.current_token.clone();
            match token{
                Token::COLON => {
                    self.get_next_token();
                    break;
                },
                Token::ID(s) => {vars.push(s)},
                Token::COMMA => {},
                _ => panic!("error in fn variable_declaration. current token: {:?}", self.current_token),
            }
            self.get_next_token();
        }

        match &self.current_token{
            Token::KEYWORD(keyword) if keyword == "INTEGER" || keyword == "REAL" => {},
            _ => panic!("error in fn variable_declaration, wrong type: {:?}", self.current_token),
        }

        let mut ret: Vec<TreeNode> = Vec::new();
        for var in vars{
            let node1 = TreeNode{node: Token::ID(var.to_string()), sub_nodes: vec![]};
            let node2 = TreeNode{node: self.current_token.clone(), sub_nodes: vec![]};
            ret.push(TreeNode{node: Token::ASTNode("VARDEC".to_string()), sub_nodes: vec![node1, node2]});
        }
        
        self.get_next_token();
        ret
    }

    fn compound_statement(&mut self) -> TreeNode{
        self.eat(Token::KEYWORD("BEGIN".to_string()));
        let nodes: Vec<TreeNode> = self.statement_list();
        self.eat(Token::KEYWORD("END".to_string()));
        TreeNode{node: Token::ASTNode("COMP".to_string()), sub_nodes: nodes}
    }

    fn statement_list(&mut self) -> Vec<TreeNode>{
        let mut nodes: Vec<TreeNode> = Vec::new();
        nodes.push(self.statement());
        while self.current_token == Token::SEMI{
            self.eat(Token::SEMI);
            nodes.push(self.statement());
        }

        if self.current_token == Token::ID("a".to_string()){
            panic!("error in fn statement list, current token: {:?}", self.current_token);
        }
        nodes
    }

    fn statement(&mut self) -> TreeNode{
        let node = match &self.current_token{
            Token::KEYWORD(keyword) if keyword.as_str() == "BEGIN" => self.compound_statement(),
            Token::ID(_) => self.assignment_statement(),
            _ => self.empty(),
        };
        node
    }

    fn assignment_statement(&mut self) -> TreeNode{
        let left = self.variable();
        let token = self.current_token.clone();
        self.eat(Token::ASSIGN);
        let right = self.expr();
        let node = TreeNode{node: token, sub_nodes: vec![left, right]};
        node
    }

    fn variable(&mut self) -> TreeNode{
        let node = TreeNode{node: self.current_token.clone(), sub_nodes: vec![]};
        self.get_next_token();
        node
    }

    fn empty(&mut self) -> TreeNode{
        TreeNode{node: Token::ASTNode("Empty".to_string()), sub_nodes: vec![]}
    }


    fn factor(&mut self) -> TreeNode{
        let token = self.current_token.clone();
        match token{
            Token::OP1(c) => {
                self.get_next_token();
                TreeNode{node: Token::UNARY(c), sub_nodes: vec![self.factor()]}
            },
            Token::INTEGER_CONST(_) | Token::REAL_CONST(_) => {
                self.get_next_token();
                TreeNode{node: token, sub_nodes: vec![]}
            },
            Token::LP => {
                self.get_next_token();
                let node = self.expr();
                self.eat(Token::RP);
                node
            },
            Token::ID(c) =>{
                self.variable()
            },
            _ => panic!("error in fn factor Token::ID branch, token: {:?}", token),
        }
    }

    fn term(&mut self) -> TreeNode{
        let mut node = self.factor();
        loop{
            match &self.current_token{
                Token::KEYWORD(keyword) if keyword.as_str() == "DIV" => {
                    let token = self.current_token.clone();
                    self.get_next_token();
                    node = TreeNode{node: token, sub_nodes: vec![node, self.factor()]};
                },
                Token::OP2(a)=> {
                    let token = self.current_token.clone();
                    self.get_next_token();
                    node = TreeNode{node: token, sub_nodes: vec![node, self.factor()]};
                }
                _ => break,
            }
        }
        node
    }

    fn expr(&mut self) -> TreeNode{
        let mut node = self.term();
       
        loop{
            match &self.current_token{
                Token::OP1(_) => {
                    let token = self.current_token.clone();
                    self.get_next_token();
                    node = TreeNode{node: token, sub_nodes: vec![node, self.term()]};
                }
                _ => break,
            }
        }
        node
    }

    pub fn parse(&mut self) -> TreeNode{
        let node = self.program();
        if self.current_token != Token::EOF{
            panic!("error in parse: do not found EOF in the end. current token: {:?}", self.current_token);
        }
        self.reset();
        node
    }

    fn reset(&mut self){
        self.idx = 0;
        self.get_next_token();
    }
}

impl fmt::Debug for TreeNode{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let mut q: VecDeque<&TreeNode> = VecDeque::new();
        q.push_back(&self);

        let mut pre_level_num;
        let mut this_level_num = 1;

        let mut ret: String = String::new();

        while !q.is_empty(){
            pre_level_num = this_level_num;
            this_level_num = 0;
            for i in 0..pre_level_num{
                let node = q.pop_front().unwrap();
                match node.node{
                    Token::ID(_) | Token::INTEGER_CONST(_) | Token::REAL_CONST(_) => 
                      ret += format!("  |{:?}(0)|  ", node.node).as_str(),
                    _ => {
                        ret += format!("  |{:?}({})|  ", node.node, node.sub_nodes.len()).as_str();
                        for n in node.sub_nodes.iter(){
                            q.push_back(n);
                            this_level_num += 1;
                        }
                    }
                }
            }
            ret.push('\n');
        }

        write!(f, "{}", ret)
    }
}




struct Visit{
    var_table: HashMap<String, Option<VarType>>,
    var_type: HashMap<String, &'static str>,
}

impl Visit{
    
    fn new() -> Self{
        Visit{var_table: HashMap::new(), var_type: HashMap::new()}
    }

    // PROGRAM BLOCK VARDEC Empty COMP
    fn visit(&mut self, root: &TreeNode){
        if root.node == Token::ASTNode("PROGRAM".to_string()) && root.sub_nodes[1].node == Token::ASTNode("BLOCK".to_string()){
            for node in root.sub_nodes[1].sub_nodes.iter(){
                self.visit_block(node);
            }
        }else{
            panic!("error in fn visit");
        }
    }

    fn visit_block(&mut self, root: &TreeNode){
        println!("visit node: {:?}", &root.node);
        match &root.node{
            Token::ASTNode(s) if s == "VARDEC" => self.visit_VarDec(root),
            Token::ASTNode(s) if s == "COMP" => self.visit_comp(root),
            _ => {panic!("error in fn visit_block, wrong AST node: {:?}", root.node)},
        }
    }

    fn visit_VarDec(&mut self, root: &TreeNode){
        //println!("visit node: {:?}", &root.node);
        let var_name = match &root.sub_nodes[0].node{
            Token::ID(s) => s.clone(),
            _ => panic!("error in fn visit_VarDec, wrong var_name. current token: {:?}", root.node),
        };

        if self.var_table.contains_key(&var_name){panic!("error: {:?} has been declared!", var_name);}

        match &root.sub_nodes[1].node{
            Token::KEYWORD(s) if s == "INTEGER" => {
                self.var_table.insert(var_name.clone(), None);
                self.var_type.insert(var_name, "INTEGER");
            },
            Token::KEYWORD(s) if s == "REAL" => {
                self.var_table.insert(var_name.clone(), None);
                self.var_type.insert(var_name, "REAL");
            },
            _ => panic!("error in fn visit_VarDec, wrong var_type. current token: {:?}", root.node),
        };
    }

    fn visit_comp(&mut self, root: &TreeNode){
        for node in root.sub_nodes.iter(){
            match &node.node{
                Token::ASSIGN => {self.visit_assign(node)},
                Token::ASTNode(s) if s == "Empty" => {},
                _ => panic!("error: {:?} has been declared!", node.node),
            }
        }
    }

    fn visit_assign(&mut self, root: &TreeNode){
        let var_name = match &root.sub_nodes[0].node{
            Token::ID(s) => s.clone(),
            _ => panic!("error in fn visit_assign, wrong var_name: {:?}", root.sub_nodes[0].node),
        };
        let var_type = *self.var_type.get(&var_name).expect(&format!("variable {} has not been declared!", var_name));

        let value = self.visit_var(&root.sub_nodes[1]);
        
        match (var_type, &value){
            ("INTEGER", VarType::Integer(n)) => {
                //let a = self.var_table.get_mut(&var_name).unwrap();
                *self.var_table.get_mut(&var_name).unwrap() = Some(VarType::Integer(*n));
            },
            ("REAL", VarType::Real(n)) => {
                *self.var_table.get_mut(&var_name).unwrap() = Some(VarType::Real(*n));
            },
            _ => panic!("type miss match, variable {}, expect {}, found {:?}", var_name, var_type, value),
        }
    }

    fn visit_var(&mut self, root: &TreeNode) -> VarType{
        //println!("visit node: {:?}", &root.node);
        match &root.node{
            Token::INTEGER_CONST(n)  => {
                VarType::Integer(*n as i64)
            },
            Token::REAL_CONST(n) => {
                VarType::Real(*n)
            },
            Token::KEYWORD(s) if s == "DIV" => {
                let left = match self.visit_var(&root.sub_nodes[0]){
                    VarType::Integer(n) => n,
                    VarType::Real(n) => n as i64,
                };
                let right = match self.visit_var(&root.sub_nodes[1]){
                    VarType::Integer(n) => n,
                    VarType::Real(n) => n as i64,
                };
                VarType::Integer(left / right)
            },
            Token::OP1(c) | Token::OP2(c) => {
                let left = self.visit_var(&root.sub_nodes[0]);
                let right = self.visit_var(&root.sub_nodes[1]);

                if left == VarType::Integer(1) && right == VarType::Integer(1){
                    let a = match left{
                        VarType::Integer(n) => n,
                        VarType::Real(n) => n as i64,
                    };

                    let b = match right{
                        VarType::Integer(n) => n,
                        VarType::Real(n) => n as i64,
                    };

                    VarType::Integer(operation(*c, a, b))
                }else{
                    let a = match left{
                        VarType::Integer(n) => n as f64,
                        VarType::Real(n) => n,
                    };

                    let b = match right{
                        VarType::Integer(n) => n as f64,
                        VarType::Real(n) => n,
                    };

                    VarType::Real(operation(*c, a, b))
                }
            },
            Token::UNARY(c) => {
                match c{
                    '+' => {self.visit_var(&root.sub_nodes[0])},
                    '-' => {
                        match self.visit_var(&root.sub_nodes[0]){
                            VarType::Integer(n) => VarType::Integer(-n),
                            VarType::Real(n) => VarType::Real(-n),
                        }
                    },
                    _ => panic!("can not recgnize UNARY: {:?}", c),
                }
                
            }
            Token::ID(s) => {
                match self.var_table.get(s).expect(&format!("varialbe {} has not been declared!", s)){
                    None => panic!("variable {} has not been init!", s),
                    Some(v) => v.clone(),
                }
            },
            _ => panic!("error in fn visit_assign, wrong var_type. {:?} = {:?}", root.sub_nodes[0].node, root.sub_nodes[1].node),
        }

    }
}

fn operation<T>(op: char, a: T, b: T,) -> T
where T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::ops::Mul<Output=T> + std::ops::Div<Output=T>{
    match op{
        '+' => {a + b},
        '-' => {a - b},
        '*' => {a * b},
        '/' => {a / b},
        _ => {panic!("wrong operation: {}", op)},
    }
}


 

fn main() {
    let input = "PROGRAM Part10AST;\n".to_string() + "VAR\n" + "   a, b : INTEGER;\n" + "   y    : REAL;\n\n" + 
        "BEGIN {Part10AST}\n" + "   a := 2;\n" + "   b := 10 * a + 10 * a DIV 4;\n" + "   y := 20 / 7 + 3.14;" + 
        "END.  {Part10AST}\n";

    let mut inp = Interpreter::from(&input);

    // loop{
    //     let token = &inp.current_token;
    //     match token{
    //         Token::EOF => break,
    //         _ => {
    //             println!("{:?}", token);
    //             inp.get_next_token();
    //         }
    //     }
    // }
    // inp.reset();

    let node = inp.parse();
    //println!("{:?}", node);

    let mut v = Visit::new();

    v.visit(&node);
    
    println!("--------------------------------");
    for (name, val) in v.var_table.iter(){
        println!("{}: {:?}", name, val.as_ref().unwrap());
    }
}
