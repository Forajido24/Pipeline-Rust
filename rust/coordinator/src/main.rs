use serde::{Serialize, Deserialize};
use tokio::time::{self, Duration};
use std::collections::HashMap;
use chrono::Utc;
use zmq;

#[derive(Serialize, Deserialize, Debug)]
struct EdgeReport {
    edge_id: String,
    window_avg: f64,
    anomaly_detected: bool,
    sample_count: usize,
    latency_ms: i64,
}

struct NodeStatus {
    last_seen: i64,
    is_active: bool,
}

#[tokio::main]
async fn main() {
    println!("========================================");
    println!("   COORDINADOR: MONITOREO ACTIVADO      ");
    println!("========================================");

    // 1. Configuración de ZeroMQ (Socket real)
    let context = zmq::Context::new();
    let socket = context.socket(zmq::PULL).unwrap();
    let slash = "/";
    let address = format!("tcp:{}{}0.0.0.0:8080", slash, slash);
    socket.bind(&address).expect("Error bindeando puerto 8080");

    // Configuramos el socket para que no bloquee todo el hilo de Tokio
    socket.set_rcvtimeo(100).unwrap();

    let mut tracked_edges: HashMap<String, NodeStatus> = HashMap::new();
    let mut check_interval = time::interval(Duration::from_secs(5));

    loop {
        tokio::select! {
            // 2. Recepción de mensajes REALES de la red
            _ = async {
                // Intentamos recibir un mensaje del socket
                if let Ok(Ok(msg)) = socket.recv_string(0) {
                    if let Ok(report) = serde_json::from_str::<EdgeReport>(&msg) {
                        let now = Utc::now().timestamp_millis();

                        // Actualizamos el estado del nodo (Esto actúa como Heartbeat)
                        tracked_edges.entry(report.edge_id.clone())
                            .and_modify(|status| {
                                status.last_seen = now;
                                if !status.is_active {
                                    status.is_active = true;
                                    println!("♻️ [RECONEXIÓN] Nodo {} ha vuelto", report.edge_id);
                                }
                            })
                            .or_insert(NodeStatus { last_seen: now, is_active: true });

                        // Imprimimos el reporte recibido
                        let icon = if report.anomaly_detected { "⚠️" } else { "✅" };
                        println!("{} Reporte de {}: Avg={:.2}", icon, report.edge_id, report.window_avg);
                    }
                }
                // Pequeña pausa para no saturar el CPU si el socket está vacío
                time::sleep(Duration::from_millis(10)).await;
            } => {}

            // 3. Tu lógica de detección de fallos (Mantenida)
            _ = check_interval.tick() => {
                let now = Utc::now().timestamp_millis();
                for (id, status) in tracked_edges.iter_mut() {
                    if status.is_active && (now - status.last_seen) > 10000 {
                        status.is_active = false;
                        println!("❌ [FALLA] Nodo {} desconectado (sin reportes > 10s)", id);
                    }
                }
            }
        }
    }
}
