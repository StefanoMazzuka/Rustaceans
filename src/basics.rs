fn main() {
    my_primitive_data_types();
}

fn my_primitive_data_types() {
    // integers
    let _i8: i8 = 1; // -128 -> 127 - 1 byte
    let _u8: u8 = 1; // 0 -> 255

    let _i16: i16 = 1; // -32.768 -> 32.767
    let _u16: u16 = 1; // 0 -> 65.535

    let _i32: i32 = 1; // -2.147.483.648 -> 2.147.483.647
    let _u32: u32 = 1; // 0 -> 4.294.967.295

    let _i64: i64 = 1; // -9.223.372.036.854.775.808 -> 9.223.372.036.854.775.807
    let _u64: u64 = 1; // 0 -> 18.446.744.073.709.551.615

    // floating
    let _f32: f32 = 0.32; // Presition 32 bits - 4 bytes
    let _f64: f64 = 0.32; // Presition 64 bits - 8 bytes

    // booleans
    let _true:  bool = true;  // 1 byte
    let _false: bool = false; // 1 byte

    // character
    let _char: char = 'c'; // 4 bytes
}

fn my_prints() {
    println!("just text");
    
    let value: i32 = 1;
    println!("text and value: {}", value);

    let vector: Vec<i32> = vec![1, 2, 3];
    println!("collections: {:?}", vector);
    println!("pretty collections: {:#?}", vector);
}
    // COLLECTIONS
    // String
    // Sequences: Vec, VecDeque, LinkedList
    // Maps: HashMap, BTreeMap
    // Sets: HashSet, BTreeSet
    // Misc: BinaryHeap

fn my_strings() {
    let _str_0: &str    = "string";               // inmutable
    let mut _str_1: str = String::from("string"); // mutable
}    
fn my_vectors() {
    
    let mut my_vec: Vec<i32> = Vec::new();

    my_vec.push(1);
    my_vec.push(2);

    println!("{:#?}", my_vec)
}