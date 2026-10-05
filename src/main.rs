use std::{
    io::{
        self,
        Write,
        BufRead,
        BufReader,
    }, 
    path::PathBuf,
    fs::File,
};


fn main() {
    print!(">> ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    
    std::io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read input");
 
    // println!("Your input is {}", input);

    let input_file: PathBuf = match input.trim().parse(){
        Ok(f) => f,
        Err(_) => {
            println!("Please give path to a file to interpret");
            return;
        }
    };

    // println!("You gave file '{}'", file.display());
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    println!("File content is: ");
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}