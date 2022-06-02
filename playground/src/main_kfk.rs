use rdkafka::{consumer, ClientConfig};
use sb_backend_3_actix::{
    actors::{
        basic::{AtrPrinter, AtrUtf8ToString, AtrStringToSerde,AtrIOWrite},
        kafka::{AtrKafkaConsumer, MsgJumpToOffset},
        sb_actor::SBActor,
        sb_atr_wrapper::ActorWrap,
    },
    messages::MsgVoid,
};
use std::time::Duration;
const TOPIC: &'static str = "modern_blocks_json";

#[actix_rt::main]
pub async fn main() {

    let actor_iowrite      = AtrIOWrite::new().wrap();
    let actor_string2serde = AtrStringToSerde::new_numbered(actor_iowrite).wrap();
    let actor_raw2string   = AtrUtf8ToString::new_numbered(actor_string2serde).wrap();

    let consumer   = AtrKafkaConsumer::new_simple(TOPIC, actor_raw2string);
    let consumer_w = consumer.wrap();

    consumer_w.do_send(MsgJumpToOffset { offset: 3_000_000 });

    loop {
        let mut ival = actix::clock::interval(Duration::from_millis(3000));
        ival.tick().await;
        consumer_w.do_send( MsgVoid{} );
    }
}
