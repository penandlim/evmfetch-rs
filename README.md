# evmfetch-rs

Fetches contract ABI and source code using the Etherscan API. Support Ethereum, Arbitrum, and Optimism.

## Overview

`evmfetch-rs` is a command-line tool written in Rust that allows you to retrieve contract ABI and source code using the Etherscan API. It provides an easy way to interact with the Ethereum blockchain and extract essential contract information.

## Features

- Fetch contract ABI from Etherscan API
- Fetch contract source code from Etherscan API
- Specify the chain ID and contract address as command-line arguments
- Outputs the retrieved contract ABI or source code

## Prerequisites

- Rust (stable version)
- Etherscan API key (obtainable from [Etherscan](https://etherscan.io/apis))

## Installation

1. Clone the repository:

   ```shell
   git clone https://github.com/penandlim/evmfetch-rs.git
   ```

2. Build the project:

   ```shell
   cd evmfetch-rs
   cargo build --release
   ```

## Usage

Run the CLI tool with the desired command, chain ID, and contract address:

```shell
cargo run --release abi <chain_id> <contract_address> <api_key>
cargo run --release code <chain_id> <contract_address> <api_key>
```

Replace `<chain_id>` with the desired chain ID (e.g., `1` for Ethereum Mainnet) and `<contract_address>` with the address of the contract you want to fetch information for. You must specify `<api_key>` since the Etherscan API requires an API key. You can obtain an API key from 

* [Ethereum Etherscan](https://etherscan.io/apis).
* [Arbitrum Etherscan](https://arbiscan.io/apis).
* [Optimism Etherscan](https://optimistic.etherscan.io/apis).

Example usage:

```shell
cargo run --release abi 1 0xContractAddress <api_key>
cargo run --release code 1 0xContractAddress <api_key>
```

## License

This project is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions, please open an issue or submit a pull request.