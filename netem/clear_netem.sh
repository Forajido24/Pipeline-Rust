#!/bin/bash

INTERFACE="wg0"

echo "🧹 Limpiando reglas de simulación en $INTERFACE..."

# Eliminar la disciplina de cola (qdisc) raíz
sudo tc qdisc del dev $INTERFACE root 2> /dev/null

if [ $? -eq 0 ]; then
    echo "✅ Red restaurada a la normalidad (Sin latencia ni pérdida)."
else
    echo "⚠️  No había reglas activas de netem para limpiar."
fi
