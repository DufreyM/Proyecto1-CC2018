# Proyecto Final — Videojuego Interactivo con Cámara, Minimap y Efectos Visuales

## Descripción General

Este proyecto consiste en el desarrollo de un **videojuego interactivo**, implementado con enfoque en **rendimiento, estética visual, control fluido y elementos inmersivos**.  
Fue diseñado para ejecutar de forma estable alrededor de **60 FPS**, integrando efectos visuales, audio ambiental, cámara libre y elementos de interfaz como minimapa y pantallas de transición.

El juego fue **implementado en hardware distinto a una computadora tradicional**, demostrando su versatilidad y capacidad de optimización.  
Ejemplo: ejecutado en una Raspberry Pi, microcontrolador con salida HDMI o dispositivo embebido con GPU integrada.

---

## Características Implementadas

### 1. Soporte de Control ( +20 pts )
- Integración de **control externo (gamepad / joystick)** para mover al jugador y realizar acciones dentro del juego.
- Detección dinámica de conexión/desconexión del control.

---

### 2. Estética del Nivel ( +30 pts )
- Se diseñó un entorno visual coherente con la temática del nivel.
- Uso de **texturas personalizadas, iluminación simulada y gradientes** para dar profundidad visual.
- Capas visuales con efectos diferenciados (terreno, objetos, ambiente, cielo).

---

### 3. Rendimiento y FPS ( +15 pts )
- El motor despliega los **FPS en pantalla**, manteniendo un promedio estable de **~60 FPS**.
- Se optimizó la carga de texturas y el renderizado para reducir tiempos de dibujo por frame.

---

### 4. Efectos Visuales ( +15 pts )
Se implementó un efecto visual especial, elegido según la temática del juego:
- **Linterna / Fog of War:** el jugador solo ve el área cercana a su posición, generando atmósfera de exploración y tensión.
- **Efecto de daño o ansiedad:** la pantalla vibra o tiembla cuando el jugador recibe daño o entra en zonas peligrosas.

---

### 5. Cámara Dinámica ( +20 pts )
- Implementación de una **cámara en tercera persona o primera persona** que sigue al jugador.
- Rotación horizontal mediante **movimiento del mouse**, brindando mayor libertad visual y sensación de inmersión.

---

### 6. Minimap ( +10 pts )
- Se agregó un **minimapa funcional** en una esquina de la pantalla (no junto al mapa principal).
- Muestra:
  - Posición y dirección del jugador.
  - Elementos del entorno relevantes (enemigos, objetivos, zonas seguras).
  - Escalado automático del área visible según el tamaño del nivel.

---

### 7. Música y Sonido ( +15 pts )
- **Música de fondo** ambiental acorde al tema del nivel (+5 pts).
- **Efectos de sonido** para acciones (pasos, daño, apertura de puertas, colisiones, etc.) (+10 pts).

---

### 8. Animaciones ( +20 pts )
- Implementación de **animaciones en sprites**.  
  Ejemplo: movimiento del jugador, enemigos o elementos del entorno (fuego, agua, luces, etc.).
- Animaciones sincronizadas con la lógica del juego (colisiones, daño, interacción).

---

### 9. Pantallas del Juego ( +25 pts )
- **Pantalla de Bienvenida** (+5 pts)  
  Incluye logotipo, título y efectos de entrada animados.
- **Selección de Niveles** (+10 pts)  
  Permite elegir entre varios mundos o escenarios antes de iniciar la partida.
- **Pantalla de Éxito / Victoria** (+10 pts)  
  Aparece cuando se cumple una condición (por ejemplo: recolectar todos los objetos, llegar a la meta o derrotar al enemigo final).

---

### 10. Sistema de Vida del Jugador ( +5 pts )
- Implementación de **barra de vida** visible en la interfaz.
- Reducción de salud al recibir daño o entrar en áreas peligrosas.
- Integración con efectos visuales (pantalla roja o sonido de alerta al perder vida).

---

## Requisitos Técnicos

- FPS promedio: **60**
- Soporte para **teclado, mouse y control**
- Interfaz adaptable a resoluciones **720p o 1080p**
- Audio y texturas optimizadas para carga rápida

---

## Ejecución

1. Compila el proyecto o ejecuta el binario principal.
   cargo run 
2. Conecta el control (opcional).
3. Desde la pantalla inicial, selecciona un nivel.
4. Explora el entorno, evita daño y completa los objetivos.
5. Al ganar, se mostrará la pantalla de éxito.

---

## Créditos

**Desarrollador:** Leonardo Dufrey Mejía Mejía  
**Curso:** Gráficas por computadora

