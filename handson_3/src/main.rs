use handson_3::{fetch_and_test_course, fetch_and_test_holiday_planning, test};

fn main() {
    fetch_and_test_holiday_planning("tests/1_test/input", "tests/1_test/output");
    fetch_and_test_course("tests/2_test/input", "tests/2_test/output");
    println!("\n\n"); test(); }
