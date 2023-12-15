mod square_and_multiply_u64;
use square_and_multiply_u64::square_and_multiply_u64::sam_u64;

fn main() {
    let tmp : u64 = sam_u64(5,1000,44);

    println!("résultat expo mod:{tmp}");
}
