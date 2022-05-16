use actix::{prelude::*, clock::interval};
use std::{fmt, time::Duration};
use attempts::{self, add_one};
pub mod actors;
use actors::{*};
use sb_backend_3_actix::actors::{basic::{AtrTimer, AtrAnyToVoid, AtrPrinter}, sb_actor::SBActor, sb_atr_wrapper::{WrappedActor, ActorWrap}};
use sb_backend_3_actix::messages::{MsgVoid};

#[actix_rt::main] 
pub async fn main() {

    let mut timer = AtrTimer::new(2000);

    let printer1  = AtrPrinter::new(Some("[ First ]"));
    let printer2  = AtrPrinter::new(Some("[ Second ]"));

    timer.add_target(printer1.wrap());
    timer.add_target(printer2.wrap());


    let timer_addr = timer.start();

    let _ = timer_addr.do_send(MsgVoid{});
    loop{
        actix::clock::sleep(Duration::from_millis(2000)).await;

        // let mut ival = actix::clock::interval(Duration::from_millis(2000));
        // for i in 0..5{
        //     ival.tick().await;
        //     timer_addr.do_send(MsgVoid{});
        // }
    }
}



