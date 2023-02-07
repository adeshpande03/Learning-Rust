#![allow(dead_code)]
pub fn run()
{
    // Numbers
    let x:i64 = 29;
    println!("The max size of a 64 bit signed integer is {}", std::i64::MAX);
    println!("{}", x);
    let y:i32 = 65*10;
    println!("{}", y);

    //Booleans
    let active = true;
    let c = 'c';
    println!("{:?}", (x, y, active, c));
    
}
