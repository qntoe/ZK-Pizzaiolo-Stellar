# 🧠 ZK-Pizzaiolo: Expert Logic & Physics Specification

## 1. Game Mechanics (Rhythm Logic)
- **Beat Sync:** El juego sincroniza la aparición de ingredientes con los BPM de la música (Mafia/Jazz/Synthwave).
- **Accuracy Window:** 
  - Perfect: ±20ms
  - Good: ±50ms
  - Miss: >100ms
- **Combo Multiplier:** Cada 10 aciertos perfectos, el multiplicador sube (x2, x3, x5).

## 2. Physics Logic (Topping Placement)
- **Gravity:** 9.81 m/s² (Standard) o ajustada para feeling arcade.
- **Colliders:** La masa de la pizza tiene un Mesh Collider circular. Cada ingrediente tiene un Sphere/Box Collider.
- **Landing Accuracy:** Se mide la distancia del centro del ingrediente al "target point" en la masa. Esto influye en el puntaje de "Estética".

## 3. ZK Proof Specification (Noir)
Para evitar trampas en el leaderboard, el circuito Noir probará:
- `Inputs`: Secuencia de inputs (tiempo, posición, precisión).
- `Witness`: Cálculo del puntaje final basado en la lógica anterior.
- `Public Output`: Score Final + Hash de la sesión.
- **Verification:** El contrato en Soroban solo acepta el score si la prueba es válida.

## 4. Smart Contract (Soroban)
- **Registry:** Mapeo de `Address -> BestScore`.
- **Verifier:** Integración con el verifier generado por Noir.
- **Rewards:** (Opcional) Distribución de tokens PizzaDAO por hitos.

---
*Consigliere Slice 🕶️🍕*
