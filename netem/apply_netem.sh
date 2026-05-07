#!/bin/bash

# Interfaz de red de la VPN
INTERFACE="wg0"

echo "🌐 Iniciando simulación de red adversa en $INTERFACE..."

# 1. Limpiar cualquier regla previa para evitar conflictos
sudo tc qdisc del dev $INTERFACE root 2> /dev/null

# 2. Aplicar latencia (100ms)
echo "⏱️  Inyectando latencia de 100ms..."
sudo tc qdisc add dev $INTERFACE root netem delay 100ms

# Si quisieras aplicar pérdida de paquetes en lugar de latencia, 
# descomenta la siguiente línea y comenta la anterior:
# echo "📉 Inyectando 10% de pérdida de paquetes..."
# sudo tc qdisc add dev $INTERFACE root netem loss 10%

echo "✅ Reglas de netem aplicadas con éxito."
echo "Puedes comprobar el estado con: tc qdisc show dev $INTERFACE"
