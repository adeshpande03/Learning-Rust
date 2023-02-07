#![allow(dead_code, unused)]
use std::mem;
pub fn run() {
    let mut numbers: Vec<i32 > = vec![1, 2, 3, 4];
    numbers[2] = 20;
    numbers.push(534254);
    numbers.pop();
    println!("{:?}", numbers);
    println!("Single Value: {}", numbers[0]);
    println!("Array Length: {}", numbers.len());
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    for x in numbers.iter()
    {
      println!("number: {}", x);
    }
    
    for x in numbers.iter_mut()
    {
      *x *= 2
    }
    println!("{:?}", numbers);
}
