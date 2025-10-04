# Event Stream - Ethereum Transfer Event Listener

A program that listens to Sepolia USDC Transfer events and prints them to std output.

Before running this application, make sure you have:

- [Rust](https://rustup.rs/) installed (version 1.85)
- Access to an Ethereum Sepolia testnet RPC endpoint (WebSocket)

## Setup

**Configure your RPC endpoint:**

Create a `.env` file in the project root:

```bash
cp .env.example .env
```

Then edit the `.env` file and add your RPC URL:

```
RPC_URL=wss://your-ethereum-sepolia-websocket-endpoint
```

**Alternative:** Set the environment variable directly:

```bash
export RPC_URL="wss://your-ethereum-sepolia-websocket-endpoint"
```

## Running the Application

### Building

```bash
cargo build
```

for production

```bash
cargo build --release
```

### Running

```bash
cargo run
```

for production

```bash
cargo run --release
```

## Alloy dependency

I decided to use alloy as it is the maintained, high performance library that provided all needed functionalities out of the box.
It's currently the industery standard library.
