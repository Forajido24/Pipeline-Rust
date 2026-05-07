#  Pipeline-Rust: Sistema Distribuido IoT

Pipeline-Rust es un proyecto de sistema distribuido enfocado en la recolección, procesamiento y orquestación de datos de telemetría (temperatura). Implementa una arquitectura de tres niveles (Sensor, Edge, Coordinator) desarrollada en **Rust** y completamente dockerizada.

Además, el entorno está diseñado para realizar pruebas de estrés de red simulando túneles VPN (WireGuard) y condiciones adversas mediante `tc netem`.

## 🏗️ Arquitectura del Sistema

El ecosistema está compuesto por tres microservicios principales que se comunican mediante **ZeroMQ**:

1. **📡 Sensor Node (`rust/sensor/`)**: 
   - Actúa como publicador (PUB) en el puerto `5555`.
   - Genera lecturas de temperatura simulando entornos reales y anomalías (ciclos de 15 segundos de temperatura normal entre 20-25°C, seguidos de 5 segundos de picos extremos entre 45-55°C).
   - Formato de payload: JSON con `sensor_id`, `timestamp_ms`, `value` y `unit`.

2. **⚡ Edge Node (`rust/edge/`)**:
   - Se suscribe a los nodos sensores.
   - Procesa los datos en el borde: calcula promedios móviles y detecta umbrales críticos de alerta (ej. > 35°C).
   - Reporta los datos ya filtrados y procesados al Coordinador.

3. **🧠 Coordinator Node (`rust/coordinator/`)**:
   - Actúa como hub centralizador.
   - Recibe la telemetría procesada de múltiples nodos Edge de manera concurrente.
   - Registra y monitorea el estado global del sistema en tiempo real.

## 🛠️ Tecnologías Utilizadas

- **Lenguaje:** Rust (v1.75)
- **Mensajería:** ZeroMQ (`libzmq3-dev`) + Serde (JSON)
- **Asincronía:** Tokio
- **Despliegue:** Docker & Docker Compose
- **Redes & Testing:** WireGuard (VPN), `tc netem` (Emulación de red), `iperf3` (Ancho de banda)

## 📂 Estructura del Repositorio

```text
Pipeline-Rust/
├── docker/                 # Configuraciones de contenedores
│   ├── coordinator/
│   ├── edge/
│   ├── sensor/             # Dockerfile (Instalación de dependencias ZeroMQ/SSL)
│   └── docker-compose.yml  # Orquestación de nodos
├── rust/                   # Código fuente de los microservicios
│   ├── coordinator/
│   ├── edge/
│   ├── sensor/             # Lógica principal del sensor (generación de datos)
│   └── shared/             # Librerías y estructuras compartidas
├── vpn/                    # Configuraciones de túneles WireGuard
├── netem/                  # Scripts para simulación de red adversa
└── .gitignore              # Exclusión de binarios, .env y certificados wg0
```
## 🚀 Instrucciones de Ejecución

### Prerrequisitos
- Docker y Docker Compose instalados.
- Puertos disponibles para la comunicación ZeroMQ (ej. 5555, 9000).

### Despliegue Local
Para levantar toda la arquitectura de nodos, ubícate en la raíz del proyecto y ejecuta:

```bash
# Navegar al directorio de nodos (si aplica según tu configuración de compose)
cd nodos

# Levantar los contenedores en segundo plano (detached mode)
sudo docker compose up -d

# Verificar el estado de los contenedores activos
sudo docker ps

# Ver los logs en tiempo real (ej. del coordinador)
sudo docker logs -f coordinator_node
```

Para detener el ecosistema y limpiar contenedores huérfanos:
```bash
sudo docker compose down --remove-orphans
```

## 🌐 Pruebas de Red (Network Emulation)

Este proyecto está preparado para pruebas de resiliencia. Se pueden inyectar reglas al túnel de la VPN (`wg0`) para evaluar el rendimiento de ZeroMQ bajo condiciones de red deficientes:

- **Inyección de Latencia (100ms):**
  ```bash
  sudo tc qdisc add dev wg0 root netem delay 100ms
  ```

- **Simulación de Pérdida de Paquetes (10%):**
  ```bash
  sudo tc qdisc change dev wg0 root netem loss 10%
  ```

*(Nota: Se recomienda usar `iperf3` para validar la degradación del enlace cifrado antes y después de aplicar las reglas de netem).*
