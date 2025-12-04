use handson_2::{fetch_and_test_is_there, fetch_and_test_min_max};

fn main() {
   println!("Min Max ----------------------");
   fetch_and_test_min_max("tests/1_test/input/", "tests/1_test/output/");
   println!("is There ---------------------");
   fetch_and_test_is_there("tests/2_test/input/", "tests/2_test/output/");
}

