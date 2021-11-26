use phf::phf_map;
use rumqttc::{Client, Event, MqttOptions, Packet, QoS};
use serde::{Deserialize, Serialize};
use std::{env, str};

static SNAME: phf::Map<&'static str, &'static str> = phf_map! {
    "csq[m1]" => "Уровень сигнала",
    "TEMP[taldom]" => "Температура",
    "HUMI[taldom]" => "Влажность",
    "btemp[taldom]" => "Подпол",
};

fn get_sensor_name(key: &str) -> &str {
    match SNAME.get(key) {
        Some(name) => name,
        _ => "???",
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Sensor {
    timestamp: String,
    sensor_name: String,
    sensor_value: String,
}

fn main() {
    let host = env::var("NSG_MQTT_HOST").unwrap();
    let password = env::var("NSG_MQTT_PASSWD").unwrap();

    let mut mqttoptions = MqttOptions::new("rumqtt-sync", host, 1883);
    mqttoptions.set_credentials("imosunov", &password);

    let (mut client, mut connection) = Client::new(mqttoptions, 10);

    client.subscribe("#", QoS::AtMostOnce).unwrap();

    // Iterate to poll the eventloop for connection progress
    for (_i, notification) in connection.iter().enumerate() {
        match notification {
            Ok(Event::Incoming(Packet::Publish(p))) => {
                match serde_json::from_str::<Sensor>(str::from_utf8(&p.payload).unwrap()) {
                    Ok(s) => {
                        println!(
                            "{} {} {}",
                            s.timestamp,
                            get_sensor_name(&s.sensor_name),
                            s.sensor_value
                        )
                    }
                    Err(_) => (),
                }
            }
            Ok(_) => (),
            Err(_) => (),
        }
    }
}
