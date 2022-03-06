// Module to read messages from AMQP queue and store in REDIS
extern crate amqp;
extern crate redis;

use amqp::{Basic, Channel, Session};
//use std::error::Error;
use redis::Commands;

fn main() {
    let amqp_url = "amqp://10.122.13.226:5672";
    let redis_url = "redis://10.122.13.226:6379";    
    let queue_name = "evt-q-triage-response-cache";

    // AMQP Connection
    let mut session = Session::open_url(amqp_url).unwrap();
    let mut channel = session.open_channel(1).unwrap();
    
    // Redis Connection
    let redis_client = redis::Client::open(redis_url).unwrap();
    let mut redis = redis_client.get_connection().unwrap();

    println!("Connected to AMQP and Redis");
    
    loop {
        for msg in channel.basic_get(&queue_name, false) {
            let msg_body : String = String::from_utf8_lossy(&msg.body).to_string();
            let msg_id : String = msg.headers.message_id.unwrap();;   
            println!("Message received: {}", msg_body);
            redis::cmd("SET").arg(&msg_id).arg(&msg_body).execute(&mut redis);
        }
    }
}