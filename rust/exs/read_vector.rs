fn main(){



}


let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}



//source https://doc.rust-lang.org/book/ch08-01-vectors.html?highlight=vector#updating-a-vector+