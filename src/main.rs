use std::io;
fn main() {
    loop {
    let row: [char; 8] = ['a','b','c','d','e','f','g','h'];
    let col: [i32; 8] = [1,2,3,4,5,6,7, 8];
    let mut positions: Vec<String> = Vec::new();
    println!("Your Move:");
    let mut input: String = String::new();
    let input: &str = input.trim();
    let moves: Vec<String> = input.split_whitespace().collect();
    // this input is for the color
    
    io::stdin().read_line(&mut input).unwrap();
    //bl or wh for color; h3, h4 positions; wh h3 h4; color current_position destination


    for letter in row.iter() {
        for number in col.iter() {
            let position = format!("{}{}", letter, number );
            positions.push(position);
            print!("{}{} ", letter, number);
        }
        println!()
    }
    }
    break;
}
