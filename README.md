
# Ohms Law & Power Equation Training Application

This project is a web application designed to help users learn and practice Ohms Law and power equations. It consists of a backend API server and a frontend user interface, built with modern web technologies.

## Project Structure

The application is divided into two main parts:

1.  **Backend API Server (Rust/Rocket.rs)**: A robust and efficient server that generates exercises based on Ohms Law and power equations.
2.  **Frontend UI (Vuetify/Vue3/TypeScript)**: A user-friendly and responsive interface that interacts with the backend API, providing an interactive learning experience. The frontend also showcases the use of internationalization (i18n) features within the Vue framework.

## Features

-   **Exercise Generation**: Dynamically generates a variety of exercises related to Ohms Law and power equations.
-   **Interactive UI**: Provides a seamless and engaging user interface for solving exercises.
-   **Multilingual Support**: Demonstrates i18n capabilities, making the application accessible to a wider audience.

## Technologies Used

-   **Backend**:
    -   **Rust**: A systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.
    -   **Rocket.rs**: A web framework for Rust that makes it simple to write fast web applications without sacrificing flexibility or type safety.
-   **Frontend**:
    -   **Vue.js (v3)**: A progressive JavaScript framework for building user interfaces.
    -   **Vuetify**: A Material Design component framework for Vue.js.
    -   **TypeScript**: A typed superset of JavaScript that compiles to plain JavaScript.
    -   **Vue i18n**: Internationalization plugin for Vue.js.

## Prerequisites

Before you begin, ensure you have met the following requirements:

-   **Rust** and **Cargo** installed. You can install them from the official [Rust website](https://www.rust-lang.org/tools/install).
-   **Node.js** and **npm** installed. You can download them from the official [Node.js website](https://nodejs.org/).

## Getting Started

To get a local copy up and running, follow these simple steps:

### Backend (Rust/Rocket.rs)

1.  Navigate to the backend directory at the repository root.
2.  Build and run the project using Cargo:

    ```bash
    cargo run
    ```

    The server will start on the default Rocket port (8000).

### Frontend (Vuetify/Vue3/TypeScript)

1.  Navigate to the frontend directory:

    ```bash
    cd frontend
    ```

2.  Install the dependencies:

    ```bash
    npm install
    ```

3.  Start the development server:

    ```bash
    npm run serve
    ```

    The application will be available at `http://localhost:3000/` (or another port if 3000 is in use).

## Usage

Once both the backend and frontend are running, open your web browser and go to `http://localhost:3000/` (or the port where the frontend is running). You can now start interacting with the application, solving exercises, and exploring the i18n features.

The frontend server proxies all `/api/*` calls to the backend at port 8000.

## Building for production

In production/release mode the Rocket web server serves the frontend files (built with `npm run build`) from a folder configurable via environment variable `PUIMURI_FRONTEND_DIR`, but by default from `./static/`.

See Dockerfile for an example of this.

## Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1.  Fork the Project
2.  Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3.  Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4.  Push to the Branch (`git push origin feature/AmazingFeature`)
5.  Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Contact

Antti Peltonen - [bcow@iki.fi](mailto:bcow@iki.fi)

Project Link: [https://github.com/braincow/puimuri-trainer](https://github.com/braincow/puimuri-trainer)

## Acknowledgements

-   [Rust Community](https://www.rust-lang.org/)
-   [Axum](https://github.com/tokio-rs/axum)
-   [Vue.js](https://vuejs.org/)
-   [Vuetify](https://vuetifyjs.com/)
-   [Vue i18n](https://kazupon.github.io/vue-i18n/)
