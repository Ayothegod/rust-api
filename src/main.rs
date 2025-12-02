// pub mod todos;
// pub mod books_api;
// pub mod error;

// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// use crate::books_api::books;

// #[tokio::main]
// async fn main() {
//     tracing_subscriber::registry()
//         .with(
//             tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
// format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
//             }),
//         )
//         .with(tracing_subscriber::fmt::layer())
//         .init();

//     // books::check();

//     // Compose the routes
//     let app = todos::create_router();

//     let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
//         .await
//         .unwrap();
//     tracing::debug!("listening on {}", listener.local_addr().unwrap());
//     axum::serve(listener, app).await.unwrap();
// }

fn main() {
    let input = "whatsup";

    fn reverse(input: &str) {
        // input.
    }
}

// NOTE: connect to dev network
// solana config set --url https://api.devnet.solana.com
// solana config get

// solana-keygen new --outfile ~/my-solana-wallet/my-keypair.json
// solana-keygen verify 2Y1r73KU6w7SfbCEMPn1aiqpvjobozrNajz9Z1H6Ma8D ~/my-solana-wallet/my-keypair.json

// PENDING: ~/my-solana-wallet/my-keypair.json -> 2Y1r73KU6w7SfbCEMPn1aiqpvjobozrNajz9Z1H6Ma8D
// PENDING: ~/my-solana-wallet/account1.json -> BrV8o2Rt3rSibxnytxBrt4TfcCEAc5SjgYVCgf6iKNzc Phrase(account sleep skull fetch material plug slender august media episode fine concert)

// solana airdrop 1 2Y1r73KU6w7SfbCEMPn1aiqpvjobozrNajz9Z1H6Ma8D --url https://api.devnet.solana.com
// solana balance 2Y1r73KU6w7SfbCEMPn1aiqpvjobozrNajz9Z1H6Ma8D --url https://api.devnet.solana.com

// NOTE: transfer sol to another account (file-system wallets) >
// solana transfer --from ~/my-solana-wallet/my-keypair.json BrV8o2Rt3rSibxnytxBrt4TfcCEAc5SjgYVCgf6iKNzc 0.2 

// --fee-payer <keypair-path>

// NOTE: check balance
// solana balance <address> --url https://api.devnet.solana.com

// TODO: local validator: solana-test-validator
