use cpast::generator;

fn main() {
    println!("{}", generator("N[10,99] S[4,'U']".to_owned()).unwrap());
}
