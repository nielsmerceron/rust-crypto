mod unintbig;

//The purpose of this code is to perform a square and multiply on integers larger than u64.
pub mod square_and_multiply_unlimited {

    use crate::square_and_multiply_unlimited::unintbig::unintbig::{hexa_to_uint,number_to_uint};

    fn acceptchar(input: &str) -> bool{
        for i in input.chars() {
            if(i>='0' && i<='9') || (i>='A' && i<='Z') || (i>='a' && i<='z'){}
            else {return false}
        }
        return true
    }
    // expo modu (x puiss y) mod n
    pub fn samu(inputa: &str, inputn: &str, inputpuiss: &str) {
        
        if acceptchar(inputa) && acceptchar(inputn) && acceptchar(inputpuiss){
            println!("input correct")
        }
        else{
            println!("input incorrect, use [0;9] U [a;z] U [A;Z]")
        }

        hexa_to_uint("'");
        number_to_uint(inputn);
        
    }
}