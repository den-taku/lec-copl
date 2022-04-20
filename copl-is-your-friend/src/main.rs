use copl_is_your_friend::nat;

fn main() {
    let input = std::env::args().nth(1).unwrap();
    println!("{}", nat::nat_plus_times(input));
}
