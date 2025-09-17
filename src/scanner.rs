use crate::token::Token;

pub struct Scanner;

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {}
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        vec![]
    }
}
