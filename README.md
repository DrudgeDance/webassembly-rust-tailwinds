# Rust + WebAssembly Hello World

A simple web application with a Rust backend and WebAssembly frontend.

## Prerequisites

- Rust (installed via Homebrew)
- Trunk (installed via `cargo install trunk`)
- wasm-bindgen-cli (installed via `cargo install wasm-bindgen-cli`)

## Running the Application

1. Start the backend server:
   ```bash
   cd backend
   cargo run
   ```

2. In a new terminal, start the frontend development server:
   ```bash
   cd frontend
   trunk serve
   ```

3. Open your browser and visit:
   - Frontend: http://localhost:8080
   - Backend API: http://localhost:8080/api/hello

## Project Structure

- `backend/`: Actix-web server providing the API
- `frontend/`: Yew-based WebAssembly frontend 