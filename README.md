# Bark Token Program

Bark Token (BARK) is a Solana-based token program built with the Anchor framework, implementing the Solana 2022-Token Standard (Solana Extension). This program is created to manage Bark Tokens, a digital asset on the Solana blockchain, and is driven by community contributions.

## Features:

- **Token Creation:** Create Bark Tokens with details such as name, symbol, and maximum supply.
- **Minting:** Mint additional Bark tokens to the existing supply.
- **Burning:** Burn Bark tokens to reduce the total supply.
- **Pausing:** Pause and resume Bark token transfers as needed.
- **Transaction Fees:** Collect transaction fees and distribute them to a "community" treasury wallet.

## Bark Token (BARK) Tokenomics (draft)

#### Token Details:

| Attribute           | Value                  |
|---------------------|------------------------|
| Token Name          | Bark Token (BARK)      |
| Token Symbol        | BARK                   |
| Token Decimals      | 9                      |
| Maximum Supply      | 20,000,000,000         |
| Burning Rate        | 2% (Quarterly)         |
| Fee Decimals        | 2                      |
| Fee Symbol          | BARK_FEE               |
| Fee Percentage      | 2%                     |
| Treasury Wallet     | 8DosypWP5rR5REnpkjw... |
| Program Address     | [Program Address] |

**Owner:** bark8LXsP1oCtaFM2KdQpBvXgEVWPZ1nm5hecFFUFeX 
Link: https://solscan.io/account/bark8LXsP1oCtaFM2KdQpBvXgEVWPZ1nm5hecFFUFeX

#### Allocation and Distributions

### 2. Allocation and Distributions

| Category            | Percentage             |
|---------------------|------------------------|
| Public Sale         | 30%                    |
| Development         | 20%                    |
| Core Team           | 15%                    |
| Reserve             | 5%                     |
| Liquidity Pool      | 10%                    |
| Partnership         | 10%                    |
| Treasury            | 5%                     |


#### Vesting Schedule and Terms

| Category            | Vesting Period         | Cliff Period           |
|---------------------|------------------------|------------------------|
| Team                | 3 years                | 1 year                 |
| Ecosystem           | 2 years                | None                   |
| Reserve             | 1 year                 | None                   |

## Getting Started

### Prerequisites

Before you begin, make sure you have the following tools installed:

- [Rust](https://www.rust-lang.org/)
- [Solana CLI](https://docs.solana.com/cli/install)
- [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html)

### Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/bark-community/bark-token/bark-token-program.git
    ```

2. Navigate to the project directory:

    ```bash
    cd bark-token && bark-token-program
    ```

3. Build the program:

    ```bash
    cargo build --release
    ```

4. Deploy the program to Solana:

    ```bash
    solana program deploy target/deploy/bark_token.so
    ```

### Usage

1. Initialize the Bark Token:

    ```bash
    solana-tokens create-account <MINT_ADDRESS> <OWNER_ADDRESS> bark_token_program_id
    ```

2. Mint new tokens:

    ```bash
    solana-tokens mint <MINT_ADDRESS> <DEST_ADDRESS> <AMOUNT> --authority <AUTHORITY_ADDRESS>
    ```

3. Burn tokens:

    ```bash
    solana-tokens burn <MINT_ADDRESS> <SOURCE_ADDRESS> <AMOUNT> --authority <AUTHORITY_ADDRESS>
    ```

4. Change the paused state:

    ```bash
    solana-tokens change-paused-state --authority <AUTHORITY_ADDRESS> --state <STATE_ADDRESS> --paused <true/false>
    ```

5. Collect transaction fees:

    ```bash
    solana-tokens collect-fees --treasury <TREASURY_ADDRESS> --state <STATE_ADDRESS>
    ```

## Contributing

Bark is a community-focused project, and contributions are welcome! Please follow our [contribution guidelines](CONTRIBUTING.md).

## License

MIT License - see the [LICENSE](LICENSE) file for details.
