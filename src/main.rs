//VSSS, by leo fisher

use std::string::String;
use base64::{engine, alphabet::STANDARD, Engine as _};
use base64::engine::general_purpose::STANDARD_NO_PAD;



fn nth_bit_u64(input: u64, bit: i8) -> bool { //you would really think that rust has a function for this, but apparently not.
    if bit < 0 {
        return false
    }
    match format!("{input:064b}").remove(bit.try_into().unwrap()).to_digit(2).unwrap() { //may god have mercy on your wretched soul
        0 => return false,
        1 => return true,
        _ => panic!("something went wrong in nth_bit_u64"),
    }
}

fn set_nth_bit_u64(input: u64, bit: i8, target: bool) -> u64 {
    let new = u64::MAX - 1 << (bit as u64);
    let mut target_u64 = u64::MIN;
    if target {
        target_u64 = u64::MAX;
    }
    return (target_u64&new)|(input&!new);
}

struct Rule {
    table: [bool; 512], // not technically the most efficent way to store this, but this is easier to access, so too bad!
}

impl Rule {
    
    fn gen_table(mut rule: String) -> Self {
        rule.drain(0..3);                                    //trim the MAP string to just the important stuff
        let decoded = STANDARD_NO_PAD.decode(rule).unwrap(); //use standard base64 alphabet to decode the rulestring into a vec[u8]
        
        let mut intable: [bool; 512] = [false; 512];         //initialize table
        let mut x = 0;
        
        for item in decoded.iter() {
            //println!("{item}");
            
            for chr in format!("{item:08b}").chars() {       //format each u8 into a binary string
                //println!("{}", chr);
                match chr {
                    '0' => intable[x] = false,
                    '1' => intable[x] = true,
                    _ => panic!("something went wrong in ruletable generation"),
                }
                x+=1;
            }
            
        }
        
        
        return Rule {
            table: intable,
        }
    }
    
    fn evolve(&self, inputs: [bool; 9]) -> bool {  //function to evolve 9 cells and get the result of the cell in the middle
        let mut total = 0;
        let mut x = 0;
        for bit in inputs {
            if bit {
                total = total + usize::pow(2, x);
            }
            x+=1;
        }
        return self.table[total];
    }
}  

struct Slice {
    height: u8,
    values: u64,
}


/*
impl Slice {
    fn next_bit(&self, prev_slice: Slice, rule: &Rule, offset: i8, period: i8, bit: i8, prevbits: u64, results: Option<Vec<Slice>>) -> Option<Vec<Slice>> { 
        //this function is part of the depth-first slice finder. it is used to generate the next set of slices given any slice.
        for x in [true,false] {
            let result = rule.evolve([ // this implements equation (*) from https://arxiv.org/pdf/cs/0004003.pdf
                nth_bit_u64(prev_slice.values, bit),nth_bit_u64(self.values, bit),x,
                nth_bit_u64(prev_slice.values, bit-period),  nth_bit_u64(self.values, bit-period),   nth_bit_u64(prevbits, bit-period),
                nth_bit_u64(prev_slice.values, bit-2*period),nth_bit_u64(self.values, bit-2*period), nth_bit_u64(prevbits, bit-2*period)
            ]);
            if result = nth_bit_u64(self.values, bit-2*period+offset) { //this verifies that the new slice so far works.
                self.next_bit(prev_slice, rule, offset, period, bit+1, prevbits, results);
                
            
        }
        //rule.evolve([]);
        None
    }
}
*/
/*
fn next_slices(slices: [&Slice; 2], rule: &Rule, offset: u8, period: u8) -> Option<Vec<Slice>> {
    
}
*/

use std::time::{Instant, Duration};

fn main() {
    let conwaylife = Rule::gen_table("MAPARYXfhZofugWaH7oaIDogBZofuhogOiAaIDogIAAgAAWaH7oaIDogGiA6ICAAIAAaIDogIAAgACAAIAAAAAAAA".to_string());
    println!("{}", conwaylife.evolve([true, false, false, false, true, false, false, false, false]).to_string());
    println!("Hello, world!");
    //assert_eq!(nth_bit_u64(1, 64), true);
    /*
    let mut x = 0;
    let mut total = 0;
    while 0 == 0 {
        x = x+1;
        for y in 0..63 {
            let start = Instant::now();
            nth_bit_u64(x, y);
            let end = start.elapsed().as_nanos() as u64;
            total = total + end;
            println!("time elapsed avg {end}");
        }
    }
    */
    let mut ooo = "ooo".to_string();
    ooo.replace_range(1..2, "p");
    println!("{ooo}");
    
    println!("{}",set_nth_bit_u64(0, 0, true));
            
}
