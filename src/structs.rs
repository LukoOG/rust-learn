fn main(){
    let width: u8 = 8;
    let length: u8 = 9;

    println!("{}", calculate_area(width, length))
}

fn calculate_area(w: u8, l: u8) -> u8{
    w * l
}