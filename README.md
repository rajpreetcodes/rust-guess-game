# rust-guess-game

![License](https://img.shields.io/badge/License-MIT-blue.svg)

A basic rust guessing game to get fluent in rust.

## About The Project

This is a simple command-line guessing game built with Rust. The primary goal of this project is to serve as a hands-on exercise for learning the fundamentals of the Rust programming language.

It covers:
* Reading user input from the standard input.
* Using an external package (crate) - `rand`.
* Working with variables, types, and basic functions.
* Understanding Rust's package manager, Cargo.

## Getting Started

To get a local copy up and running, follow these simple steps.

### Prerequisites

You must have the Rust toolchain installed on your system. If you don't, you can install it easily via [rustup](https://rustup.rs/).

* rustc & cargo
    ```sh
    curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
    ```

### Usage

1.  **Clone the repository (or download the files)**
    ```sh
    # If you are using git
    git clone [https://github.com/your_username/rust-guess-game.git](https://github.com/your_username/rust-guess-game.git)
    ```
2.  **Navigate to the project directory**
    ```sh
    cd rust-guess-game
    ```
3.  **Run the application**
    The `cargo run` command will compile and execute the program. Cargo will automatically handle downloading the `rand` dependency for you.
    ```sh
    cargo run
    ```
4.  **Interact with the game**
    ```
    I will guess your number!
    Enter a number:
    50
    I guess your number is: 82
    You entered: 50
    ```

## License

Distributed under the MIT License. See the `LICENSE` file for more information.

---

**Next Steps:**

1.  **Create a `LICENSE` file:** Since you specified the MIT license, create a file named `LICENSE` (no extension) in your project directory and paste the full [MIT License text](https://opensource.org/licenses/MIT) into it.
2.  **(Optional) Create a Git repository:** If you want to share your project on a site like GitHub, you can run `git init` in your project folder to get started.
