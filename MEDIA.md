Great question! Here's a comprehensive roundup of the latest and most relevant Rust crates for rendering **images, audio, video, and 3D content** in the terminal — including options that work in terminals with limited protocol support (like VS Code's integrated terminal).

---

## 🖼️ **Images in the Terminal**

### 1. **`ratatui-image`** (Updated Feb 2026)
An image widget for ratatui, supporting sixels, kitty, iterm2, and unicode-halfblocks. This is the go-to for TUI apps built with Ratatui.

- It tackles 3 general problems: querying the terminal for available graphics protocols (Sixels, iTerm2, Kitty), guessing by env vars, and querying the terminal with control sequences.
- Falls back to "halfblocks" which uses some unicode half-block characters with fore- and background colors — this is what makes it work in **VS Code's terminal** and other terminals without full graphics protocol support.

### 2. **`viuer`** (Latest: v0.9.2, March 2026)
viuer is a Rust library that makes it easy to show images in the terminal. It has a straightforward interface and is configured through a single struct.

- Kitty and iTerm graphic protocols are supported. Uses either iTerm or Kitty graphics protocol, if supported, and half blocks otherwise.
- Great for "dump" image display but viuer "dumps" the image, making it difficult to work for TUI programs.

### 3. **`viu`** (CLI frontend for viuer)
A small command-line application to view images from the terminal written in Rust. It is basically the front-end of viuer.

### 4. **`tui-image`**
An image viewer widget for tui-rs. RGB: Relies on a RGB compatible terminal to show filled blocks with full RGB color.

### 5. **`termimage`**
This Rust crate enables displaying images in your terminal. It automatically downscales images to fit the terminal's size and approximates their colors to match the terminal's display capabilities.

---

## 🎬 **Video & Audio in the Terminal**

### 6. **`tplay`** ⭐ (Updated ~Feb 2026)
This is the **most comprehensive media crate** for the terminal!

A media player that visualizes images and videos as ASCII art directly in the terminal (with sound).

- Converts and shows any media to ASCII art in the terminal. Supports images/gifs/videos/webcam and YouTube links.
- Works in terminals without graphics protocol support (like VS Code) since it uses **ASCII art** rendering.
- Requires dependencies like `ffmpeg`, `libmpv`, `yt-dlp`, etc. The crate can run on Windows and all prerequisites (ffmpeg) can be installed with vcpkg.

Install:
```bash
cargo install tplay
```

---

## 🧊 **3D in the Terminal**

### 7. **`tortuise`** 🔥🆕 (v1.80.0 — Published just 2 days ago!)
This is the **newest and most exciting crate** — literally published days ago!

Terminal-native 3D Gaussian Splatting viewer — render .ply and .splat scenes in your terminal. Tagged #3d, #gaussian-splatting, #rendering, #terminal, #tui.

A CPU-first 3D Gaussian Splatting viewer inspired by ratatui, built on crossterm. Fully parallelized rendering pipeline via rayon, perceptual color mapping, six render modes — all running on pure CPU.

Key highlights:
- Six render modes (halfblock, braille, ASCII, point cloud, matrix, block density), auto-detects truecolor vs 256-color terminals, modal Free/Orbit camera.
- Runs comfortably over SSH on a headless Mac Mini, even better locally — interactive framerates at reasonable terminal resolution on M2/M4.
- The pipeline is straightforward: load splats, project them into screen space, depth-sort, splat onto a framebuffer, then convert to terminal characters.
- Single photo to 3D in your terminal.

Since it uses **crossterm** and half-block/braille/ASCII modes, it can work in **limited terminals like VS Code's**!

Install:
```bash
cargo install tortuise
```

### 8. **`TermGL`**
TermGL — A terminal-based graphics library for 2D and 3D graphics. (Not Rust-native, but worth mentioning in the ecosystem.)

---

## 🏗️ **The TUI Foundation: `ratatui`**

All the above image/media widgets live in the **Ratatui** ecosystem:

Ratatui is a Rust crate for cooking up terminal user interfaces (TUIs). It provides a simple and flexible way to create text-based user interfaces in the terminal.

Starting with Ratatui 0.30.0, the project was reorganized into a modular workspace to improve compilation times, API stability, and dependency management.

---

## 📊 Summary Table

| Media Type | Crate | VS Code Terminal? | Status |
|---|---|---|---|
| **Image** | `ratatui-image` | ✅ (halfblocks fallback) | Active (Feb 2026) |
| **Image** | `viuer` | ✅ (halfblocks fallback) | Active (Mar 2026) |
| **Video + Audio** | `tplay` | ✅ (ASCII art) | Active (2025-2026) |
| **3D Gaussian Splatting** | `tortuise` 🆕 | ✅ (ASCII/braille/halfblock) | **Brand new** (Mar 2026) |
| **Image (widget)** | `tui-image` | ✅ (RGB blocks) | Maintained |

**`tortuise`** is the hottest new entry — it renders .ply and .splat scenes using CPU-only rasterization — halfblock characters, depth sorting, alpha compositing, all on the CPU — Rust + crossterm + rayon. Absolutely the latest crate doing groundbreaking media rendering in the terminal!
