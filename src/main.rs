// Module to read messages from AMQP queue and store in REDIS
extern crate amqp;
extern crate redis;

use amqp::{Basic, Channel, Session};
use std::error::Error;
use redis::Commands;
//use redis::Connection;

// Get message from AMQP queue and return body
fn get_message(channel: &Channel,queue_name: String) -> Result<String, Box<dyn Error>> {
    for delivery in channel.basic_get(&queue_name,false) {
        
        let body = String::from_utf8(delivery.body.to_vec())?;
        //let id = String::from_utf8(delivery.delivery_tag.to_vec())?;
        return Ok(body);
    }
    
    Ok("striong".to_string())
}
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

    loop {
        let msg_id = "String".to_string();
        let msg_body = get_message(&channel, queue_name.to_string()).unwrap();
        let cmd = redis::Cmd::new().arg("SET").arg(msg_id).arg(msg_body);
        cmd.query::<()>(&redis).unwrap();
    }
}