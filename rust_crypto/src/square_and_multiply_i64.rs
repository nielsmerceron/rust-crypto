pub mod square_and_multiply {

    use std::vec::Vec;

    //change a signed integer into a binary array
    fn integer_64_to_bit(y: u64) -> Vec<i8> {
        let mut tmp_y: u64 = y;
        let mut binar: Vec<i8> = Vec::new();

        while tmp_y != 0 {
            if (tmp_y % 2) != 0 {
                tmp_y = tmp_y - 1;
                tmp_y = tmp_y / 2;
                binar.push(1);
            } else {
                tmp_y = tmp_y / 2;
                binar.push(0);
            }
        }
        return binar;
    }

    // square and multiply's intermediary operation
    fn operation(x: u64, binary_value: i8, n: u64, tmp: u64) -> u64 {
        let mut tmpz = (tmp * tmp) % n;

        if binary_value == 1 {
            tmpz = (tmpz * x) % n;
        }
        return tmpz;
    }

    // expo modu (x puiss y) mod n
    pub fn sam_i64(x: u64, y: u64, n: u64) -> u64 {
        let mut binar: Vec<i8> = integer_64_to_bit(y);
        //println!("binaire de la puissance:{:?}", binar);

        let mut tmp_binary_value = binar.pop();
        let mut tmp: u64 = 1;

        loop {
            match tmp_binary_value {
                Some(p) => tmp = operation(x, p, n, tmp),
                None => break,
            }
            tmp_binary_value = binar.pop();
        }

        return tmp;
    }
}
