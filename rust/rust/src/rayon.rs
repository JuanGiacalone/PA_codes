use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;


fn main()
{

    (0..100).into_par_iter().for_each(|x| println!("{:?}", x));
}

