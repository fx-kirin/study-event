use std::collections::VecDeque;

#[derive(Debug)]
struct TestStruct {
    name: String,
}

fn main() {
    let mut deq = VecDeque::new();
    deq.push_back(TestStruct{name:"name".to_string()});
    let slice : Vec<&TestStruct> = deq.iter().collect();
    let slice2 : Vec<&TestStruct> = slice.into_iter().collect();
    let slice3 : Vec<&&TestStruct> = slice2.iter().collect();
}
