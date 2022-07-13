use std::fs;
use std::fs::File;
use std::io::Read;


fn main() {
    let filename: String = "../roms/Tetris.gb".to_string();
    let contents = get_file_as_byte_vec(&filename);
        
    println!("{:x?}", contents);
}


fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}