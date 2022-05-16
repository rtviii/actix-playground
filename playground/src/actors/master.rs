
use actix::{Actor, Context, System, Recipient};
use actix::prelude::*;
use rand::Rng;

pub struct Master{
    emitted    : usize,
    last_signal: usize
}







#[derive(Debug)]
pub struct Signal{
    pub id  : usize,
    pub data: usize
}



impl Signal{
    pub fn new(id:&usize)->Signal{

        let mut rng = rand::thread_rng();

        // let mut r = 
        Signal { id:*id, data: rng.gen_range(0..100) }
    }
}
