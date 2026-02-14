# BTC Wallet

> [!WARNING]
> This project move to [crypto-wallet](https://crates.io/crates/crypto-wallet)

## Description

This is a Rust-based tool that generates a Bitcoin wallet from a given text seed. It provides the private key in WIF (Wallet Import Format), the public key in hexadecimal format, and the corresponding P2PKH Bitcoin address.

## Features

-   Generate Bitcoin wallets from a text seed.
-   Support for compressed and uncompressed public keys.
-   Print the entire wallet (private key, public key, and address) or specific parts.
-   Option to print raw values for easy use in scripts.
-   Generate shell completions for `bash`, `elvish`, `fish`, `powershell`, and `zsh`.

## Getting Started

### Prerequisites

-   [Rust](https://www.rust-lang.org/tools/install)

### Building

1.  Clone the repository:
    ```sh
    git clone https://github.com/ZocketZero/btc-wallet-rust.git
    cd btc-wallet
    ```
2.  Build the project:
    ```sh
    cargo build --release
    ```
    The executable will be located in `target/release/btc-wallet`.

### Installation

You can install `btc-wallet` directly from `crates.io` using `cargo`:

```sh
cargo install btc-wallet
```

After installation, the `btc-wallet` executable will be available in your Cargo bin directory (usually `~/.cargo/bin`).

## Usage

### Generate a wallet

To generate a wallet, use the following command:

```sh
btc-wallet "your-secret-seed-phrase"
```

This will print the private key, public key, and Bitcoin address.

### Options

-   `--seed-text <TEXT>`: Text to use as a seed for the wallet generation.
-   `-p, --print <MODE>`: Print only a specific value.
    -   `all`: Print all information (default).
    -   `secret`: Print only the private key.
    -   `public`: Print the public key and address.
-   `-r, --raw`: Print only the raw value.
-   `-c, --compressed`: Generate a compressed public key (default is `false` for uncompressed).
-   `--completion <SHELL>`: Generate shell completion script for the specified shell.

### Examples

-   Generate a wallet with a compressed public key:
    ```sh
    btc-wallet "your-secret-seed-phrase" -c
    ```
-   Print only the Bitcoin address:
    ```sh
    btc-wallet "your-secret-seed-phrase" -p public -r
    ```
-   Generate a shell completion script for `bash`:
    ```sh
    btc-wallet --completion bash > /usr/share/bash-completion/completions/btc-wallet
    ```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
