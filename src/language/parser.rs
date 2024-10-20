use std::iter::empty;

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
    IfStatement {
        test: Box<Statement>,
        body: Vec<Box<Statement>>,
    },
    FunctionDefinitionStatement {
        id: Identifier,
        params: Vec<Identifier>,
        body: Vec<Box<Statement>>,
    },
    ExpressionStatement(Expression),
}
#[derive(Debug)]
struct IfStatement {
    test: Box<Statement>,
    body: Vec<Box<Statement>>,
}

#[derive(Debug)]
struct FunctionDefinitionStatement {
    id: Identifier,
    params: Vec<Identifier>,
    body: Vec<Box<Statement>>,
}

#[derive(Debug)]
enum Expression {
    Literal(Literal),
    Identifier(Identifier),
    UnaryExpression(Box<Expression>, Operator),
    BinaryExpression(Box<Statement>, Operator, Box<Statement>),
    FunctionCallExpression(Identifier, Vec<Box<Statement>>),
    AssignmentExpression(Identifier, Box<Statement>),
}
#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Exponent,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Equality,
    NotEquals,
    And,
    Or,
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

    fn get_current_token(&self) -> &Token {
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
     * Blocks (if, for, while, do-while, class)
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
            body.push(self.parse_block_statement());
        }

        Program { body }
    }

    fn parse_if_statement(&mut self) -> Statement {
        let mut body = Vec::new();
        self.advance(); //consume if
        self.advance(); //consume lparen
        let test = Box::new(self.parse_expression());
        self.expect(
            Token::RParen,
            "Missing closing parentheses in if statement".to_string(),
        );
        self.expect(
            Token::Colon,
            "Expected : after if statement definition".to_string(),
        );

        body = self.parse_block_statement_body();
        return Statement::IfStatement { test, body };
    }

    fn parse_block_statement_body(&mut self) -> Vec<Box<Statement>> {
        let mut body = Vec::new();
        self.advance(); //consume newline
        self.advance(); //consume indent
        while let Some(token) = Some(self.get_current_token()) {
            println!("Token :: {:?}", token);
            match token {
                Token::EOF => {
                    break;
                }
                Token::Newline => {
                    if *self.tokens.get(self.current_token + 1).unwrap() == Token::Dedent {
                        self.advance(); //consume newline
                                        //consume dedent
                        while *self.get_current_token() == Token::Dedent {
                            self.advance();
                        }
                        break;
                    }
                    self.advance();
                }

                _ => {
                    let statement = Box::new(self.parse_block_statement());
                    body.push(statement)
                }
            }
        }

        body
    }

    fn parse_function_definition_statement(&mut self) -> Statement {
        //initialize empty identifier to set the function name later
        let mut function_name = Identifier {
            name: "".to_string(),
        };
        let mut function_params = Vec::new();
        let mut function_body = Vec::new();

        self.advance(); //consume def keyword
        if let Some(token) = Some(self.get_current_token()) {
            function_name = match token {
                Token::Identifier(e) => Identifier {
                    name: e.to_string(),
                },
                _ => panic!("Expected function name after def"),
            };
        }
        self.advance(); //consume function name identifier
        self.advance(); //consume lparen

        if *self.tokens.get(self.current_token + 1).unwrap() == Token::RParen {
            //no arguments defined in the function
            self.advance(); //consume rparen
            self.advance(); //consume colon

            function_body = self.parse_block_statement_body();

            return Statement::FunctionDefinitionStatement {
                id: function_name,
                params: function_params,
                body: function_body,
            };
        }

        while let Some(token) = Some(self.advance()) {
            println!("Function param token -> {:?}", token);
            match token {
                Token::Identifier(e) => function_params.push({
                    Identifier {
                        name: e.to_string(),
                    }
                }),
                Token::Comma => {
                    continue;
                }
                Token::RParen => {
                    break;
                }
                _ => panic!("error"),
            };
        }
        self.advance(); //consume colon
        function_body = self.parse_block_statement_body();

        Statement::FunctionDefinitionStatement {
            id: function_name,
            params: function_params,
            body: function_body,
        }
    }

    fn parse_block_statement(&mut self) -> Statement {
        let current_token = self.get_current_token();
        //parse if statement
        if *current_token == Token::IfKeyword {
            return self.parse_if_statement();
        }

        if *current_token == Token::DefKeyword {
            return self.parse_function_definition_statement();
        }

        self.parse_statement()
    }

    fn parse_statement(&mut self) -> Statement {
        self.parse_expression()
    }

    fn parse_function_call_expression(&mut self) -> Statement {
        let current_token = self.get_current_token().to_owned();

        let statement = match current_token {
            Token::Identifier(e) => {
                if *self.tokens.get(self.current_token + 1).unwrap() == Token::LParen {
                    self.advance(); //consume the identifier
                    self.advance(); //consume the lparen
                    Statement::ExpressionStatement(Expression::FunctionCallExpression(
                        Identifier {
                            name: e.to_string(),
                        },
                        self.parse_function_arguments(),
                    ))
                } else {
                    self.parse_logical_expression()
                }
            }
            _ => self.parse_logical_expression(),
        };

        statement
    }

    fn parse_function_arguments(&mut self) -> Vec<Box<Statement>> {
        let mut arguments = Vec::new();

        if *self.get_current_token() == Token::RParen {
            self.advance(); //consume rparen
            return arguments;
        }

        loop {
            let argument = Box::new(self.parse_expression());
            arguments.push(argument);

            if *self.get_current_token() == Token::Comma {
                self.advance();
            }

            if *self.get_current_token() == Token::RParen {
                self.advance();
                break;
            }
        }

        arguments
    }

    fn parse_expression(&mut self) -> Statement {
        self.parse_logical_expression()
    }

    fn parse_multiplicative_expression(&mut self) -> Statement {
        let mut left = self.parse_primary();

        while *self.get_current_token() == Token::Star
            || *self.get_current_token() == Token::Slash
            || *self.get_current_token() == Token::DoubleStar
        {
            let operator = match self.advance() {
                Token::Star => Operator::Multiply,
                Token::Slash => Operator::Divide,
                Token::Percent => Operator::Modulus,
                Token::DoubleStar => Operator::Exponent,
                _ => panic!("Invalid operator"),
            };

            let right = self.parse_primary();
            left = Statement::ExpressionStatement(Expression::BinaryExpression(
                Box::new(left),
                operator,
                Box::new(right),
            ))
        }

        left
    }

    fn parse_additive_expression(&mut self) -> Statement {
        let mut left = self.parse_multiplicative_expression();

        while *self.get_current_token() == Token::Plus || *self.get_current_token() == Token::Minus
        {
            let operator = match self.advance() {
                Token::Plus => Operator::Add,
                Token::Minus => Operator::Subtract,
                _ => panic!("Invalid operator"),
            };

            let right = self.parse_multiplicative_expression();
            left = Statement::ExpressionStatement(Expression::BinaryExpression(
                Box::new(left),
                operator,
                Box::new(right),
            ))
        }

        left
    }

    fn parse_comparision_expression(&mut self) -> Statement {
        let mut left = self.parse_additive_expression();

        while *self.get_current_token() == Token::GreaterThan
            || *self.get_current_token() == Token::LessThan
            || *self.get_current_token() == Token::GreaterThanOrEqual
            || *self.get_current_token() == Token::LessThanOrEqual
        {
            let operator = match self.advance() {
                Token::GreaterThan => Operator::GreaterThan,
                Token::LessThan => Operator::LessThan,
                Token::GreaterThanOrEqual => Operator::GreaterThanOrEqual,
                Token::LessThanOrEqual => Operator::LessThanOrEqual,
                _ => panic!("Invalid operator"),
            };

            let right = self.parse_additive_expression();
            left = Statement::ExpressionStatement(Expression::BinaryExpression(
                Box::new(left),
                operator,
                Box::new(right),
            ))
        }

        left
    }

    fn parse_logical_expression(&mut self) -> Statement {
        let mut left = self.parse_comparision_expression();

        while *self.get_current_token() == Token::And
            || *self.get_current_token() == Token::Or
            || *self.get_current_token() == Token::DoubleEquals
            || *self.get_current_token() == Token::NotEquals
        {
            let operator = match self.advance() {
                Token::And => Operator::And,
                Token::Or => Operator::Or,
                Token::DoubleEquals => Operator::Equality,
                Token::NotEquals => Operator::NotEquals,
                _ => panic!("Invalid operator"),
            };

            let right = self.parse_comparision_expression();
            left = Statement::ExpressionStatement(Expression::BinaryExpression(
                Box::new(left),
                operator,
                Box::new(right),
            ))
        }

        left
    }

    fn parse_primary(&mut self) -> Statement {
        let current_token = self.advance().to_owned();

        let node = match current_token.to_owned() {
            Token::Identifier(v) => {
                //case to check for function call expression
                if *self.get_current_token() == Token::LParen {
                    self.advance(); //consume the lparen
                    return Statement::ExpressionStatement(Expression::FunctionCallExpression(
                        Identifier {
                            name: v.to_string(),
                        },
                        self.parse_function_arguments(),
                    ));
                } else
                //case to check for assignment expression
                if *self.get_current_token() == Token::Equals {
                    self.advance(); //consume equals
                    return Statement::ExpressionStatement(Expression::AssignmentExpression(
                        Identifier {
                            name: v.to_string(),
                        },
                        Box::new(self.parse_expression()),
                    ));
                } else {
                    return Statement::ExpressionStatement(Expression::Identifier(Identifier {
                        name: v.to_string(),
                    }));
                }
            }
            Token::Integer(v) => {
                Statement::ExpressionStatement(Expression::Literal(Literal::Int(v)))
            }
            Token::Float(v) => {
                Statement::ExpressionStatement(Expression::Literal(Literal::Float(v)))
            }
            Token::StringLiteral(v) => {
                Statement::ExpressionStatement(Expression::Literal(Literal::String(v.to_string())))
            }
            Token::LParen => {
                let value = self.parse_expression();
                self.expect(
                    Token::RParen,
                    "Error: missing closing parenthesis".to_string(),
                );
                value
            }
            Token::Indent => self.parse_block_statement(),
            Token::Newline => self.parse_block_statement(),
            _ => panic!(
                "Undefined Symbol encountered while parsing, {:?}",
                current_token
            ),
        };

        node
    }
}
