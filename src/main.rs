// Module to read messages from AMQP queue and store in REDIS
extern crate amqp;
extern crate redis;
use amqp::{Basic, Channel, Connection, Consumer, ConsumerMessage, Delivery};
use std::env;
use std::Error;

fn connect_to_amqp(url: &str) -> Result<(Connection, Channel), Error> {
    let conn = Connection::open(url)?;
    let ch = conn.open_channel(None)?;
    Ok((conn, ch))
}

fn connect_to_redis(url: &str) -> Result<RedisClient, redis::RedisError> {
    let client = RedisClient::open(url)?;
    Ok(client)
}

fn get_message(ch: &Channel, queue_name: &str) -> Result<(String, String), Error> {
    let queue = ch.queue_declare(queue_name, false, false, false, false, None)?;
    let msg = ch.basic_get(&queue, false)?;
    let body = msg.body.unwrap();
    let body = String::from_utf8(body.to_vec()).unwrap();
    let body = body.trim();
    let body = body.split(" ").collect::<Vec<&str>>();
    let msg_id = body[0];
    let msg_body = body[1];
    Ok((msg_id.to_string(), msg_body.to_string()))
}

fn main() {
    let amqp_url = "amqp://10.122.13.226:5672";
    let redis_url = "redis://10.122.13.226:6379";    
    let queue_name = "evt-q-triage-response-cache";

    let mut (conn,amqp) = connect_to_amqp(amqp_url).unwrap();
    let mut redis = connect_to_redis(redis_url).unwrap();

    loop {
        let (msg_id, msg_body) = get_message(&amqp.1, &queue_name).unwrap();
        redis.set(&msg_id, &msg_body).unwrap();    
    }
}