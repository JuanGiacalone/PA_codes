fn main()
{

    seq_range(5,5);

}




fn seq_range (low: i32 ,hi: i32) -> Vec<i32>
{
    let mut v: Vec<i32> = Vec::new(); // mut indica que es mutuable y puede ser modificado su valor, no su tipo

    for i in Vec!range(low,hi){
        
            Vec.push(&mut v,i); // gracias al compilador agregué &mut v  para que la referencia sea mutuable
    }
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
