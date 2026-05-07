# 🌐 Network Emulation (netem)

Esta carpeta contiene los scripts para automatizar las pruebas de estrés en la red del sistema distribuido. Utiliza `tc netem` para inyectar latencia y pérdida de paquetes sobre el túnel cifrado de WireGuard (`wg0`).

## 🚀 Uso de los scripts

Antes de ejecutarlos por primera vez, dales permisos de ejecución:
```bash
chmod +x apply_netem.sh clear_netem.sh
```

Para iniciar la simulación (Latencia/Pérdida):
```bash
./apply_netem.sh
```

Para restaurar la red a su estado normal:
```bash
./clear_netem.sh
```

## 📊 Medición sugerida

Se recomienda mantener un servidor iperf3 -s activo en el Coordinador y realizar pruebas desde un nodo Edge con iperf3 -c <IP_COORDINADOR> antes y después de aplicar estos scripts para documentar la degradación del ancho de banda.
