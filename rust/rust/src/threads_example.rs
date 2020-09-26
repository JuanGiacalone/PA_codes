std::thread
std::sync::Arc

fn main()
{
    let v = Arc::new(vec![1,2,3,4]);

    for _ in 0..10 {
        
        let v2 = v.clone();
        thread::std::thread::spawn(move || {
            println!("v = {:?}",v2);  
        });

    }
    

   
}