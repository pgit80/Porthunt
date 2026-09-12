# Porthunt 🎯

**Porthunt** is a lightweight, high-performance desktop application designed for developers to quickly identify, filter, and manage active network ports and their associated background processes.

Built with **Tauri v2**, **Rust**, and **React**, Porthunt combines system-level performance with a modern dark-mode user interface.

---

## Features

- 🔍 **Real-Time Port Scanning:** Instantly inspect active TCP/UDP ports and their listening processes.
- ⚡ **App & Framework Identification:** Automatically identifies frameworks and common services (e.g., Spring Boot, Node.js, React dev servers) bound to ports.
- 🎯 **Instant Search & Filtering:** Live filter by port number, process name, PID, or application type.
- 🛡️ **Safe Process Termination:** Securely kill processes locking target ports with built-in system safety rules.
- 📊 **Detailed Process Insights:** Expand rows to view executable file paths and complete execution arguments.
- 🌙 **Modern Dark Theme:** Optimized dark UI for high readability during dev sessions.

---

## Tech Stack

- **Desktop Framework:** [Tauri v2](https://tauri.app/)
- **Backend Core:** [Rust](https://www.rust-lang.org/)
- **Frontend UI:** [React](https://react.dev/) + [Vite](https://vitejs.dev/)
- **Styling:** Custom CSS (Slate Dark Palette)

---

## Prerequisites

Before running or building Porthunt locally, ensure you have the following installed:

- **Node.js** (v18 or higher) & `npm`
- **Rust Toolchain** (`rustc`, `cargo`) — Install via [rustup.rs](https://rustup.rs/)
- **Platform Build Tools:**
  - **Windows:** C++ Build Tools via Visual Studio Installer
  - **Linux:** WebKitGTK and development libraries (`libwebkit2gtk-4.1-dev`, `build-essential`, `libssl-dev`)
  - **macOS:** Xcode Command Line Tools (`xcode-select --install`)

---

## Getting Started

### 1. Clone the Repository
```bash
git clone [https://github.com/your-username/porthunt.git](https://github.com/your-username/porthunt.git)
cd porthunt