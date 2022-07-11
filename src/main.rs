// Module to read messages from AMQP queue and store in REDIS
extern crate amqp;
extern crate redis;

use amqp::{Basic, Session};
use serde::Deserialize;
use serde::Serialize;
use confy;

#[derive(Serialize, Deserialize)]
struct Config {
    version: u8,
    amqp_url: String,
    amqp_queue: String,
    redis_url: String,
}

impl ::std::default::Default for Config {
    fn default() -> Self { 
        Self { 
            version: 1, 
            amqp_url: "amqp://10.122.13.226:5672".into(), 
            amqp_queue: "evt-q-triage-response-cache".into(),
            redis_url: "redis://10.122.13.226:6379".into()
        }
    }
}

fn main() {
    // Use config file instead of static variables
    //TODO: Cannot use ? operator as we're inside main() and main() doesn't return types (on failure)
    //TODO: https://stackoverflow.com/questions/48015600/cannot-use-operator-for-functions-that-return-result-error
    let cfg : Config = confy::load("amqp-to-redis").expect("Cannot parse config file!");

    // Store back config file to capture any dynamic config changes.
    // TODO: Dont need to do this yet as there are no dynamic config changes implemented.
    //confy::store("amqp_to_redis",&cfg).expect("Could not store config file!");

    // AMQP Connection
    let mut session = Session::open_url(cfg.amqp_url.as_str()).unwrap();
    let mut channel = session.open_channel(1).unwrap();
    println!("Connected to AMQP");
    // Redis Connection
    let redis_client = redis::Client::open(cfg.redis_url).unwrap();
    let mut redis = redis_client.get_connection().unwrap();

    println!("Connected to Redis");

    
    //loop {
        for msg in channel.basic_get(cfg.amqp_queue.as_str(), false) {
            let msg_body : String = String::from_utf8_lossy(&msg.body).to_string();
            let msg_id : String = msg.headers.message_id.unwrap(); 
            println!("Message received: {}", msg_body);
            redis::cmd("SET").arg(&msg_id).arg(&msg_body).execute(&mut redis);
        }
    //}

}