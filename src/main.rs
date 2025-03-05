enum Token{
    Number(f64),
    Minus,
    Plus,
    Multply,
    Divide,
    EqualDouble,
    NotEqual,
    Greater,
    GreaterEqual,
    True,
    False,
    LeftParen,
    RightParen,
    Bang,
    Less,
    LessEqual,
    Nil,
}


struct Parser<'a> {
    tokens: &'a [Token], 
    pos: usize,
}

impl<'a> Parser<'a> {
    fn expr(&mut self) -> i32 {
        let mut value = self.term();
        while self.match_token('+') {
            value += self.term(); 
        }
        value
    }


    fn term(&mut self) -> i32 {
        let mut value = self.equality();
        while self.match_token('*') {
            value +=self.equality();
        }
        value
    }

    fn equality(&mut self) ->i32{
        let mut value = self.factor();
        while self.match_token('-'){
            value +=self.factor();
        }
        value
    }

    fn factor(&mut self) ->i32{
        if self.match_token('('){
            let value =self.expr();
            self.match_token(')');
            value
        }else{
            self.number()
        }
    }

    fn number(&mut self) -> i32{
        let digits =self.tokens[self.pos].to_digit(10).unwrap();
        self.pos +=1;

        digits as i32
    }


    fn match_token(&mut self, expected: char) -> bool {
        if self.pos < self.tokens.len() && self.tokens[self.pos] == expected {
            self.pos += 1;
            true
        } else {
            false
        }
    }

}




fn main() {
    println!("Hello, world!");
}
