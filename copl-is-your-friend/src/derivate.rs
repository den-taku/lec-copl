use crate::parse::Ast::{self, *};
use crate::parse::*;
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
