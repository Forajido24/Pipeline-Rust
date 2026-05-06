FROM rust:1.75-slim

# ESTA ES LA LÍNEA CLAVE: Instalamos las librerías del sistema
RUN apt-get update && apt-get install -y \
    libzmq3-dev \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

CMD ["./target/release/sensor"]
