//VSSS, by leo fisher

use std::string::String;
use base64::{engine, alphabet::STANDARD, Engine as _};
use base64::engine::general_purpose::STANDARD_NO_PAD;


struct Slice {
    height: u8,
    values: u64,
}

struct Rule {
    table: [bool; 512],
}

impl Rule {
    
    fn gen_table(mut rule: String) -> Self {
        rule.drain(0..3);
        println!("{rule}");
        //let config = engine::GeneralPurposeConfig::new();
        //let engine = engine::GeneralPurpose::new(&STANDARD, config);
        let decoded = STANDARD_NO_PAD.decode(rule).unwrap(); 
        let mut table: [bool; 512] = [false; 512];
        let mut x = 0;
        for item in decoded.iter() {
            //println!("{item}");
            for chr in format!("{item:b}").chars() {
                println!("{}", chr);
                match chr {
                    '0' => table[x] = false,
                    '1' => table[x] = true,
                    _ => panic!("something went wrong in ruletable generation"),
                }
                x+=1;
            }
            x+=1;
        }
        return Rule {
            table: table,
        }
    }
    
    fn result(&self, inputs: [bool; 9]) -> bool {
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

/*
fn next_slices(slices: [Slice; 2]) {
    
}
*/
fn main() {
    let conwaylife = Rule::gen_table("MAPARYXfhZofugWaH7oaIDogBZofuhogOiAaIDogIAAgAAWaH7oaIDogGiA6ICAAIAAaIDogIAAgACAAIAAAAAAAA".to_string());
    println!("{}", conwaylife.result([true, true, true, false, false, false, false, false, false]).to_string());
    println!("Hello, world!");
}
