#[derive(Debug)]
enum Token{
    Number(f64),
    Minus,
    Plus,
    Multply,
    Divide,
    modulus,
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
    fn expr(&mut self) ->i32  {
            self.equality();
    }


    fn term(&mut self) -> i32 {
        let mut value = self.equality();
        while self.match_token('*') {
            value +=self.equality();
        }
        value
    }

    fn equality(&mut self) ->i32{
        let mut left_value = self.comparison();
        while let Some(op)=self.match_token(&[Token::EqualDouble, Token::NotEqual]){
            let right_value= self.comparison();
            
            left_value= match op {
                Token::EqualDouble => (left_value ==right_value) as i32,
                Token::NotEqual => (left_value != right_value) as i32,
                _ => (),
            };
        }
      left_value
        
    }

    fn comparison() ->i32 {
        let mut left_value = self.term();
        while let Some(op)= self.match_token(&[Token::Greater,Token::Less,Token::GreaterEqual,LessEqual]){
            let right_value = self.term();
            left_value = match op{
                Token::Greater => (left_value > right_value) as i32,
                Token::Less => (left_value < right_value) as i32,
                Token::LessEqual => (left_value <= right_value) as i32,
                Token::GreaterEqual => (left_value >= right_value) as i32,
                _ => ()
            }
        }
        left_value
         
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
