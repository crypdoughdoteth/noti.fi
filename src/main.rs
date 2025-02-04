use pyth::types::{ChainFeedId, PythClient, PythSSE, SuiFeedId};
use sui_sdk::types::base_types::SuiAddress;
use suilend::{
    objects::SuilendAccount,
    types::{Asset, Deposit, Loan, Position},
};
use tokio::sync::mpsc::channel;
pub mod errors;
pub mod pyth;
pub mod suilend;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PythClient::new();
    let feed = vec![
        ChainFeedId::Sui(SuiFeedId::Sui),
        ChainFeedId::Sui(SuiFeedId::Deep),
    ];
    client
        .stream_price_feeds(feed, |price| async move {
            let json: PythSSE = serde_json::from_str(&price).unwrap();
            println!("{json:?}")
        })
        .await;

    Ok(())
}

// let client = PythClient::new();
// let feed = ChainFeedId::Sui(SuiFeedId::Sui);
// let (tx, mut rx) = channel(1000);
//
// tokio::spawn(async move {
//     while let Some((price, decimals)) = rx.recv().await {
//         let deposit = Deposit {
//             asset: Asset::Sui,
//             amount: 2200,
//         };
//         let loan = Loan {
//             asset: Asset::WUSDC,
//             amount: 2206,
//         };
//
//         let position = Position { loan, deposit };
//         let position_deposit_value = price * position.deposit.amount;
//         let equity = position_deposit_value - 220600000000;
//         let liq_price = position_deposit_value * 75 / 100;
//         println!(
//             "\nLiq % Away: {}",
//             ((liq_price / position.loan.amount) as f64 / 1000000.0) - 75.0
//         );
//         println!("Liq Threshold: {}", liq_price);
//         println!("Weighted Borrow: {}", position.loan.amount);
//         println!("Deposit Value: {}", position_deposit_value);
//         println!("Equity: {}\n", equity);
//     }
// });
//
// client
//     .stream_price_feed(feed, |x| {
//         let tx = tx.clone();
//         async move {
//             let json: PythSSE = serde_json::from_str(&x).unwrap();
//             for json in json.parsed.iter() {
//                 let spot_price: u128 = json.price.price.parse::<u128>().unwrap();
//                 let decimals = json.price.expo;
//                 let ema_price = json.ema_price.price.parse::<u128>().unwrap();
//                 tx.send((
//                     std::cmp::min(spot_price, ema_price),
//                     decimals.unsigned_abs() as u32,
//                 ))
//                 .await
//                 .unwrap()
//             }
//         }
//     })
//     .await;
//
