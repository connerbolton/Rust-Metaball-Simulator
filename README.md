# 🦀 Rust Metaball Simulator

A high-performance, real-time metaball simulation built with **Rust**, **Macroquad**, and **GLSL**. This project uses GPU-accelerated shaders to render organic, merging shapes with a smooth, fluid aesthetic.

---

## 🚀 Features

* **⚡ GPU-Powered:** Heavy math is offloaded to a fragment shader for buttery-smooth performance.
* **🎨 Interactive UI:** Real-time sliders for speed, ball radius, and colors (via `egui`).
* **📦 Resolution Scaling:** Automatically adjusts rendering based on your window size.
* **⌨️ Dynamic Spawning:** Add up to 100 metaballs on the fly.

## 🛠️ Tech Stack

* **Language:** Rust
* **Graphics:** [Macroquad](https://macroquad.rs/)
* **UI:** [egui-macroquad](https://github.com/optozorax/egui-macroquad)
* **Shaders:** GLSL (Vertex & Fragment)

## 🎮 Getting Started

1. **Clone the repo:**
   ```bash
   git clone https://github.com/connerbolton/Rust-Metaball-Simulator.git
   cd Rust-Metaball-Simulator

2. **Run the Program:**
   ```bash
   cargo run