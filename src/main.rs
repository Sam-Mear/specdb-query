use specdb::get_spec_db;

// struct Query;


fn main() {
    let spec_db = get_spec_db("/home/sam/Documents/code/SpecDB/specs".to_string());
    
    println!("Files parsed total: {}", spec_db.files.iter().count());
}