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

// solana-keygen new --outfile ~/my-solana-wallet/my-keypair.json
// solana-keygen verify 2Y1r73KU6w7SfbCEMPn1aiqpvjobozrNajz9Z1H6Ma8D ~/my-solana-wallet/my-keypair.json
// pubkey: 2Y1r73KU6w7SfbCEMPn1aiqpvjobozrNajz9Z1H6Ma8D

// NOTE: connect to dev network
// solana config set --url https://api.devnet.solana.com
// solana config get
// solana airdrop 1 <recipient-address> --url https://api.devnet.solana.com NOTE: confirm transaction on dev net block explorer
// solana balance <account-address> --url https://api.devnet.solana.com

// NOTE: transfer sol to another account (file-system wallets) >
// solana transfer --from <keypair-path> <recipient-address> <amount> --fee-payer <keypair-path>

// NOTE: check balance
// solana balance <address> --url https://api.devnet.solana.com

// TODO: send and reccieve funds using the command line
// TODO: fizz buzz in rust: function that
// (loop counting to 301, if count divisible by 3, print fizz, versa, 3.5 print fizzbuzz, at the end print no -f times fizzbuzz occured )

// TODO: local validator: solana-test-validator