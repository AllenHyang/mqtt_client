use mqtt311::QoS;
use mqtt_client::mqtt::connect;
use std::thread;
use std::time::Duration;

fn main() {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let a = String::new();







    

    let (mut mqtt_client, _notifications) = connect("39.105.42.131", "p1").unwrap();

    let sleep_time = Duration::from_secs(1);
    thread::spawn(move || {
        for _i in 0..10 {
            let payload = data;
            thread::sleep(sleep_time);
            mqtt_client
                .publish("hello", QoS::AtLeastOnce, false, payload)
                .unwrap();
        }
    });

    loop {}
}
