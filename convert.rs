fn main() {
    const SIZE: usize = 26;

    for i in 0..SIZE {
        for j in 0..SIZE {
            if i == 0 || i == SIZE - 1 || j == 0 || j == SIZE - 1 {
                print!("*");
            }
            else if i == j || i + j == SIZE - 1 {
                print!("*");
            } 
            else {
                print!(" ");
            }
        }
        println!();
    }
}
