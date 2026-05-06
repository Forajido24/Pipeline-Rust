use serde::{Serialize, Deserialize};
use tokio::time::{self, Duration};
use rand::Rng;
use chrono::Utc;
use std::env;
use zmq; 

#[derive(Serialize, Deserialize, Debug)]
struct SensorReading {
    sensor_id: String,
    timestamp_ms: u64, 
    value: f64,
    unit: String,
}

#[tokio::main]
async fn main() {
    let sensor_id = env::var("SENSOR_ID").unwrap_or_else(|_| "sensor-03".to_string());
    let interval_ms = 1000;
    let mut interval = time::interval(Duration::from_millis(interval_ms));

    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();
    
    let slash = "/";
    let addr = format!("tcp:{}{}*:5555", slash, slash);
    publisher.bind(&addr).expect("Error bindeando el puerto del sensor");

    println!("🚀 Sensor {} activo y publicando en el puerto 5555...", sensor_id);

    // Agregamos un contador de ciclos para simular las anomalías
    let mut cycle_counter = 0;

    loop {
        interval.tick().await;
        let mut rng = rand::thread_rng();
        
        // LÓGICA DE SIMULACIÓN DE ALERTAS:
        // 15 segundos de temperatura normal, seguidos de 5 segundos de sobrecalentamiento extremo.
        let temperature: f64 = if cycle_counter % 20 < 15 {
            rng.gen_range(20.0..25.0) // Estado normal y estable
        } else {
            rng.gen_range(45.0..55.0) // ¡Pico de calor extremo!
        };

        cycle_counter += 1;

        let reading = SensorReading {
            sensor_id: sensor_id.clone(),
            timestamp_ms: Utc::now().timestamp_millis() as u64, 
            value: (temperature * 100.0).round() / 100.0,
            unit: "Celsius".to_string(),
        };

        match serde_json::to_string(&reading) {
            Ok(json) => {
                publisher.send(&json, 0).unwrap();
                println!("📡 Publicando lectura: {:.2}°C", reading.value);
            }
            Err(e) => eprintln!("Error al serializar: {}", e),
        }
    }
}
