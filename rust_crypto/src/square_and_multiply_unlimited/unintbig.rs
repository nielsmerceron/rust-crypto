//The purpose of this code is to define a way to show big number (>64bit)
pub mod unintbig {

    //use std::vec::Vec;

    //The purpose of this code is to take a hexa string and convert it to a binary vect
    pub fn hexa_to_uint(input: &str){
        //
        for i in input.chars() {
            if(i>='0' && i<='9') || (i>='A' && i<='Z') || (i>='a' && i<='z'){
            }
            else {
                println!("wrong input : {}",input);
                return;}
        }
        println!("correct input : {}",input);

    }

    //The purpose of this code is to take a hexa string and convert it to a binary vect
    pub fn number_to_uint(input: &str){
        //
        for i in input.chars() {
            if i>='0' && i<='9'{}
            else {
                println!("wrong input : {}",input);
                return;
            }
        }
        println!("correct input : {}",input);

    }
}