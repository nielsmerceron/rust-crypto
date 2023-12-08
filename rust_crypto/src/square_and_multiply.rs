use std::vec::Vec;

//change a signed integer into a binary array
fn integer_64_to_bit(y: u64) -> Vec<i8>{

    
    let mut tmp_y : u64 = y;
    let mut binar: Vec<i8> = Vec::new();  
  

    while tmp_y!=0 {
        if (tmp_y%2)!=0{
            tmp_y=tmp_y-1;
            tmp_y=tmp_y/2;
            binar.push(1);
        }
        else{
           
            tmp_y=tmp_y/2;
            binar.push(0);
        }
        
    }
    return binar;
}

// expo modu (x puiss y) mod n
pub fn sam(x: u64, y: u64 , n: u64) -> u64{
   
    let mut binar: Vec<i8> = Vec::new();
    binar= integer_64_to_bit(y);
    println!("binaire de la puissance:{:?}",binar);
    
    return 10;
}