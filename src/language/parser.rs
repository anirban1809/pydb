use super::tokenizer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current_token: usize,
}

#[derive(Debug)]
pub struct Program {
    body: Vec<Statement>,
}
#[derive(Debug)]
enum Statement {
    ExpressionStatement(Expression),
}
#[derive(Debug)]
enum Expression {
    Literal(Literal),
    Identifier(Identifier),
    UnaryExpression(Box<Expression>, Operator),
    BinaryExpression(Box<Statement>, Operator, Box<Statement>),
}
#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Exponent
}

#[derive(Debug)]
enum Literal {
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Debug)]
struct Identifier {
    name: String,
}

impl Program {
    fn new(body: Vec<Statement>) -> Self {
        Program { body }
    }
}

impl Parser {
    pub fn new(tokens: &Vec<Token>) -> Self {
        Parser {
            tokens: tokens.to_vec(),
            current_token: 0,
        }
    }

    fn not_eof(&mut self) -> bool {
        let current_token = self.tokens.get(self.current_token);
        let mut is_eof = false;
        if let Some(v) = current_token {
            is_eof = match v {
                Token::EOF => true,
                _ => false,
            };
        }

        !is_eof
    }

    fn get_current_token(&mut self) -> &Token {
        self.tokens.get(self.current_token).unwrap()
    }

    fn advance(&mut self) -> &Token {
        let current_token = self.tokens.get(self.current_token);
        self.current_token += 1;
        current_token.unwrap()
    }

    fn expect(&mut self, expected: Token, error_message: String) {
        let current_token = self.tokens.get(self.current_token).unwrap();
        self.current_token += 1;

        if *current_token != expected {
            panic!("{}", error_message.to_string());
        }
    }

    /**
     * Order of precedence
     * 
     * Assignment
     * MemberExpression
     * FunctionCallExpression
     * LogicalExpression
     * ComparisionExpression
     * AdditiveExpression
     * MultiplicativeExpression
     * UnaryExpression
     * PrimaryExpression
     */
    pub fn parse(&mut self) -> Program {
        let mut body = Vec::new();
        while self.not_eof() {
            body.push(self.parse_statement());
        }

        Program { body }
    }

    fn parse_statement(&mut self) -> Statement {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Statement {
        self.parse_additive_expression()
    }

    fn parse_multiplicative_expression(&mut self) -> Statement{
        let mut left = self.parse_primary();

        while *self.get_current_token() == Token::Star || *self.get_current_token() == Token::Slash {
            let operator = match self.advance() {
                Token::Star => Operator::Multiply,
                Token::Slash => Operator::Divide,
                Token::Percent => Operator::Modulus,
                Token::DoubleStar => Operator::Exponent,
                _ => panic!("Invalid operator")
            };

            let right = self.parse_primary();
            left = Statement::ExpressionStatement(Expression::BinaryExpression(Box::new(left), operator, Box::new(right)))
        }  

        left 
    }

    fn parse_additive_expression(&mut self) -> Statement {
        let mut left = self.parse_multiplicative_expression();

        while *self.get_current_token() == Token::Plus || *self.get_current_token() == Token::Minus {
            let operator = match self.advance() {
                Token::Plus => Operator::Add,
                Token::Minus => Operator::Subtract,
                _ => panic!("Invalid operator")
            };

            let right = self.parse_multiplicative_expression();
            left = Statement::ExpressionStatement(Expression::BinaryExpression(Box::new(left), operator, Box::new(right)))
        }  

        left 
    }

    fn parse_primary(&mut self) -> Statement {
        let current_token = self.advance();

        println!("Token : {:?}", current_token);

        let node = match current_token {
            Token::Identifier(v) => {
                Statement::ExpressionStatement(Expression::Identifier(Identifier {
                    name: v.to_string(),
                }))
            }
            Token::Integer(v) => {
                Statement::ExpressionStatement(Expression::Literal(Literal::Int(*v)))
            }
            Token::Float(v) => {
                Statement::ExpressionStatement(Expression::Literal(Literal::Float(*v)))
            }
            Token::StringLiteral(v) => {
                Statement::ExpressionStatement(Expression::Literal(Literal::String(v.to_string())))
            }
            Token::LParen => {
                let value = self.parse_expression();
                self.expect(Token::RParen, "Error: missing closing parenthesis".to_string());
                value
            },
            _ => panic!("Undefined Symbol encountered while parsing, {:?}", current_token),
        };

        node
    }
}
