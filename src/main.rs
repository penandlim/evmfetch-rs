use reqwest::Client;
use clap::{App, Arg};

fn parse_arguments() -> clap::ArgMatches {
    App::new("evmfetch")
        .version("1.0")
        .about("Fetches contract ABI and source code using the Etherscan API")
        .arg(
            Arg::with_name("command")
                .help("The command to execute")
                .possible_values(&["abi", "code"])
                .required(true),
        )
        .arg(
            Arg::with_name("chain_id")
                .help("The chain ID")
                .required(true),
        )
        .arg(
            Arg::with_name("contract_address")
                .help("The contract address")
                .required(true),
        )
        .get_matches()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure the HTTP client
    let client = Client::new();

    // Parse command-line arguments
    let args = parse_arguments();
    let command = args.value_of("command").unwrap();
    let chain_id = args.value_of("chain_id").unwrap();
    let contract_address = args.value_of("contract_address").unwrap();

    // Determine the base api URL based on the chain ID
    let base_url = match chain_id {
        "1" => "https://api.etherscan.io/api",
        "5" => "https://api-goerli.etherscan.io/api",
        "10" => "https://api-optimistic.etherscan.io/api",
        "420" => "https://api-goerli-optimistic.etherscan.io/api",
        "42161" => "https://api.arbiscan.io/api",
        "421613" => "https://api-goerli.arbiscan.io/api",
        _ => {
            println!("Invalid chain ID. Please provide a valid chain ID.");
            std::process::exit(1);
        }
    };

    // Etherscan API parameters
    let api_key = "YOUR_API_KEY";

    match command {
        "abi" => {
            // Fetch contract ABI
            let abi_url = format!(
                "{}?module=contract&action=getabi&address={}&apikey={}",
                base_url, contract_address, api_key
            );
            let abi_response = client.get(&abi_url).send().await?.text().await?;
            let abi: serde_json::Value = serde_json::from_str(&abi_response)?;

            // Extract the ABI from the response
            let abi_str = abi["result"].as_str().unwrap_or("ABI not found.");
            println!("Contract ABI: {}", abi_str);
        }
        "code" => {
            // Fetch contract source code
            let source_code_url = format!(
                "{}?module=contract&action=getsourcecode&address={}&apikey={}",
                base_url, contract_address, api_key
            );
            let source_code_response = client.get(&source_code_url).send().await?.text().await?;
            let source_code: serde_json::Value = serde_json::from_str(&source_code_response)?;

            // Extract the source code from the response
            let source_code_str = source_code["result"][0]["SourceCode"].as_str().unwrap_or("Source code not found.");
            println!("Contract Source Code: {}", source_code_str);
        }
        _ => {
            println!("Invalid command. Please provide either 'abi' or 'code'.");
            std::process::exit(1);
        }
    }

    Ok(())
}