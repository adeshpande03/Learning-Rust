
pub fn run()
{
    let mut hi = String::from("hello");
    println!("{}", hi);
    println!("{}", hi.len());
    let capacity = hi.capacity();
    for letter in hi.chars() 
    {
        println!("{}", letter)
    }
    hi.push_str("string");
    println!("{}", hi);
    println!("The capacity is {capacity}", capacity = capacity);
    hi.push('c');

    
    
}