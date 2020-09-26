
use std::io;
use io::sys::rand;

fn main()
{

    
    println!("{:?}", seq_range(5,5));

}



fn seq_range (low: i32 ,hi: i32) -> Vec<u32>
{
    let mut v: Vec<u32> = (0..100).collect(); // mut indica que es mutuable y puede ser modificado su valor, no su tipo

    v.shuffle(&mut rand::thread_rng());
    return v;
}



fn for_example_names() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}

fn ex_vec(inpu) -> Vec<i32>{

    let mut output = vec![1_i32, 2, 4];
    output.extend(input.iter().map(|x| x + 1));
    return output;
}

