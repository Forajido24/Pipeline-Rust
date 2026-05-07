FROM rust:1.75

# Instalación de dependencias del sistema para ZeroMQ y herramientas de red >
RUN apt-get update && apt-get install -y \
    pkg-config \
    libzmq3-dev \
    iproute2 \
    iperf3 \
    iputils-ping \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copiamos el workspace completo (necesario para el Cargo.toml raíz y depend>
COPY . .

# Compilación del paquete específico del edge
RUN cargo build --package edge

# El binario se ejecuta desde la carpeta de salida de debug del workspace
CMD ["./target/debug/edge"]
