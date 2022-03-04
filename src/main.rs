// Module to read messages from AMQP queue and store in REDIS
extern crate amqp;
extern crate redis;

use amqp::{Basic, Channel, Session};
use std::error::Error;
use redis::Commands;
use redis::Connection;

fn main() {
    let amqp_url = "amqp://10.122.13.226:5672";
    let redis_url = "redis://10.122.13.226:6379";    
    let queue_name = "evt-q-triage-response-cache";

    // AMQP Connection
    let mut session = Session::open_url(amqp_url).unwrap();
    let mut channel = session.open_channel(1).unwrap();
    
    // Redis Connection
    let redis_client = redis::Client::open(redis_url).unwrap();
    let mut redis = redis_client.get_connection();

    loop {
        //let (msg_id, msg_body) = get_message(&ch.1, &queue_name).unwrap();
        //let msg_id = get_message(&ch, queue_name);
        //redis.set(msg_id, "teste".to_string()).unwrap();    
    }
}