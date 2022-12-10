

#[cfg(test)]

pub fn get_input_of_the_day(path: &str) -> String {

    use std::{fs::File, io::Read};
    use core::{panic};

    // let d = PathBuf::from(format!("{}\\src\\{}", env!("CARGO_MANIFEST_DIR"), path));
    let d = format!("{}/src/{}", env!("CARGO_MANIFEST_DIR"), path);
    let mut file = match File::open(d.clone()) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't open {}: {}", d, why),
    };
    let mut input_string: String = String::new();
    match file.read_to_string(&mut input_string) {
        Ok(_) => println!("Cool"),
        Err(_) => panic!("{} contains \n{}", d, input_string),
    }

    input_string
} 
