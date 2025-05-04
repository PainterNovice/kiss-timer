# KISS Timer

KISS Timer is a simple countdown timer application built with Rust and Leptos. It allows users to set a timer for various durations trought url input.

## Usage

kiss-timer/?90        (90 seconds)

kiss-timer/?1:30      (1 minute 30 seconds)

kiss-timer/?1:05:30   (1 hour, 5 minutes, 30 seconds)


## Installation

To run the KISS Timer application locally, follow these steps:

1. **Clone the repository:**

   ```bash
   git clone https://github.com/painternovice/kiss-timer.git
   cd kiss-timer
   ```

2. **Install Rust and Cargo:**

   Make sure you have Rust and Cargo installed. You can install them from [rustup.rs](https://rustup.rs/).
   ```bash
    rustup toolchain install nightly
    rustup default nightly
    rustup target add wasm32-unknown-unknown
   ```

3. **Install Trunk**
    ```bash
    cargo install trunk --locked
    ```

4. **Local dev:**

   ```bash
   trunk serve --port 3000 --public-url "/kiss-timer/"
   ```

5. **Open your browser:**

   Navigate to `http://localhost:3000/kiss-timer/` or `http://127.0.0.1:3000/kiss-timer/` to view the application.



## Deployment

The application is deployed on GitHub Pages. You can access it at:

[https://painternovice.github.io/kiss-timer](https://painternovice.github.io/kiss-timer)

>If you would like to deploy on your own GitHub, make sure to change the workflow configuration in [./.github/workflows/gh-pages-deploy.yml](.github/workflows/gh-pages-deploy.yml) line 58 to your own github.io page.
>```bash
>run: ./trunk build --release --public-url "https://painternovice.github.io/${{ github.event.repository.name }}/"
>```

## Contributing

Contributions are welcome! If you have suggestions or improvements, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.