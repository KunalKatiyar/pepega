pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut crate::interpreter::interpreter::Interpreter, arguments: Vec<crate::lexer::token::LiteralValue>) -> Result<crate::lexer::token::LiteralValue, String>;
}