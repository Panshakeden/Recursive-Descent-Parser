#[derive(Debug,PartialEq,Clone)]
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

#[derive(Debug)]

struct Parser<'a> {
    tokens: &'a [Token], 
    pos: usize,
}

impl<'a> Parser<'a> {
    fn expression(&mut self) ->f64  {
            self.equality()
    }


    fn equality(&mut self) ->f64{
        let mut left_value = self.comparison();
        while let Some(op)=self.match_token(&[Token::EqualDouble, Token::NotEqual]){
            let right_value= self.comparison();
            
            left_value= match op {
                Token::EqualDouble => (left_value == right_value) as i32 as f64,
                Token::NotEqual => (left_value != right_value)as i32 as f64,
                _ => unreachable!(),
            };
        }
      left_value
        
    }

    fn comparison(&mut self) ->f64 {
        let mut left_value = self.term();
        while let Some(op)= self.match_token(&[Token::Greater,Token::Less,Token::GreaterEqual,Token::LessEqual]){
            let right_value = self.term();
            left_value = match op{
                Token::Greater => (left_value > right_value) as i32 as f64,
                Token::Less => (left_value < right_value) as i32 as f64,
                Token::LessEqual => (left_value <= right_value) as i32 as f64,
                Token::GreaterEqual => (left_value >= right_value) as i32 as f64,
                _ => unreachable! (),
            }
        }
        left_value
         
    }

    fn term(&mut self) -> f64 {
        let mut left_value = self.factor();
         while let Some(op) = self.match_token(&[Token::Minus,Token::Plus]){
            let right_value=self.factor();
            left_value= match op{
            Token::Minus => (left_value - right_value) as f64,
            Token::Plus => (left_value - right_value) as f64,
            _ => unreachable!(),
            }
         }
         left_value
    }

    fn factor(&mut self) ->f64{
      let left_value= self.unary();
      while let Some(op) = self.match_token(&[Token::Multply,Token::Divide]){
       let right_value= self.unary();

       let left_value = match op{
        Token::Multply => left_value * right_value,
        Token::Divide => left_value / right_value,
        _ => unreachable!(),
       };
      }
      left_value
    }

    fn unary(&mut self) -> f64{
        while let Some(op) = self.match_token(&[Token::Bang,Token::Minus]){
            let right_value= self.unary();

            return match op{
                Token::Bang => if right_value ==0.0 {1.0} else{0.0},
                Token::Minus => -right_value,
                _ => unreachable!(),
            };
        }
        self.primary()

    }

    fn primary(&mut self) -> f64{
        match self.consume(){
        Some(Token::Number(n)) => n,
        Some(Token::True) => 1.0,
        Some(Token::False) => 0.0,
        Some(Token::Nil) => 0.0,
        Some(Token::LeftParen) => {
            let expr = self.expression();
            self.expect(Token::RightParen);
            expr
        },
        _ => panic!("Unexpected token"),

        }
    }

    fn consume(&mut self)-> Option<Token>{
        if self.pos < self.tokens.len() {
            let token = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(token)
        } else {
            None
        }
    }

    fn expect(&mut self, token: Token) {
        if self.consume() != Some(token) {
            panic!("Expected token not found!");
        }
    }
    


    fn match_token(&mut self, expected: &[Token]) -> Option<Token> {
        if self.pos < self.tokens.len() {
            for token in expected {
                if &self.tokens[self.pos] == token {
                    self.pos += 1;
                    return Some(token.clone());
                }
            }
        }
        None
    }

}




fn main() {
    let tokens = vec![
        Token::Number(8.0),
        Token::Minus,
        Token::Number(3.0),
        // Token::LeftParen,
        // Token::Number(8.0),
        //Token::Multiply
        // Token::Number(9.0),
        // Token::RightParen,
    ];
    
    let mut parser = Parser { tokens: &tokens, pos: 0 };
    println!("Parser: {:?}", parser);
    let result = parser.expression();
    println!("Result: {}", result); 
}
