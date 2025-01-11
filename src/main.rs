
fn main() {
    let row = ['a','b','c','d','e','f','g','h'];
    let col = [1,2,3,4,5,6,7, 8];
    let mut positions = Vec::new();
    for letter in row.iter() {
        for number in col.iter() {
            let position = format!("{}{}", letter, number );
            positions.push(position);
            print!("{}{} ", letter, number);
        }
        println!()
    }
  /*   for pos in &positions {
        for _ in 0..8
        {
            print!("{}", pos.to_string())
        }
        println!();
    }
*/
}
