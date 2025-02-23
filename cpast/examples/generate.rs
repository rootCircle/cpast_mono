use clex_gen::generator;

fn main() {
    println!("{}", generator("N[10,99] S[4,4,'U']".to_owned()).unwrap());
}
