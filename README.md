# 🕶️ PIZZA HERO: The Grand Restaurant

## El Juego: Producción y Entrega ZK

**PIZZA HERO** es una experiencia dividida en dos fases críticas que ponen a prueba tanto el ritmo como la precisión física del jugador, todo verificado mediante pruebas de conocimiento cero (ZK).

### 🏭 Fase 1: La Cocina (60 Segundos)
- **Estilo:** Rhythm Game (estilo Guitar Hero).
- **Controles:** Teclas `A`, `S`, `K`, `L`.
- **Objetivo:** Recolectar ingredientes al compás del ritmo para llenar la barra de masa (Dough Bar).
- **Resultado:** Cada barra completada genera **+1 Pizza**. El stock de pizzas preparadas es la base para la Fase 2.

### 🍽️ Fase 2: El Restaurante (Entrega Física)
- **Estilo:** Physics-based Delivery.
- **Controles:** Mouse / Touch (Drag & Flick).
- **Objetivo:** Lanzar las pizzas preparadas desde el mostrador hacia las mesas de los clientes.
- **Mecánica:** Se utiliza una trayectoria física con fuerza y dirección calculada por el gesto del jugador. Golpear una mesa otorga el máximo puntaje y "entrega" la orden.

## Stack Técnico
- **Game Engine:** Phaser.js (Fases secuenciales y motor de físicas).
- **Styling:** Tailwind CSS (UI de alta gama).
- **ZK Verification:** Noir (Circuitos para validar la producción legítima de pizzas y la precisión de entrega).
- **Blockchain:** Stellar (Contratos Soroban para el registro de récords y victorias).

---
*Consigliere Slice 🕶️🍕*
