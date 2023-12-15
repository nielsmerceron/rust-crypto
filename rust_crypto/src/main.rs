mod square_and_multiply_i64;
use square_and_multiply_i64::square_and_multiply::sam_i64;

fn main() {
    let tmp : u64 = sam_i64(5,1000,44);

    println!("résultat expo mod:{tmp}");
}
