# Rust + Leptos + Tailwind Full-Stack Application

A modern full-stack web application built with Rust, Leptos, and WebAssembly (Wasm). This project demonstrates how to build high-performance web applications by compiling Rust to WebAssembly, combining the speed of native code with the reach of the web.

## Project Structure

```
my_first_rust_app/
├── ui-components/        # UI Component Library (compiles to Wasm)
│   ├── src/
│   │   ├── components/   # Reusable UI components
│   │   ├── layouts/      # Layout components
│   │   ├── theme/        # Theme system and providers
│   │   └── styles/       # Tailwind CSS styles
│   ├── package.json      # Node dependencies for Tailwind
│   └── tailwind.config.js
│
├── frontend/            # Main Application (compiles to Wasm)
│   └── src/
│       ├── features/    # Application features
│       └── services/    # API and business services
│
└── backend/            # Backend Server (native Rust)
```

## Features

- **WebAssembly-Powered**: Frontend code compiled to Wasm for near-native performance
- **Modern Stack**: Rust 🦀 for type-safe, high-performance code
- **Frontend Framework**: Leptos for reactive web applications with Wasm
- **Styling**: Tailwind CSS for utility-first styling
- **Theme System**:
  - Dynamic theme switching (Light/Dark modes)
  - Seasonal themes (Spring/Summer variations)
  - CSS variable-based theming
  - Reactive theme updates
- **Architecture**:
  - Modular UI components library (Wasm)
  - Feature-based frontend organization
  - Clean separation of concerns
  - Full-stack type safety
  - Zero-cost abstractions through Rust and Wasm
  - Excellent performance through WebAssembly compilation

## Prerequisites

- Rust (latest stable)
- Node.js & Yarn
- Trunk (`cargo install trunk`) - for building and bundling Wasm
- wasm32-unknown-unknown target (`rustup target add wasm32-unknown-unknown`)
- wasm-bindgen-cli (`cargo install wasm-bindgen-cli`) - for Wasm bindings

## Setup & Running

1. **Install Dependencies**:
   ```bash
   # Install Rust dependencies
   cargo build
   
   # Install UI library dependencies
   cd ui-components
   yarn install
   ```

2. **Start Development Servers**:

   In separate terminal windows:

   ```bash
   # Terminal 1: Start the backend server
   cd backend
   cargo run
   
   # Terminal 2: Watch and compile Tailwind CSS
   cd ui-components
   yarn watch:css
   
   # Terminal 3: Start the frontend development server
   cd frontend
   trunk serve --port 3000
   ```

   The application will be available at:
   - Frontend: http://localhost:3000
   - Backend API: http://localhost:8080

## Development Workflow

1. **UI Components**:
   - Add new components in `ui-components/src/components/`
   - Components are compiled to Wasm for optimal performance
   - Style using Tailwind CSS classes
   - Components are automatically available in the frontend
   - Use theme variables for consistent styling

2. **Theme Development**:
   - Theme definitions in `ui-components/src/theme/themes/`
   - Add new themes by creating theme files
   - Use CSS variables for dynamic theme updates
   - Test themes in both light and dark modes

3. **Frontend Features**:
   - Add new features in `frontend/src/features/`
   - Use components from the UI library
   - All code is compiled to Wasm for high performance
   - Add API calls in `frontend/src/services/`

4. **Backend Development**:
   - Implement API endpoints in `backend/src/`
   - Use shared types for full-stack type safety
   - Native Rust performance on the server

## Theme System

The application includes a robust theme system that supports:

- **Dynamic Themes**: Switch between light and dark modes
- **Seasonal Variations**: Spring and Summer theme variations
- **CSS Variables**: Theme colors are managed through CSS variables
- **Reactive Updates**: Themes update instantly without page reload
- **Consistent Styling**: All components use theme variables for colors

Theme variables available:
```css
--background      /* Background color */
--surface        /* Surface/card color */
--text           /* Primary text color */
--text-muted     /* Secondary text color */
--primary        /* Primary brand color */
--secondary      /* Secondary brand color */
--accent         /* Accent color */
--seasonal-primary    /* Season-specific primary */
--seasonal-secondary  /* Season-specific secondary */
--seasonal-accent     /* Season-specific accent */
```

## Performance Benefits

- **WebAssembly Execution**: Near-native performance in the browser
- **Small Bundle Size**: Rust code compiles to efficient Wasm binaries
- **Type Safety**: Catch errors at compile time, not runtime
- **Memory Safety**: Rust's ownership system prevents memory-related bugs
- **Zero-Cost Abstractions**: High-level code without runtime overhead
- **Efficient Theme Updates**: CSS variables for performant theme switching

## Building for Production

```bash
# Build the UI components
cd ui-components
yarn build:css

# Build the frontend
cd frontend
trunk build --release

# Build the backend
cd backend
cargo build --release
```

## Browser Support

The application works best in modern browsers that support:
- WebAssembly
- CSS Variables
- Modern JavaScript features

Note: Some browser console warnings about subresource integrity for preloaded resources are expected in Chrome and can be safely ignored.
