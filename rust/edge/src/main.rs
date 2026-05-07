use std::collections::VecDeque;
use std::env;
use std::time::Duration;
use tokio::time;
use zmq;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct SensorReading {
    sensor_id: String,
    timestamp_ms: u64,
    value: f64,
    unit: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct EdgeReport {
    edge_id: String,
    window_avg: f64,
    anomaly_detected: bool,
    sample_count: usize,
    latency_ms: u64,
}

#[tokio::main]
async fn main() {
    // 1. Configuración de Identidad y Red
    let edge_id = env::var("EDGE_ID").unwrap_or_else(|_| "edge-01".to_string());

    // Obtenemos la dirección del coordinador (IP o IP:PUERTO)
    let coordinator_addr = env::var("COORDINATOR_IP").unwrap_or_else(|_| "10.0.0.1:9000".to_string());

    let threshold = 35.0;
    let slash = "/";
    let context = zmq::Context::new();

    // 2. Configuración del Socket SUB (Recibir del Sensor local)
    let subscriber = context.socket(zmq::SUB).unwrap();

    // IMPORTANTE: Conectamos a sensor-03 según el docker-compose.yml
    let sensor_addr = format!("tcp:{}{}sensor-01:5555", slash, slash);

    subscriber.connect(&sensor_addr).expect("❌ Error conectando al Sensor local");
    subscriber.set_subscribe(b"").unwrap();
    subscriber.set_rcvtimeo(100).unwrap(); // Timeout de 100ms para no bloquear el hilo

    // 3. Configuración del Socket PUSH (Enviar al Coordinador vía Wireguard)
    let pusher = context.socket(zmq::PUSH).unwrap();

    // Construcción dinámica de la URL: Si ya tiene puerto, no lo agregamos
    let coord_target = if coordinator_addr.contains(':') {
        format!("tcp:{}{}{}", slash, slash, coordinator_addr)
    } else {
        format!("tcp:{}{}{}:9000", slash, slash, coordinator_addr)
    };

    pusher.connect(&coord_target).expect("❌ Error conectando al Coordinador remoto");

    // 4. Inicialización del Buffer para Promedio Móvil
    let mut window: VecDeque<f64> = VecDeque::with_capacity(10);

    println!("==========================================");
    println!("🚀 NODO EDGE [{}] INICIADO", edge_id);
    println!("📡 Suscrito a sensor en: {}", sensor_addr);
    println!("📤 Reportando a Hub en : {}", coord_target);
    println!("⚠️  Umbral de alerta   : {}°C", threshold);
    println!("==========================================");

    loop {
        // 5. Recepción de datos del Sensor
        match subscriber.recv_string(0) {
            Ok(Ok(msg)) => {
                if let Ok(reading) = serde_json::from_str::<SensorReading>(&msg) {

                    // Lógica de Ventana Deslizable (Promedio Móvil)
                    if window.len() >= 10 {
                        window.pop_front();
                    }
                    window.push_back(reading.value);

                    let avg: f64 = window.iter().sum::<f64>() / window.len() as f64;
                    let is_anomaly = avg > threshold;

                    // 6. Creación del Reporte para el Hub
                    let report = EdgeReport {
                        edge_id: edge_id.clone(),
                        window_avg: (avg * 100.0).round() / 100.0,
                        anomaly_detected: is_anomaly,
                        sample_count: window.len(),
                        latency_ms: 5, // Valor base para el Sprint 1
                    };

                    // 7. Serialización y Envío Remoto
                    if let Ok(json) = serde_json::to_string(&report) {
                        // Imprimimos el estado normal
                        println!("📊 [ID: {}] Valor actual: {:.2}°C | Promedio Móvil: {:.2}°C",
                                  reading.sensor_id, reading.value, report.window_avg);

                        // ¡Si hay anomalía, imprimimos una advertencia gigante en rojo/sirenas!
                        if is_anomaly {
                            println!("🚨 ¡ALERTA CRÍTICA! Promedio superó el umbral de {}°C -> Enviando evento de emergencia al Hub...", threshold);
                        }

                        if let Err(e) = pusher.send(&json, 0) {
                            println!("⚠️ Error de red al enviar al Coordinador: {}", e);
                        }
                    }
                }
            }
            Ok(Err(_)) => {
                // Error de decodificación de string
            }
            Err(zmq::Error::EAGAIN) => {
                // Timeout del socket, simplemente reintentamos en la siguiente iteración
            }
            Err(e) => {
                println!("❌ Error crítico en el socket SUB: {}", e);
            }
        }

        // 8. Control de flujo para no saturar el CPU del contenedor
        time::sleep(Duration::from_millis(10)).await;
    }
}








