use std::{time::Duration, vec, thread};

use log::{warn, info};
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{Consumer, ConsumerContext, StreamConsumer, CommitMode, BaseConsumer},
    error::{KafkaError, KafkaResult},
    ClientConfig, ClientContext, Offset, Message, TopicPartitionList,
};

pub struct MyCtx;

impl ClientContext for MyCtx {}
impl ConsumerContext for MyCtx {}


type MyConsumer = StreamConsumer<MyCtx>;

#[tokio::main]
pub async fn main() {
    // struct CustomContext
    // let context = MyCtx;


        let consumer : StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("group.id", "rt-rdkfk")
        .create().expect("Consumer creation failed");

        // assign the specified partition at the specified read-offset to the kafka consumer.
        let mut partition_list = TopicPartitionList::new();

        let topic     = "historic_blocks_json";
        let offset    = 57000000;
        let partition = 0;

        partition_list
        .add_partition_offset(topic, 0, Offset::from_raw(offset as i64))
        .expect(&format!("Failed to create partition offset for assignment to kafka consumer:\
                topic-{:?} $ partition-{:?} $ offset-{:?}", topic, partition, offset));

        consumer.assign(&partition_list)
            .expect(&format!("Failed to assign topic partition to kafka-consumer topic-{:?} $ partition-{:?}", topic, partition));

        let x = consumer.recv().await.unwrap();

        println!("GOT X : {:?}", x);


    // let consumer: BaseConsumer = ClientConfig::new()
    //     .set("bootstrap.servers", "localhost:9092")
    //     .set("group.id", "rt-rdkfk")
    //     .create()
    //     .expect("invalid consumer config");
    // consumer
    // .subscribe(&["historic_blocks_json"])
    // .expect("subscripotion failed");


    // for msgresult in consumer.iter(){
    //     let m = msgresult.unwrap();

    //         let payload = match m.payload_view::<str>() {
    //             None => "",
    //             Some(Ok(s)) => s,
    //             Some(Err(e)) => {
    //                 warn!("Error while deserializing message payload: {:?}", e);
    //                 ""
    //             }
    //         };
    //     info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
    //             m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());

        // .set("enable.partition.eof", "false")
        // .set("session.timeout.ms", "6000")
        // .set("enable.auto.commit", "true")
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
        // .set_log_level(RDKafkaLogLevel::Debug)
        // .create_with_context(MyCtx)
        // .expect("Couldn create consumer.");

    // consumer
    //     .subscribe(&[TOPIC])
    //     .expect("Couldn't subscript to topic.");

    // consumer
    //     .subscribe(&vec!["historic_blocks_json"])
    //     .expect("Can't subscribe to specified topics");

    // consumer.seek("historic_blocks_json", 0 as i32, Offset::End, Duration::from_secs(4)).map_err(
    //     |err| {println!("Error: {:?}", err)});

    // if let Ok(sub) = consumer.subscription() {
    //     println!("Subscription is such: {:?}", sub);
    // } else {
    //     println!("errored out");
    // }
    // let x = consumer.recv().await;
    // println!("RECVd : {:?}", x);
    // match consumer.recv().await {
    //     Err(e) => warn!("Kafka error: {}", e),
    //     Ok(m) => {
    //         let payload = match m.payload_view::<str>() {
    //             None => "",
    //             Some(Ok(s)) => s,
    //             Some(Err(e)) => {
    //                 warn!("Error while deserializing message payload: {:?}", e);
    //                 ""
    //             }
    //         };
    //         info!(
    //             "key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
    //             m.key(),
    //             payload,
    //             m.topic(),
    //             m.partition(),
    //             m.offset(),
    //             m.timestamp()
    //         );
    //         // if let Some(headers) = m.headers() {
    //         //     headers.iter().into_iter().for_each(|header| {
    //         //         info!("  Header {:#?}: {:?}", header.key, header.value);
    //         //     });
    //         // }
    //         consumer.commit_message(&m, CommitMode::Async).unwrap();
    //     }
    // }
}
