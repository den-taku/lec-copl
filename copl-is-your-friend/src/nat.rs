use crate::nat::derivate::derivate;
use crate::nat::parse::parse;

pub fn nat_plus_times(input: String) -> String {
    let ast = parse(input);
    let ans = derivate(ast);
    ans
}

mod parse {
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
}

mod derivate {
    use crate::nat::parse::Ast::{self, *};
    use crate::nat::parse::*;
    use std::fmt::Write;

    pub fn derivate(ast: Ast) -> String {
        let mut buffer = String::new();
        derivate_rec(0, ast, &mut buffer);
        buffer
    }

    fn succ_zero(n: u32) -> String {
        if n == 0 {
            "Z".to_string()
        } else {
            format!("S({})", succ_zero(n - 1))
        }
    }

    fn derivate_rec(position: usize, ast: Ast, buf: &mut String) {
        match ast {
            Plus(left, right, ans) => {
                if is_zero(&left) {
                    // P-Zero
                    let right = if let Num(right) = *right {
                        right
                    } else {
                        unreachable!()
                    };
                    let ans = if let Num(ans) = *ans {
                        ans
                    } else {
                        unreachable!()
                    };
                    write!(
                        buf,
                        "{}Z plus {} is {} by P-Zero {{}};\n",
                        "\t".chars().cycle().take(position).collect::<String>(),
                        succ_zero(right),
                        succ_zero(ans)
                    )
                    .unwrap();
                } else {
                    // P-Succ
                    let left = if let Num(left) = *left {
                        left
                    } else {
                        unreachable!()
                    };
                    let right = if let Num(right) = *right {
                        right
                    } else {
                        unreachable!()
                    };
                    let ans = if let Num(ans) = *ans {
                        ans
                    } else {
                        unreachable!()
                    };
                    write!(
                        buf,
                        "{}{} plus {} is {} by P-Succ {{\n",
                        "\t".chars().cycle().take(position).collect::<String>(),
                        succ_zero(left),
                        succ_zero(right),
                        succ_zero(ans)
                    )
                    .unwrap();
                    derivate_rec(
                        position + 1,
                        Plus(
                            Box::new(Num(left - 1)),
                            Box::new(Num(right)),
                            Box::new(Num(ans - 1)),
                        ),
                        buf,
                    );
                    write!(
                        buf,
                        "{}}};\n",
                        "\t".chars().cycle().take(position).collect::<String>(),
                    )
                    .unwrap();
                }
            }
            Times(left, right, ans) => {
                if is_zero(&left) {
                    // T-Zero
                    let right = if let Num(right) = *right {
                        right
                    } else {
                        unreachable!()
                    };
                    write!(
                        buf,
                        "{}Z times {} is Z by T-Zero {{}};\n",
                        "\t".chars().cycle().take(position).collect::<String>(),
                        succ_zero(right),
                    )
                    .unwrap();
                } else {
                    // T-Succ
                    let left = if let Num(left) = *left {
                        left
                    } else {
                        unreachable!()
                    };
                    let right = if let Num(right) = *right {
                        right
                    } else {
                        unreachable!()
                    };
                    let ans = if let Num(ans) = *ans {
                        ans
                    } else {
                        unreachable!()
                    };
                    write!(
                        buf,
                        "{}{} times {} is {} by T-Succ {{\n",
                        "\t".chars().cycle().take(position).collect::<String>(),
                        succ_zero(left),
                        succ_zero(right),
                        succ_zero(left * right)
                    )
                    .unwrap();
                    // n1 times n2 is n3
                    derivate_rec(
                        position + 1,
                        Times(
                            Box::new(Num(left - 1)),
                            Box::new(Num(right)),
                            Box::new(Num((left - 1) * right)),
                        ),
                        buf,
                    );
                    // n2 plus n3 is n4
                    derivate_rec(
                        position + 1,
                        Plus(
                            Box::new(Num(right)),
                            Box::new(Num((left - 1) * right)),
                            Box::new(Num(ans)),
                        ),
                        buf,
                    );
                    write!(
                        buf,
                        "{}}};\n",
                        "\t".chars().cycle().take(position).collect::<String>(),
                    )
                    .unwrap();
                }
            }
            _ => unreachable!(),
        }
    }
}
