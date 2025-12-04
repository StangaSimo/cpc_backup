use std::fs;
use std::path::PathBuf;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Range { /* range [i,j] structure */
    i: usize,
    j: usize
}

struct LazyNode {
    key: u32,  /* always positive integers as the prof says */ 
    lazy: Option<u32>,  /* lazy value for the lazy controls */
    range: Range,   
    left: Option<usize>, 
    right: Option<usize>
}

impl LazyNode { /* return new Node */
    fn new(key: u32, range: Range) -> Self {
        Self {
            key,
            range,
            lazy: None,
            left: None,
            right: None,
        }
    }
}

pub struct STlazy {
    nodes: Vec<LazyNode>,
}

impl STlazy {
    /* first constructor */ 
    pub fn new() -> Self {
        Self{
            nodes: vec![],
        }
    }

    pub fn build(&mut self, a: Vec<u32>) {
        let n: usize = a.len(); /* n len */
        
        /* build the ST dividing the array a in segment, starting from the root of the ST, 
         * Notare: the root id will be the last element */
        self.rec_build(&a, 0, n-1);
    }

    fn rec_build(&mut self, a: &Vec<u32>, i: usize, j: usize) -> usize {
        if i == j { /* leaf case */
            let id = self.nodes.len(); /* i use this metod to keep the right id of the nodes */
            self.nodes.push(LazyNode::new(a[i], Range {i, j})); 
            return id 
        } 
         
        let id_left = self.rec_build(a, i, (i+j)/2);
        let id_right = self.rec_build(a, (i+j)/2 + 1, j);
        
        let max = self.nodes[id_left].key.max(self.nodes[id_right].key); 

        /* insert the node in the tree */
        let id = self.nodes.len();
        self.nodes.push(LazyNode { key: max, 
                                   range: Range {i, j}, 
                                   lazy: None, 
                                   left: Some(id_left), 
                                   right: Some(id_right)}); 
        id
    }

    pub fn update(&mut self, i: usize, j: usize, t: u32) {
        let n = self.nodes.len()-1; /* start from the root */
        self.rec_update(n, i-1, j-1, t); 
    }

    /* NOTARE: nel total update si aggiorna solamente il valore lazy e si ritorna il minimo, questo
     * per stare consistenti con i nodi sopra che in caso di partial overlap devono aggiornare il
     * proprio massimo.
     * Il valore lazy verrà comunque propagato nel caso di non overlap (riga 94) sempre per mantere aggiornati i nodi sopra,
     * anche nel caso di partial overlap (riga 148) visto che ci serve lo stesso anche per fare l'update corretto, 
     * oppure si propaga dentro max sempre per gli stessi motivi (ma solo quando c'è overlap o partial overlap, visto che no overlap 
     * non ci interessa).
     * Le leaf vengono sempre aggiornate direttamente dalla fn propagate o direttamente da update. */

    fn rec_update(&mut self, cur_id: usize, i: usize, j: usize, t: u32) -> u32 {
        let range : &Range = &self.nodes[cur_id].range;
        let id_left : Option<usize> = self.nodes[cur_id].left;
        let id_right : Option<usize> = self.nodes[cur_id].right;
        let lazy: Option<u32> = self.nodes[cur_id].lazy;

        /* no overlap */
        if range.j < i || range.i > j {
            /* if lazy and not a leaf update and propagate */
            if id_left != None && id_right != None { 
                if lazy != None {
                    self.nodes[cur_id].key = lazy.unwrap().min(self.nodes[cur_id].key); 
                    self.propagate(id_left.unwrap(), id_right.unwrap(), lazy.unwrap());    
                    self.nodes[cur_id].lazy = None;
                }
            }
            return self.nodes[cur_id].key
        }

        /* leaf case, only total overlap possible so update */
        if id_left == None && id_right == None { 
            self.nodes[cur_id].key  = t.min(self.nodes[cur_id].key);
            return self.nodes[cur_id].key
        }
        
        /* total overlap, just lazy but return the minimum */
        if i <= range.i && j >= range.j {
            let mut lazy_value = t;

            /* if lazy already present we keep the min lazy */
            if lazy != None {
                lazy_value = lazy_value.min(lazy.unwrap());
            }

            self.nodes[cur_id].lazy = Some(lazy_value);
            return self.nodes[cur_id].key.min(lazy_value)
        }
        
        /* partial overlap (if we aren't in no overlap or total overlap) */
        /* always check for previusly lazy values */ 
        if lazy != None {
            /* if there is a lazy it means that there was a total overlap, we propagate 
            *  then we call the rec_update and update the max */
            self.nodes[cur_id].key = self.nodes[cur_id].key.min(lazy.unwrap());
            self.propagate(id_left.unwrap(), id_right.unwrap(), lazy.unwrap()); 
            self.nodes[cur_id].lazy = None;
        }

        /* take the max beetween childrens and update the current node key */ 
        let max_left: u32 = self.rec_update(id_left.unwrap(), i, j, t);
        let max_right: u32 = self.rec_update(id_right.unwrap(), i, j, t);
        self.nodes[cur_id].key = max_left.max(max_right);
        return self.nodes[cur_id].key
    }
    
    /* answer the max query */
    pub fn max(&mut self, i: usize, j: usize) -> u32 {
        let n = self.nodes.len()-1; /* start from the root */
        let mut result = 0;
        self.rec_max(n, i-1, j-1, &mut result);
        result
    }

    fn rec_max(&mut self, cur_id: usize, i: usize, j: usize, result: &mut u32) {
        let range : &Range = &self.nodes[cur_id].range;
        let left : Option<usize> = self.nodes[cur_id].left;
        let right : Option<usize> = self.nodes[cur_id].right;
        let lazy: Option<u32> = self.nodes[cur_id].lazy;

        /* no overlap */
        if range.j < i || range.i > j {
            return
        }

        /* leaf case */
        if left == None && right == None { 
            /* leaf in total overlap so update the rusult  */
            if i <= range.i && j >= range.j {
                *result = self.nodes[cur_id].key.max(*result);
            }
            return 
        }

        let id_left = left.unwrap();
        let id_right = right.unwrap();

        /* total overlap, we check the lazy status and propagate */ 
        if i <= range.i && j >= range.j {

            /* if no lazy then we just return the answer */
            /* if lazy situation, propagate */
            if lazy != None {
                /* lazy situation, propagate */
                let lazy_value = lazy.unwrap();

                let min = self.nodes[cur_id].key.min(lazy_value);
                self.nodes[cur_id].key  = min;
                self.nodes[cur_id].lazy = None;

                /* propagate */ 
                self.propagate(id_left, id_right, lazy_value);
            }
            
            /* result of the query */
            *result = self.nodes[cur_id].key.max(*result); 

            return
        }

        /* partial overlap, if needed always propagate */ 
        if lazy != None {             
            let lazy_value = lazy.unwrap();
            self.nodes[cur_id].key = self.nodes[cur_id].key.min(lazy_value);
            self.nodes[cur_id].lazy = None;

            /* propagate */
            self.propagate(id_left, id_right, lazy_value);
        }

        /* recursive calls */ 
        self.rec_max(id_left, i, j, result);
        self.rec_max(id_right, i, j, result);
    }

    /* propagate the lazy value, update if leaf */
    pub fn propagate(&mut self, id_left: usize, id_right: usize, t: u32) {
        let left_lazy = self.nodes[id_left].lazy;           
        let right_lazy = self.nodes[id_right].lazy;           

        if left_lazy == None {
            self.nodes[id_left].lazy = Some(t);
        } else {
            /* lazy already present */
            self.nodes[id_left].lazy = Some(t.min(left_lazy.unwrap()));
        }

        if right_lazy == None {
            self.nodes[id_right].lazy = Some(t);
        } else {
            /* lazy already present */
            self.nodes[id_right].lazy = Some(t.min(right_lazy.unwrap()));
        }
        
        /* leaf case */
        if self.nodes[id_left].left == None && self.nodes[id_left].right == None {
            self.nodes[id_left].key = self.nodes[id_left].key.min(self.nodes[id_left].lazy.unwrap());
        }

        if self.nodes[id_right].left == None && self.nodes[id_right].right == None {
            self.nodes[id_right].key = self.nodes[id_right].key.min(self.nodes[id_right].lazy.unwrap());
        }
    }
}

struct IsThereNode {
    key: HashSet<u32>,  /* always positive integers */ 
    range: Range,   
    left: Option<usize>, 
    right: Option<usize>
}

impl IsThereNode {
    fn new(range: Range) -> Self {
        Self {
            key: HashSet::new(),
            range,
            left: None,
            right: None,
        }
    }
}

pub struct STIsThere {
    nodes: Vec<IsThereNode>,
}

impl STIsThere {
    pub fn new() -> Self {
        Self{
            nodes: vec![],
        }
    }

    pub fn build(&mut self, a: &mut Vec<Range>) {
        let n: usize = a.len();

        let overlaps = sweepline(a, n);     

        /* same build process as the lazy tree */
        self.rec_build(&overlaps, 0, n-1);
    }

    fn rec_build(&mut self, overlaps: &Vec<i32>, i: usize, j: usize) -> usize {
        if i == j { /* leaf case */
            let id = self.nodes.len();
            let mut node = IsThereNode::new(Range {i, j});
            node.key.insert(overlaps[i] as u32);
            self.nodes.push(node); 
            return id 
        } 
         
        let id_left = self.rec_build(overlaps, i, (i+j)/2);
        let id_right = self.rec_build(overlaps, (i+j)/2 + 1, j);
        
        let mut node = IsThereNode::new(Range {i, j});

        for c in i..j {
            node.key.insert(overlaps[c] as u32);
        }

        node.left = Some(id_left);
        node.right = Some(id_right);

        /* insert the node in the tree */
        let id = self.nodes.len();
        self.nodes.push(node);   

        id
    }

    fn is_there(&mut self, i: usize, j: usize, k: u32) -> bool {
       let n = self.nodes.len()-1;
       self.rec_is_there(n ,i, j, k) 
    }

    fn rec_is_there(&mut self,cur_id: usize, i: usize, j: usize, k: u32) -> bool {
        let range = &self.nodes[cur_id].range;
        let left = self.nodes[cur_id].left;
        let right = self.nodes[cur_id].right;

        /* no overlap */
        if range.j < i || range.i > j {
            return false
        }

        /* total overlap */
        if i <= range.i && j >= range.j {   
            if self.nodes[cur_id].key.contains(&k) {
                return true
            }
            return false 
        }

        /* partial overlap */
        let left_res: bool = self.rec_is_there(left.unwrap(), i, j, k);
        let right_res: bool = self.rec_is_there(right.unwrap(), i, j, k);

        left_res || right_res
    }
}

fn sweepline (a: &mut Vec<Range>, n: usize) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::with_capacity(n);
    a.sort_unstable_by_key(|r| r.i);
    let mut diff: Vec<i32> = vec![0; n+1]; /* +2 per i -1 alla fine */

    for i in 0..n {
        diff[a[i].i] += 1;
        diff[a[i].j+1] -= 1;
    }

    let mut overlap = 0; 
    for i in 0..n {
       overlap += diff[i];
       res.push(overlap);
    }

    res
}

pub fn fetch_and_test_min_max(input_folder: &str, output_folder: &str) {
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
        let mut st = STlazy::new(); 
        st.build(a);

        for _ in 0..m {
            let bit_tipo: u32 = input_chars.next().unwrap().parse().unwrap();

            if bit_tipo == 0 { /* update */

                let i: usize = input_chars.next().unwrap().parse().unwrap();
                let j: usize = input_chars.next().unwrap().parse().unwrap();
                let t: u32 = input_chars.next().unwrap().parse().unwrap();
                st.update(i, j, t);                
             
            } else { /* max */

                let i: usize = input_chars.next().unwrap().parse().unwrap();
                let j: usize = input_chars.next().unwrap().parse().unwrap();

                let result: u32 = st.max(i, j);                

                let output_result: u32 = output_chars.next().unwrap().parse().unwrap();                
                if output_result != result {
                    check_err = true;
                    println!("TEST FAILED {:?} != {:?}", result, output_result);
                }
            } 
        }

        if check_err {
            println!("TEST FAILED ");
        } else {
            println!("TEST PASSED ");
        }
    }
}

pub fn fetch_and_test_is_there(input_folder: &str, output_folder: &str) {
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

    /* sort first by length then by string */
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

        let mut segments: Vec<Range> = Vec::new();

        for _ in 0..n {
            let i: usize = input_chars.next().unwrap().parse().unwrap();
            let j: usize = input_chars.next().unwrap().parse().unwrap();
            segments.push(Range {i,j});
        }

        let mut check_err: bool = false;
        let mut st = STIsThere::new(); 
        st.build(&mut segments);

        for _ in 0..m {
            let i: usize = input_chars.next().unwrap().parse().unwrap();
            let j: usize = input_chars.next().unwrap().parse().unwrap();
            let k: u32 = input_chars.next().unwrap().parse().unwrap();

            let result: bool = st.is_there(i, j, k);                

            let output_result: i32 = output_chars.next().unwrap().parse().unwrap();                

            let mut c: bool = false;

            if output_result == 1 {
                c = true;
            }
            
            check_err = false;

            if c != result {
                check_err = true;
                println!("TEST FAILED {:?} != {:?}", result, c);
            }
        }

        if check_err {
            println!("TEST FAILED ");
        } else {
            println!("TEST PASSED ");
        }
    }
}


