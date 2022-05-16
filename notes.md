
- is this the wrap? https://docs.rs/actix/0.5.1/actix/dev/trait.ToEnvelope.html

- is this send to self? https://docs.rs/actix/0.5.1/actix/prelude/trait.AsyncContext.html#method.notify

- why does my shit doesn't work with `do_send` and not `send(...).await`  or `try_send`

- Having problems syncing the timee and the main() interval together. Reading the book:
"In this example MyActor actor is asynchronous and is started in the same thread as the caller - threads are covered in the SyncArbiter chapter"

I want to see if it's possible to implement heterogenous actors defined by their own set strongly typed messages. 

Actix's constraints.


### Lars's snippet from 3rd attempt


```rust
use std::time::Duration;
use actix::{Actor, AsyncContext, Context, Handler};
use tokio::task::JoinHandle;
use crate::actors_3_actor_wrap::wrapper_actor::{ActorWrap, WrappableActor, WrappedActor};
use crate::messages::activate::Activate;
use crate::messages::alternative_message::AlternativeMessage;

#[derive(Default)]
pub struct TimerActor {
    wrapper            : Option<&'static WrappedActor<Activate, Self>>,
    sinks              : Vec   <&'static dyn ActorWrap<Activate>>,
    pub     async_task : Option<JoinHandle<()>>,
}

impl TimerActor {
    pub fn add_sink(&mut self, sink : &'static dyn ActorWrap<Activate>){
        self.sinks.push(sink);
    }
}

impl Actor for TimerActor {
    type Context = Context<Self>;
}

impl WrappableActor<Activate> for TimerActor {
    fn setup_wrapper(&mut self, wrapper_actor: &'static WrappedActor<Activate, Self>) -> () {
        self.wrapper = Some(wrapper_actor);
    }
}

impl Handler<Activate> for TimerActor {
    type Result = ();

    fn handle(&mut self, _msg: Activate, _ctx: &mut Context<Self>) -> Self::Result {
        println!("\ntimer activation");

        // send timer-signal to all sinks
        for sink in &self.sinks {
            sink.do_send(Activate);
        }

        // self-schedule again!
        let addr = _ctx.address();
        self.async_task = Some(actix::spawn(async move {
            actix::clock::sleep(Duration::from_millis(2000)).await;
            addr.do_send(Activate);
        }));
    }
}
```