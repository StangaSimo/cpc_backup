#![allow(unused)] /* TODO: remove this */
use std::fs;
use std::path::PathBuf;

struct It {  /* Itinerary */
    d: u16, /* day */
    a: u16, /* number of attraction */
}

impl It { /* return new Node */
    fn new(day: u16, attraction: u16) -> Self {
        Self {
            d: day,
            a: attraction,
        }
    }
}

struct Node {
    id: usize,
    edges: Vec<usize>,
}

struct Dag {
    vec
}

//pub struct STlazy {
//    nodes: Vec<LazyNode>,
//}
//
//impl STlazy {
//    /* first constructor */ 
//    pub fn new() -> Self {
//        Self{
//            nodes: vec![],
//        }
//    }
//
//    pub fn build(&mut self, a: Vec<u32>) {
//        let n: usize = a.len(); /* n len */
//        
//        /* build the ST dividing the array a in segment, starting from the root of the ST, 
//         * Notare: the root id will be the last element */
//        self.rec_build(&a, 0, n-1);
//    }
//
//    fn rec_build(&mut self, a: &Vec<u32>, i: usize, j: usize) -> usize {
//        if i == j { /* leaf case */
//            let id = self.nodes.len(); /* i use this metod to keep the right id of the nodes */
//            self.nodes.push(LazyNode::new(a[i], Range {i, j})); 
//            return id 
//        } 
//         
//        let id_left = self.rec_build(a, i, (i+j)/2);
//        let id_right = self.rec_build(a, (i+j)/2 + 1, j);
//        
//        let max = self.nodes[id_left].key.max(self.nodes[id_right].key); 
//
//        /* insert the node in the tree */
//        let id = self.nodes.len();
//        self.nodes.push(LazyNode { key: max, 
//                                   range: Range {i, j}, 
//                                   lazy: None, 
//                                   left: Some(id_left), 
//                                   right: Some(id_right)}); 
//        id
//    }
//}
























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

        let n: usize = input_chars.next().unwrap().parse().unwrap();
        let m: usize = input_chars.next().unwrap().parse().unwrap(); 

        let mut a: Vec<u32> = Vec::new();

        for _ in 0..n {
            let val: u32 = input_chars.next().unwrap().parse().unwrap();
            a.push(val);
        }

        let mut check_err: bool = false;
        //let mut st = STlazy::new(); 
        //st.build(a);

        //for _ in 0..m {
        //    let bit_tipo: u32 = input_chars.next().unwrap().parse().unwrap();

        //    if bit_tipo == 0 { /* update */

        //        let i: usize = input_chars.next().unwrap().parse().unwrap();
        //        let j: usize = input_chars.next().unwrap().parse().unwrap();
        //        let t: u32 = input_chars.next().unwrap().parse().unwrap();
        //        st.update(i, j, t);                
        //     
        //    } else { /* max */

        //        let i: usize = input_chars.next().unwrap().parse().unwrap();
        //        let j: usize = input_chars.next().unwrap().parse().unwrap();

        //        let result: u32 = st.max(i, j);                

        //        let output_result: u32 = output_chars.next().unwrap().parse().unwrap();                
        //        if output_result != result {
        //            check_err = true;
        //            println!("TEST FAILED {:?} != {:?}", result, output_result);
        //        }
        //    } 
        //}

        if check_err {
            println!("TEST FAILED ");
        } else {
            println!("TEST PASSED ");
        }
    }
}


