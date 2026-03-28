Here's a comprehensive list of the best Rust crates for media editing and processing as of March 2026, organized by category:

---

## 🖼️ Image Processing & Manipulation

### 1. **`image`** (v0.25.10) — The Standard Image Library
This crate provides native Rust implementations of image encoding and decoding as well as some basic image manipulation functions. It provides a unified interface for image encodings and simple generic buffers for their content. It's possible to use either feature without the other. The focus is on a small and stable set of common operations that can be supplemented by other specialized crates. With ~7.3M downloads, it's the most widely used image crate in Rust.

### 2. **`imageproc`** — Advanced Image Processing Operations
Image processing operations are provided by the `imageproc` crate under the image-rs organization, which builds on top of `image` to offer filtering, edge detection, drawing, morphology, and more. Actively maintained (last update March 2026).

### 3. **`fast_image_resize`** — SIMD-Accelerated Image Resizing
A Rust library for fast image resizing with using of SIMD instructions. This crate provides the `PixelComponentMapper` structure that allows you to create colorspace converters for images whose pixels based on u8 and u16 components. Great for high-performance resizing tasks.

### 4. **`photon`** (photon_rs) — High-Performance Image Processing + WASM
Photon is a high-performance Rust image processing library, which compiles to WebAssembly, allowing for safe, blazing-fast image processing both natively and on the web. Pure Rust — unlike other libraries, 100% of the library's codebase is written in Rust, so security and safety is guaranteed. Great for filters, channel manipulation, and effects.

### 5. **`ril`** (Rust Imaging Library) — High-Level Image & Animation Processing
RIL is a Rust crate designed to provide an easy-to-use, high-level interface around image processing in Rust. Image and animation processing has never been this easy before. RIL supports high-level encoding, decoding, and processing of animated images of any format, such as GIF or APNGs. Animated images can be lazily decoded, meaning you can process frames one by one, leading to huge performance and memory gains.

### 6. **`image_compressor`** — Image Compression with MozJPEG
Compress and resize a single image to JPG format. Multithreading. Customize the quality and size ratio of compressed images. Uses MozJPEG under the hood for excellent JPEG compression.

### 7. **`resize`** — Simple Pure-Rust Image Resampling
Simple image resampling library in pure Rust. Lightweight and focused on just resizing/resampling.

### 8. **`imagequant`** (pngquant) — PNG Lossy Compression
Convert 24/32-bit images to 8-bit palette with alpha channel. For lossy PNG compression and high-quality GIF images.

---

## 🎬 Video Processing

### 9. **`ffmpeg-next`** — Safe FFmpeg Wrapper (High-Level Bindings)
This is a fork of the abandoned ffmpeg crate. This crate is currently in maintenance mode, and aims to be compatible with all of FFmpeg's versions from 3.4 (currently from 3.4 til 8.0). The most popular high-level safe FFmpeg wrapper for Rust.

### 10. **`rsmpeg`** — Thin & Safe FFmpeg Bindings (by Lark/Bytedance)
rsmpeg is a thin and safe layer above the FFmpeg's Rust bindings; its main goal is to safely expose FFmpeg inner APIs in Rust as much as possible. Supported FFmpeg versions are 6.*, 7.*. Minimum Supported Rust Version is 1.81.0 (stable channel). Excellent for building custom video/audio processing pipelines.

### 11. **`ffmpeg-sidecar`** — FFmpeg CLI Wrapper with Iterator Interface
By wrapping the CLI, this crate avoids the downsides of low-level bindings, and also solves some of the pain points — raw data can easily move in and out of FFmpeg instances, or pipe between them. It can decode H265, modify the decoded frames, and then write back to H265. Great for quick integrations without needing to link FFmpeg at compile time.

### 12. **`rav1e`** — AV1 Video Encoder
Listed on lib.rs as a top video crate, `rav1e` is a pure-Rust, safe, fast AV1 encoder used in production. Perfect if you need to encode to the modern AV1 format.

---

## 🔊 Audio Processing

### 13. **`rodio`** — Audio Playback Library
Rodio is a simple and easy-to-use audio playback library for Rust. It supports various audio formats, including WAV, MP3, and FLAC. With rodio, you can play audio files from disk, memory, or network streams. It also provides a simple API for controlling playback, such as pausing, resuming, and stopping audio playback.

### 14. **`audio-processor`** — Modular Audio Processing (FFmpeg-backed)
A modular audio processing crate for Rust that leverages FFmpeg to perform a wide range of audio operations. The crate is designed for ease of integration and reuse in various projects. It provides a set of functions for audio editing—such as trimming, seeking, transcoding, applying effects, merging, reversing, normalizing, and overlaying audio. This is essentially the "Swiss army knife" for audio editing tasks like cutting, trimming, and converting.

### 15. **`cpal`** — Low-Level Cross-Platform Audio I/O
Low-level cross-platform audio I/O library in pure Rust. The go-to crate for audio capture and output at the device level.

### 16. **`RustFFT`** — Fast Fourier Transform
RustFFT is a fast and reliable FFT implementation written in Rust. This crate is perfect for audio processing applications that require real-time processing of audio signals. RustFFT is designed to be easy to use and provides a simple API.

---

## 🆕 Cutting-Edge (March 2026)

### 17. **`OxiMedia`** (v0.1.0) — Pure Rust FFmpeg + OpenCV Replacement ⭐ NEW
Today they're releasing OxiMedia v0.1.0 — an open-source multimedia and computer vision framework written entirely in Rust. It reconstructs the capabilities of both FFmpeg and OpenCV from the ground up, without a single line of C or Fortran, without patent-encumbered codecs, and without unsafe code. 92 crates. ~1.36 million lines of Rust. One `cargo add`. Single binary. No DLL dependencies. No system library requirements. No `brew install ffmpeg opencv`. Just `cargo add oximedia`. OxiMedia targets wasm32-unknown-unknown natively, enabling browser-based multimedia processing without server-side transcoding. Note: this is a brand-new v0.1.0 release, so maturity is still growing.

---

## Summary Table

| Task | Recommended Crate(s) |
|---|---|
| **Image decode/encode** | `image`, `ril` |
| **Image resize** | `fast_image_resize`, `resize`, `image` |
| **Image compression** | `image_compressor`, `imagequant`, `photon` |
| **Image filters/effects** | `photon`, `imageproc` |
| **Image cropping/subregion** | `image` (SubImage), `ril` |
| **Video transcode/cut/edit** | `rsmpeg`, `ffmpeg-next`, `ffmpeg-sidecar` |
| **Video encoding (AV1)** | `rav1e` |
| **Audio trim/cut/merge** | `audio-processor` |
| **Audio playback** | `rodio` |
| **Audio I/O (low-level)** | `cpal` |
| **Audio DSP/FFT** | `rustfft` |
| **All-in-one (new!)** | `oximedia` |

For most media editing projects, a combination of **`image`** + **`ffmpeg-sidecar`** (or **`rsmpeg`**) + **`audio-processor`** will cover the vast majority of use cases. If you want a pure-Rust, zero-C-dependency future, keep an eye on **OxiMedia**.
