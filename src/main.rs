struct Parser<'a> {
    tokens: &'a [char], 
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
