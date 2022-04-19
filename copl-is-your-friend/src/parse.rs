use Ast::*;

/// denotes object language and judgment
#[derive(Debug, Clone)]
pub enum Ast {
    /// S(n), Z
    Num(u32),
    /// n1 plus n2 is n3
    Plus(Box<Ast>, Box<Ast>, Box<Ast>),
    /// n1 times n2 is n3
    Times(Box<Ast>, Box<Ast>, Box<Ast>),
}

pub fn is_zero(a: &Ast) -> bool {
    if let Num(n) = a {
        n == &0
    } else {
        false
    }
}

fn succ(ast: Ast) -> Ast {
    match ast {
        Num(n) => Num(n + 1),
        _ => unreachable!(),
    }
}

pub fn parse(input: String) -> Ast {
    let mut stream = input.chars().peekable();
    let mut left = Num(0);
    let mut right = Num(0);
    let mut ans = Num(0);
    let mut ret = Num(0);
    while let Some(c) = stream.next() {
        match c {
            'S' => left = succ(left),
            'p' => {
                while let Some(c) = stream.next() {
                    match c {
                        'S' => right = succ(right),
                        'i' => {
                            while let Some(c) = stream.next() {
                                match c {
                                    'S' => ans = succ(ans),
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
                ret = Plus(Box::new(left), Box::new(right), Box::new(ans));
                break;
            }
            't' => {
                stream.next();
                while let Some(c) = stream.next() {
                    match c {
                        'S' => right = succ(right),
                        'i' => {
                            while let Some(c) = stream.next() {
                                match c {
                                    'S' => ans = succ(ans),
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
                ret = Times(Box::new(left), Box::new(right), Box::new(ans));
                break;
            }
            _ => {}
        }
    }
    ret
}
