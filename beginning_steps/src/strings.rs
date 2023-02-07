
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
    println!("{}", hi);
    let mut s: String = String::with_capacity(6);
    s.push('a');
    s.push('b');    
    s.push_str("string");
    let s_cap = s.capacity();
    println!("{}", s_cap);
    println!("{}", s);
}