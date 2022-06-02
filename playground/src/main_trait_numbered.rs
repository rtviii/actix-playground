use std::fmt::Display;
use actix::Message;
extern crate proc_macro;
use proc_macro::TokenStream;




trait OrderedIndex{
    fn ordered_index(&self) -> Option<u64>;
}

pub trait SBMessage : Message<Result=()> + 'static + Send {}


#[derive(Default)]
pub struct MsgRawBinary {
    pub key: Option<Vec<u8>>,
    pub value : Vec<u8>
}

impl SBMessage for MsgRawBinary {}
impl Message for MsgRawBinary { type Result = (); }


#[derive(Default, Debug)]
pub struct MsgString {
    pub key  : Option<Vec<u8>>,
    pub value: String
}

impl Display for MsgString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MsgString {{ key: {:?}, value: {} }}", self.key, self.value)
    }
}


impl OrderedIndex for MsgString{

}


// impl SBMessage for MsgString {}
// impl Message for MsgString { type Result = (); }

pub fn main(){
    println!("Hello, world!"); 

    let mut msg = MsgString::default();
    println!("{}", msg);



}