std::sync::{Arc,Mutex}

let num = Mutex::new(0);
{
    let mut guard = num.lock().unwrap();
}
println!("{:?}", num);