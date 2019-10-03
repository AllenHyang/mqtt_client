pub use rumqtt::{MqttClient, MqttOptions, QoS, Notification, ConnectError};
use std::{thread, time::Duration, str};
use mqtt311::Publish;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use serde_json::json;



pub trait Unwrap<T> {
     fn unwrap<>(self) -> T;
}

 impl Unwrap<Publish> for Notification {
  fn unwrap(self) -> Publish {
        match self {
            Notification::Publish(val) => val,
            _ =>
                panic!("called `Notification::unwrap()` on a `None` value"),
        }
    }
}


pub fn connect(host: &str,name: &str) -> Result<(MqttClient, crossbeam_channel::Receiver<Notification>), ConnectError> {
    let mut mqtt_options = MqttOptions::new(name, host, 1883).set_keep_alive(30).set_clean_session(false);

    MqttClient::start(mqtt_options)
}

#[test]
fn test_pub() {
    let john = json!({
    "name": "john",
    "age":  21,
    "phones": [
        format!("+44 {}", 1232421)
    ]
});
    let (mut client, recv) = connect("39.105.42.131","testpub").unwrap();
    client.subscribe("hello", QoS::AtLeastOnce).unwrap();
    client.publish("hello", QoS::AtLeastOnce, false, john.to_string()).unwrap();

    for r in recv {
        println!("{:?}", r);
    }
}


fn main() {
    /*
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
    // let v:Value= serde_json::from_str(data).unwrap();
    //println!("Please call {} at the number {}", v["name"], v["phones"][0]);


    let (mut mqtt_client, notifications) = connect("39.105.42.131").unwrap();
    // let (done_tx, done_rx) = crossbeam_channel::bounded(1);
    // mqtt_client.subscribe("hello/world", QoS::AtLeastOnce).unwrap();
    //mqtt_client.subscribe("test", QoS::AtLeastOnce).unwrap();
    let sleep_time = Duration::from_secs(1);
    thread::spawn(move || {
        for i in 0..10 {
            let payload = data;
            thread::sleep(sleep_time);
            mqtt_client.publish("hello", QoS::AtLeastOnce, false, payload).unwrap();
        }

        //thread::sleep(sleep_time * 10);
    });


    /*for notification in notifications {
        let p =notification.unwrap();
        println!("{:?},{:?}", p.topic_name,p.payload);
        let s= str::from_utf8(&**p.payload).unwrap();
        println!("{}",s);
        let j :Value = serde_json::from_slice(&**p.payload).unwrap();
        println!("{:?}",j);


    }*/

    loop {}
    println!("Hello, world!");*/
}
