use crate::isas::ISA;

pub(super) fn eval(cpu: &mut impl ISA, exp: &str) -> Option<u64> {
    let tokens = tokenize(exp)?;
    eval_tokens(cpu, &tokens, 0, tokens.len() - 1)
}

fn eval_tokens(cpu: &mut impl ISA, tokens: &[Token], start: usize, end: usize) -> Option<u64> {
    if tokens[start] == Token::Lparen && tokens[end] == Token::Rparen {
        return eval_tokens(cpu, tokens, start + 1, end - 1);
    }

    if start == end {
        match &tokens[start] {
            Token::Number(n) => Some(*n),
            Token::Register(r) => {
                if r == "pc" {
                    return Some(cpu.pc() as u64);
                }
                cpu.read_register_by_name(r).map(|v| v as u64)
            }
            _ => None,
        }
    } else {
        let i = find_delimiter(tokens, start, end)?;
        match tokens[i] {
            Token::Operator(op) => {
                if let Op::Star = op {
                    let addr = eval_tokens(cpu, tokens, i + 1, end)?;
                    return cpu.load_mem(addr as u32, 4).map(|v| v as u64);
                }
                let left = eval_tokens(cpu, tokens, start, i - 1)?;
                let right = eval_tokens(cpu, tokens, i + 1, end)?;
                match op {
                    Op::Add => Some(left + right),
                    Op::Sub => Some(left - right),
                    Op::Mul => Some(left * right),
                    Op::Div => Some(left / right),
                    Op::And => Some((left != 0) as u64 & (right != 0) as u64),
                    Op::BitAnd => Some(left & right),
                    Op::Or => Some((left != 0) as u64 | (right != 0) as u64),
                    Op::BitOr => Some(left | right),
                    Op::Eq => Some((left == right) as u64),
                    Op::Ne => Some((left != right) as u64),
                    Op::Star => unreachable!(),
                }
            }
            _ => todo!("eval_tokens: {:?}", tokens[i]),
        }
    }
}

fn tokenize(exp: &str) -> Option<Vec<Token>> {
    let mut tokens = vec![];
    let mut chars = exp.chars().peekable();
    while chars.peek().is_some() {
        match chars.next() {
            Some(c) if c.is_ascii_whitespace() => continue,
            Some(c) if c.is_ascii_digit() => {
                if c == '0' && chars.peek() == Some(&'x') {
                    // hex
                    chars.next();
                    let mut num = 0;
                    while let Some(c) = chars.peek() {
                        if c.is_alphanumeric() {
                            num = num * 16 + chars.next().unwrap().to_digit(16)? as u64;
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(num));
                } else {
                    // decmial
                    let mut num = c.to_digit(10).unwrap() as u64;
                    while let Some(c) = chars.peek() {
                        if c.is_ascii_digit() {
                            num = num * 10 + chars.next().unwrap().to_digit(10)? as u64;
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(num));
                }
            }
            Some('$') => {
                let mut reg = String::new();
                while let Some(c) = chars.peek() {
                    if c.is_alphanumeric() {
                        reg.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Register(reg));
            }
            Some('+') => tokens.push(Token::Operator(Op::Add)),
            Some('-') => tokens.push(Token::Operator(Op::Sub)),
            Some('*') => tokens.push(Token::Operator(Op::Mul)),
            Some('/') => tokens.push(Token::Operator(Op::Div)),
            Some('&') => match chars.peek() {
                Some(c) if *c == '&' => {
                    chars.next();
                    tokens.push(Token::Operator(Op::And));
                }
                _ => tokens.push(Token::Operator(Op::BitAnd)),
            },
            Some('|') => match chars.peek() {
                Some(c) if *c == '|' => {
                    chars.next();
                    tokens.push(Token::Operator(Op::Or));
                }
                _ => tokens.push(Token::Operator(Op::BitOr)),
            },
            Some('=') => match chars.peek() {
                Some(c) if *c == '=' => {
                    chars.next();
                    tokens.push(Token::Operator(Op::Eq));
                }
                _ => return None,
            },
            Some('!') => match chars.peek() {
                Some(c) if *c == '=' => {
                    chars.next();
                    tokens.push(Token::Operator(Op::Ne));
                }
                _ => return None,
            },
            Some('(') => tokens.push(Token::Lparen),
            Some(')') => tokens.push(Token::Rparen),
            _ => return None,
        }
    }
    for i in 0..tokens.len() {
        if let Token::Operator(Op::Mul) = tokens[i] {
            if i == 0 {
                tokens[i] = Token::Operator(Op::Star);
            } else {
                match tokens[i - 1] {
                    Token::Lparen | Token::Operator(_) => tokens[i] = Token::Operator(Op::Star),
                    _ => (),
                }
            }
        }
    }
    Some(tokens)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Eq,
    Ne,
    BitAnd,
    BitOr,
    Star,
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(u64),
    Operator(Op),
    Lparen,
    Rparen,
    Register(String),
}

impl Op {
    pub fn precedence(&self) -> u8 {
        match self {
            Op::And | Op::Or => 1,
            Op::Eq | Op::Ne => 2,
            Op::BitAnd => 3,
            Op::BitOr => 4,
            Op::Add | Op::Sub => 5,
            Op::Mul | Op::Div => 6,
            Op::Star => 7,
        }
    }
}

fn find_delimiter(exp: &[Token], start: usize, end: usize) -> Option<usize> {
    if start >= end {
        return None;
    }
    let mut i = start;
    let mut dele = None;
    let mut precedence = std::u8::MAX;
    let mut depth = 0;
    while i < end {
        match exp[i] {
            Token::Lparen => depth += 1,
            Token::Rparen => {
                depth -= 1;
                if depth == 0 {
                    return None;
                }
            }
            Token::Operator(op) => {
                if depth == 0 && op.precedence() <= precedence {
                    dele = Some(i);
                    precedence = op.precedence();
                }
            }
            _ => (),
        }
        i += 1;
    }
    dele
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tokenize() {
        let exp = "1+2 * $x0";
        let tokens = tokenize(exp).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(1),
                Token::Operator(Op::Add),
                Token::Number(2),
                Token::Operator(Op::Mul),
                Token::Register("x0".to_string()),
            ]
        );

        let exp = "1+2 * *$pc";
        let tokens = tokenize(exp).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(1),
                Token::Operator(Op::Add),
                Token::Number(2),
                Token::Operator(Op::Mul),
                Token::Operator(Op::Star),
                Token::Register("pc".to_string()),
            ]
        );

        let exp = "0xfff";
        let tokens = tokenize(exp).unwrap();
        assert_eq!(tokens, vec![Token::Number(0xfff),]);

        let exp = "((*0xfffff000) + 0x1000) && ($pc == 0x1000)";
        let tokens = tokenize(exp).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Lparen,
                Token::Lparen,
                Token::Operator(Op::Star),
                Token::Number(0xfffff000),
                Token::Rparen,
                Token::Operator(Op::Add),
                Token::Number(0x1000),
                Token::Rparen,
                Token::Operator(Op::And),
                Token::Lparen,
                Token::Register("pc".to_string()),
                Token::Operator(Op::Eq),
                Token::Number(0x1000),
                Token::Rparen,
            ]
        );
    }

    #[test]
    fn test_eval() {
        use crate::isas::riscv::RV32CPU;
        use crate::isas::RegisterModel;

        let mut cpu = RV32CPU::default();

        cpu.update_pc(0x1000);
        let exp = "1+2 * $pc";
        let value = eval(&mut cpu, exp).unwrap();
        assert_eq!(value, 0x2001);

        let exp = "$pc == 0x1000";
        let value = eval(&mut cpu, exp).unwrap();
        assert_eq!(value, 1);
    }
}
