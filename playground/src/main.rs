use actix::{clock::interval, prelude::*};
use sb_backend_3_actix::actors::{
    basic::{AtrAnyToVoid, AtrPrinter, AtrTimer},
    sb_actor::SBActor,
    sb_atr_wrapper::{ActorWrap, WrappedActor},
};
use sb_backend_3_actix::messages::MsgVoid;
use std::{fmt, process::Output, time::Duration};


pub fn main() {

    let msg = Message{
        payload:"hi".to_string()
    };

    let x = msg.ordered_index(&mut|y|{ y.and_then(|s|{Some(*s + 2)})});
    println!("{:#?}", msg);

    let y = msg.ordered_index(&mut|y|{ y.and_then(|s|{Some(*s -12 )})});
    println!("Ordered index for this message is {:#?}", y.unwrap());
}



#[derive(Default,Debug)]
pub struct Message {
    pub payload:String
}

#[derive(Default,Debug)]
pub struct BinaryBlock {
    pub offset: u64,
    pub payload:Vec<u8>
}
trait OrderedIndex {
    fn ordered_index(&self, alterer: &mut dyn FnMut(Option<&mut u64>) ->Option<u64>) -> Option<u64>;
}

impl OrderedIndex for Message{
     fn ordered_index(&self, getset_index: &mut dyn FnMut(Option<&mut u64>) ->Option<u64>) -> Option<u64> {
        let mut initial_ix           = 100;
        let mut change_state = move | alterer: &mut dyn FnMut(Option<&mut u64>) ->Option<u64> |  alterer(Some(&mut initial_ix)) ;
        let next_state       = change_state(getset_index);
        next_state
    }
}

impl OrderedIndex for BinaryBlock{
    fn ordered_index(&self, getset_index: &mut dyn FnMut(Option<&mut u64>) ->Option<u64>) -> Option<u64> {
        let mut initial_ix        = self.offset.clone();
        let mut change_state = move | alterer: &mut dyn FnMut(Option<&mut u64>) ->Option<u64> |  alterer(Some(&mut initial_ix));
        let next_state       = change_state(getset_index);
        next_state
    }
}