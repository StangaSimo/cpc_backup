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

struct Node {
    key: u32,
    right: Option<usize>,
    left: Option<usize>,
}

impl Node {
    pub fn new (key: u32, right: Option<usize>, left: Option<usize>) -> Node {
        Self{ 
            key,
            right,
            left,
        }
    }
}

struct Bst {
    v: Vec<Node>,
}

impl Bst {
    pub fn new () -> Bst {
        Self{ 
            v: Vec::new(),
        }
    }

    /* we build it with the random values and not in order, 
     * starting from the root we search for the right place for the keys*/
    pub fn build(&mut self, difficulty: Vec<u32>) {
        for i in 0..difficulty.len() {
            println!("i {:?}",i);
            self.add(difficulty[i]);
        }
    }

    pub fn add(&mut self, key: u32) {
        self.rec_add(key, 0);
    }

    pub fn rec_add(&mut self, key: u32, index: usize) {
        let n = self.v.len();

        /* root */
        if n == 0 {
            self.v.push(Node::new(key,None,None));
            return 
        }

        let node_key = self.v[index].key;

        /* right */
        if key >= node_key {
            if self.v[index].right == None {
                self.v.push(Node::new(key,None,None));
                self.v[index].right = Some(n);
            } else {
                self.rec_add(key, self.v[index].right.unwrap());
            }
            /* left */
        } else {
            if self.v[index].left == None {
                self.v.push(Node::new(key,None,None));
                self.v[index].left = Some(n);
            } else {
                self.rec_add(key, self.v[index].left.unwrap());
            }
        }
    }

    /* return Some(successor) */
    /* if right exists, then go in, and search for the min, so all left */
    pub fn successor(&mut self, key: u32) -> Option<usize> {
        if self.v.len() == 0 {
            return None 
        }

        /* if root_key == target and right exists return right or min left right*/
        if self.v[0].key == key && self.v[0].right != None {
            let right_id = self.v[0].right.unwrap();
            return Some(self.min_left(right_id));
        }

        let mut succ: Option<usize> = None; 
        let mut curr : Option<usize> = Some(0); 

        /* si cerca l'index della key piu grande ma piu vicina al target */
        while curr != None {
            let curr_id = curr.unwrap();
            let curr_key = self.v[curr_id].key;
            if key < curr_key {
                succ = curr;
                curr = self.v[curr_id].left;
            } else {
                curr = self.v[curr_id].right;
            }
        }

        succ 
    }

    fn min_left(&mut self, i: usize) -> usize {
        if (self.v[i].left != None) {
            return self.min_left(self.v[i].left.unwrap());
        }
        return i
    }

    fn max_right(&mut self, i: usize) -> usize {
        if (self.v[i].right != None) {
            return self.max_right(self.v[i].right.unwrap());
        }
        return i
    }

    pub fn print(&mut self) {
        println!("-------------------------------------");
        self.print_recursive(Some(0), 0);
        println!("-------------------------------------");
    }

    fn print_recursive(&mut self, i: Option<usize>, level: usize) {
        if i == None {
            return;
        }

        let index = i.unwrap(); 
        let right_id = self.v[index].right;
        let left_id = self.v[index].left;;

        const INDENT: usize = 4; // Spazi per livello

        self.print_recursive(right_id, level + 1);

        let spaces = " ".repeat(level * INDENT);
        let key = self.v[index].key;

        if level > 0 {
            println!("{:?}┌── {:?}:{:?}", spaces, index, key);
        } else {
            println!("{:?}└── {:?}:{:?}", spaces, index, key); // root
        }

        self.print_recursive(left_id, level + 1);
    }
}

struct Course {
    b: Vec<u32>, /* beauty */
    d: Vec<u32>, /* difficulty */
    bst: Bst,
}

impl Course {
    pub fn new (b: Vec<u32>, d: Vec<u32>) -> Course {
        Self { 
            b,
            d,
            bst: Bst::new(),
        }
    }

    pub fn bst_len(&mut self) -> usize {
        self.bst.v.len()
    }

    pub fn lis(&mut self) -> usize {
        let mut pairs: Vec<(&u32, &u32)> = self.d.iter().zip(self.b.iter()).collect();

        /* sort by difficulty, if equal by beauty */
        pairs.sort_by(|a, b| {
            if a.0 != b.0 {
                a.0.cmp(b.0)
            } else {
                b.1.cmp(a.1)
            }
        });

        /* collect beaty */
        let b_by_d: Vec<u32> = pairs.into_iter().map(|(_d, b)| *b).collect();

        if b_by_d.is_empty() { return 0; }

        self.bst.add(b_by_d[0]);

        for i in 1..b_by_d.len() {
            let key = b_by_d[i];
            let max_id = self.bst.max_right(0);
            let max_key = self.bst.v[max_id].key;

            if key > max_key {
                /* if greater than max add key */
                self.bst.add(key);
            } else {
                /* if succ exists, we swap succ with key */ 
                let succ = self.bst.successor(key);
                if succ != None {
                    self.bst.v[succ.unwrap()].key = key;
                }
            } 
        }
        /* the length of the bst */
        self.bst_len()
    }
}

pub fn fetch_and_test_course(input_folder: &str, output_folder: &str) {
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

        let output_result: usize = output_chars.next().unwrap().parse().unwrap();                

        let n: usize = input_chars.next().unwrap().parse().unwrap();

        let mut beauty: Vec<u32> = Vec::with_capacity(n);
        let mut difficulty: Vec<u32> = Vec::with_capacity(n);

        /* b d*/
        for _ in 0..n {
            let b: u32 = input_chars.next().unwrap().parse().unwrap();
            let d: u32 = input_chars.next().unwrap().parse().unwrap();
            beauty.push(b);
            difficulty.push(d);
        }

        let mut course: Course = Course::new(beauty, difficulty);

        let result: usize = course.lis();

        let mut check_err: bool = false;

        if output_result != result {
            check_err = true;
            println!("TEST FAILED {:?} should be {:?}", result, output_result);
        }

        if check_err {
            println!("TEST FAILED ");
        } else {
            println!("TEST PASSED ");
        }

    }
}

