use url::Url;
use tungstenite::{connect, Message};
use serde_json;
use std::sync::{Arc, Mutex};

use crypto::app::App;

const BASE_URL: &str = "wss://stream.data.alpaca.markets/v1beta2/crypto";

fn alpaca(
        security1: Arc<Mutex<f64>>, 
        security2: Arc<Mutex<f64>>, 
        security3: Arc<Mutex<f64>>, 
        security4: Arc<Mutex<f64>>, 
        security5: Arc<Mutex<f64>>, 
        security6: Arc<Mutex<f64>>, 
        data_security1: Arc<Mutex<Vec<[f64; 2]>>>, 
        data_security2: Arc<Mutex<Vec<[f64; 2]>>>, 
        data_security3: Arc<Mutex<Vec<[f64; 2]>>>, 
        data_security4: Arc<Mutex<Vec<[f64; 2]>>>, 
        data_security5: Arc<Mutex<Vec<[f64; 2]>>>, 
        data_security6: Arc<Mutex<Vec<[f64; 2]>>>, 
    ) {
    let (mut socket, _response) = connect(Url::parse(BASE_URL).unwrap()).expect("Can't connect");
    println!("{:#?}", _response);

    let authorization = r#"{"action": "auth", "key": "{YOUR_SECRET_ID}", "secret": "YOUR_SECRET_KEY"}"#;
    let subscribe = r#"{"action":"subscribe","trades":["BTC/USD","ETH/USD","SOL/USD","LINK/USD","LTC/USD","MATIC/USD"],"quotes":[],"bars":[]}"#;

    socket.write_message(Message::Text(authorization.into())).unwrap();
    socket.write_message(Message::Text(subscribe.into())).unwrap();

    let mut counter: f64 = 0.0;

    loop {  
        let msg = socket.read_message().expect("Error reading message").to_string();
        if msg != "" {
            let msg_vector: Vec<serde_json::Value> = serde_json::from_str(&msg).unwrap(); 
            for m in msg_vector {
                if m["T"] == "t" {
                    if m["S"] == "ETH/USD" {
                        *security1.lock().unwrap() = m["p"].as_f64().unwrap();
                        let new_point: Vec<[f64; 2]> = vec!([counter, m["p"].as_f64().unwrap()]);
                        data_security1.lock().unwrap().extend(new_point);
                    }
                    if m["S"] == "SOL/USD" {
                        *security2.lock().unwrap() = m["p"].as_f64().unwrap();
                        let new_point: Vec<[f64; 2]> = vec!([counter, m["p"].as_f64().unwrap()]);
                        data_security2.lock().unwrap().extend(new_point);
                    }
                    if m["S"] == "LINK/USD" {
                        *security3.lock().unwrap() = m["p"].as_f64().unwrap();
                        let new_point: Vec<[f64; 2]> = vec!([counter, m["p"].as_f64().unwrap()]);
                        data_security3.lock().unwrap().extend(new_point);
                    }
                    if m["S"] == "LTC/USD" {
                        *security4.lock().unwrap() = m["p"].as_f64().unwrap();
                        let new_point: Vec<[f64; 2]> = vec!([counter, m["p"].as_f64().unwrap()]);
                        data_security4.lock().unwrap().extend(new_point);
                    }
                    if m["S"] == "MATIC/USD" {
                        *security5.lock().unwrap() = m["p"].as_f64().unwrap();
                        let new_point: Vec<[f64; 2]> = vec!([counter, m["p"].as_f64().unwrap()]);
                        data_security5.lock().unwrap().extend(new_point);
                    }
                    if m["S"] == "BTC/USD" {
                        *security6.lock().unwrap() = m["p"].as_f64().unwrap();
                        let new_point: Vec<[f64; 2]> = vec!([counter, m["p"].as_f64().unwrap()]);
                        data_security6.lock().unwrap().extend(new_point);
                    }
                    println!("{:#?}", &m);
                }
                counter += 1.0;
            }   
        }   
    }
}

fn main() {
    let app = App::new();

    let security1 = app.security1.clone();
    let security2 = app.security2.clone();
    let security3 = app.security3.clone();
    let security4 = app.security4.clone();
    let security5 = app.security5.clone();
    let security6 = app.security6.clone();
    let data_security1 = app.data_security1.clone();
    let data_security2 = app.data_security2.clone();
    let data_security3 = app.data_security3.clone();
    let data_security4 = app.data_security4.clone();
    let data_security5 = app.data_security5.clone();
    let data_security6 = app.data_security6.clone();
    std::thread::spawn(move || {
        alpaca(
            security1, 
            security2, 
            security3, 
            security4, 
            security5, 
            security6, 
            data_security1, 
            data_security2, 
            data_security3, 
            data_security4, 
            data_security5, 
            data_security6, 
        ); 
    });
    
    let tick = app.tick.clone();
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            *tick.lock().unwrap() += 1.0;
        }
    });

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Newton Financial Toolbox",
        native_options,
        Box::new(|_| Box::new(app)),
    );
}
