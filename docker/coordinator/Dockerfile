FROM rust:1.75

# Instalación de dependencias del sistema necesarias para ZeroMQ
# pkg-config: permite a Rust encontrar librerías de C
# libzmq3-dev: archivos de desarrollo de ZeroMQ
# iproute2 e iperf3: requeridos para simulación de red y mediciones>
RUN apt-get update && apt-get install -y \
    pkg-config \
    libzmq3-dev \
    iproute2 \
    iperf3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copiamos el workspace completo (necesario para el Cargo.toml raíz)
COPY . .

# Ahora la compilación ya no fallará al buscar libzmq
RUN cargo build --package coordinator

CMD ["./target/debug/coordinator"]
