# 🦀 Rust Metaball Simulator

A high-performance, real-time metaball simulation built with **Rust**, **Macroquad**, and **GLSL**. This project uses GPU-accelerated shaders to render organic, merging shapes with a smooth, fluid aesthetic.

This version has been specifically optimized for **Intel Arc**, **NVIDIA**, and **AMD** GPUs by ensuring strict GLSL precision matching and constant-bound loop structures.

---

## 🚀 Features

* **⚡ GPU-Powered:** Heavy math is offloaded to a fragment shader for buttery-smooth performance.
* **🎨 Interactive UI:** Real-time sliders for speed and radius, plus an integrated color picker.
* **📦 Cross-Platform:** Optimized to run on NVIDIA, AMD, and Intel Arc GPUs (includes strict GLSL compliance).
* **⌨️ Dynamic Spawning:** Add up to 100 metaballs on the fly.

## 🛠️ Tech Stack

* **Language:** Rust
* **Graphics Library:** [Macroquad](https://macroquad.rs/)
* **UI Library:** [egui-macroquad](https://github.com/optozorax/egui-macroquad)
* **Shaders:** GLSL (OpenGL ES 2.0/3.0 compatible)

---

## 🎮 Controls & Interface

### ⌨️ Keyboard Shortcuts
| Key | Action |
| :--- | :--- |
| **Space** | Spawn a new metaball at a random position (Max 100) |
| **Tab** | Toggle the visibility of the Settings Menu |

### 🎛️ Settings Menu
The UI allows you to manipulate the physics and appearance of the simulation in real-time:

* **Speed Slider:** Adjusts how fast the balls move. Uses frame delta time for consistent motion across different monitors.
* **Ball Radius Slider:** Changes the physical size of the balls. Larger balls create more "sticky" merging effects.
* **Color Picker:** Change the color of the metaballs instantly using the RGB selector.
* **Clear Balls Button:** Instantly wipes all active metaballs from the screen.

---

## 🔬 How it Works

The "sticky" merging effect is achieved through a **Fragment Shader** that calculates a "field" value for every pixel on the screen.

For each pixel, the shader sums the influence of all active metaballs. Unlike standard circles with hard edges, metaballs use an inverse square law. The influence ($I$) of a single ball is calculated as:

$$I = \frac{R^2}{d^2}$$

Where **$R$** is the radius and **$d$** is the distance from the pixel to the ball's center. The shader sums these values for all balls:

$$Total = \sum_{i=0}^{n} \frac{R_i^2}{(x - x_i)^2 + (y - y_i)^2}$$

If the **Total** exceeds a specific threshold (set to 1.0 in this project), the pixel is colored. When two balls get close, their fields overlap and sum together, causing the "bridge" between them to pass the threshold and merge the shapes before they even touch.

---

## ⚙️ Installation & Running

1.  **Clone the repo:**
    ```bash
    git clone [https://github.com/connerbolton/Rust-Metaball-Simulator.git](https://github.com/connerbolton/Rust-Metaball-Simulator.git)
    cd Rust-Metaball-Simulator
    ```

2.  **Run the Program:**
    ```bash
    cargo run --release
    ```
3. **Spawn in Some Metaballs**

   Press **SPACE** to spawn in some balls.
