use aptos_sdk::rest_client::Client;
use aptos_sdk::rest_client::Transaction;
use tokio::time::{sleep, Duration};
use url::Url;

#[tokio::main]
async fn main() {
    // Create a new client connected to the Aptos Devnet
    let url = Url::parse("https://nd-830-678-480.p2pify.com/daa554334812a6c01e043968a4d56753/v1")
        .expect("Fail to parse Url");
    let client = Client::new(url);

    // Get the latest ledger version to start streaming transactions
    let mut latest_version = u64::from(
        client
            .get_index()
            .await
            .unwrap()
            .into_inner()
            .ledger_version,
    );

    loop {
        // for test
        latest_version = 1657120524;
        // Fetch transactions starting from the latest known version
        let res_transactions = client.get_transactions(Some(latest_version), Some(1)).await;

        let transactions: Vec<Transaction>;
        if res_transactions.as_ref().ok().is_some() {
            transactions = res_transactions.unwrap().into_inner();
        } else {
            println!("Err: {:#?}", res_transactions);
            // Wait for rate limit
            sleep(Duration::from_secs(5)).await;
            continue;
        };

        if !transactions.is_empty() {
            println!("num of transaction: {:#?}", transactions.len());
            for tx in transactions.clone() {
                match tx {
                    Transaction::UserTransaction(_) => {
                        println!("New User Transaction");
                    }
                    Transaction::GenesisTransaction(_) => {
                        println!("Genesis Transaction");
                    }
                    Transaction::BlockMetadataTransaction(_) => {
                        println!("Block Metadata Transaction");
                    }
                    Transaction::StateCheckpointTransaction(_) => {
                        println!("State Checkpoint Transaction");
                    }
                    Transaction::PendingTransaction(_) => {
                        println!("State Pending Transaction");
                    }
                    Transaction::BlockEpilogueTransaction(_) => {
                        println!("State Block Epilogue Transaction");
                    }
                    Transaction::ValidatorTransaction(_) => {
                        println!("State Validator Transaction");
                    }
                }

                let tx_info = tx.transaction_info().unwrap().clone();
                let tx_hash = tx_info.hash;

                println!(
                    "timestamp: {:#?}, tx_hash: {:#?}",
                    tx.timestamp(),
                    tx_hash.to_string()
                );
                println!("tx_info: {:?}", tx_info);
            }

            // Update latest_version to continue from the next transaction
            latest_version = transactions.last().unwrap().version().unwrap() + 1;
        }

        // Wait before polling again
        sleep(Duration::from_secs(15)).await;
    }
}
