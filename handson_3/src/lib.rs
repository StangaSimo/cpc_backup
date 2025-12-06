#![allow(unused)] use core::net;
/* TODO: remove this */
use std::fs;
use std::path::PathBuf;
use std::mem;

/* one array max for the "old" best max attraction days and new_max
 * for the current best maxes */
struct Holidays {
    n: usize,
    d: usize, 
    prefixes: Vec<u32>,
    max: Vec<u32>,
    new_max: Vec<u32>
}

impl Holidays {
    pub fn new(n: usize, d: usize, v: Vec<u32>) -> Self {
        Self{
            n,
            d,
            prefixes: Holidays::compute_prefix(n,d,v),
            max: vec![0; d],
            new_max: vec![0; d],
        }
    }

    fn compute_prefix(n: usize, d: usize, v: Vec<u32>) -> Vec<u32> {
        let mut result: Vec<u32> = vec![0;n*d];
        let mut sum = 0;
        for i in 0..n*d {
            if (i % d) == 0 {sum = 0;}
            sum += v[i];
            result[i] = sum;
        }
        result
    }

    fn compute_holiday(&mut self) -> u32 {
        let d = self.d; /* column */
        let n = self.n; /* rows */

        /* first best max attraction days */
        for i in 0..d {
            self.max[i] = self.prefixes[i];
        }

        /* for each prefix j we ask 
         * j better than max j ?
         * 0 and j-1 better than max j? 
         * 1 and j-2 better than max j
         * ... 
         * */

        for i in 1..n {
            for j in 0..d {

                let mut max: u32 = 0;
                max = self.prefixes[i*d+j].max(self.max[j]);

                let mut i_l: usize = 0; /* i_left */

                for i_r in (0..j).rev() { /* i_right = j-1 to 0 */ 
                    max = max.max(self.prefixes[i*d+i_r] + self.max[i_l]);   
                    i_l += 1;
                }

                self.new_max[j] = max;  
            }
            mem::swap(&mut self.max,&mut self.new_max); /* should swap them with pointers */
        }

        self.max[d-1] /* return the last element that is the 
                           max attractions possibile in the day */
    }
}

pub fn fetch_and_test_holiday_planning(input_folder: &str, output_folder: &str) {
    let mut input_files : Vec<PathBuf> = fs::read_dir(input_folder)
        .expect("ERROR TEST FILES")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect();

    let mut output_files : Vec<PathBuf> = fs::read_dir(output_folder)
        .expect("ERROR OUTPUT FILES")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect();

    /* sort first by length then by string (se no input10 era meno di input2) */
    input_files.sort_by_key(|p| (p.to_str().unwrap().len(), p.clone()));
    output_files.sort_by_key(|p| (p.to_str().unwrap().len(), p.clone()));

    let mut input_iter = input_files.iter(); let mut output_iter = output_files.iter();

    assert!(input_files.len() == output_files.len());

    for _ in 0..input_files.len() {
        let input_path = input_iter.next().unwrap();
        let output_path = output_iter.next().unwrap();
        let input_string = fs::read_to_string(input_path).unwrap();
        let output_string = fs::read_to_string(output_path).unwrap();

        println!("{:?} and {:?}", input_path, output_path);
        
        let mut input_chars = input_string.split_whitespace();
        let mut output_chars = output_string.split_whitespace();

        let output_result: u32 = output_chars.next().unwrap().parse().unwrap();                
        let n: usize = input_chars.next().unwrap().parse().unwrap();
        let d: usize = input_chars.next().unwrap().parse().unwrap(); 
        let mut v: Vec<u32> = Vec::with_capacity(n*d); 

        for _ in 0..(n*d) {
            let val: u32 = input_chars.next().unwrap().parse().unwrap();
            v.push(val);
        }

        let mut h: Holidays = Holidays::new(n,d,v);

        let result: u32 = h.compute_holiday();

        let mut check_err: bool = false;

        if output_result != result {
            check_err = true;
            println!("TEST FAILED {:?} != {:?}", result, output_result);
        }
 
        if check_err {
            println!("TEST FAILED ");
        } else {
            println!("TEST PASSED ");
        }
    }
}





//pub fn test_h() {
//    let  n: usize = 5;
//    let  d: usize = 4;
//
//    let v: Vec<u32> = vec![4,1,1,2,1,1,0,5,5,0,1,1,2,1,0,4,3,1,0,3];
//
//    println!("------- v");
//    for i in 0..n {
//        for j in 0..d {
//            print!(" {:?} ",v[i*d+j]);
//        }
//        print!("\n");
//    }
//
//    let mut h: Holidays = Holidays::new(n,d,v);
//
//    println!("------- p");
//    for i in 0..n {
//        for j in 0..d {
//            print!(" {:?} ",h.prefixes[i*d+j]);
//        }
//        print!("\n");
//    }
//
//    println!("result : {:?} ==? 14",h.compute_holiday())
//}


