use clap::Parser;
use errors::Errors;
use pyth::{
    prices::LATEST_PRICES,
    types::{ChainFeedId, PythClient, PythSSE, SuiFeedId},
};
use sui_sdk::types::base_types::SuiAddress;
use suilend::objects::{Obligation, SuilendAccount};
// use suilend::{
//     objects::SuilendAccount,
//     types::{Asset, Deposit, Loan, Position},
// };
pub mod errors;
pub mod pyth;
pub mod suilend;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    address: SuiAddress,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = Cli::parse().address;

    // This should get us the feeds we need to track
    // after unpacking the info from the move object
    // todo: unpack relevant data from second object's fields
    // Obligation Cap Obj -> Obligation Obj
    let obligations = SuilendAccount::get_suilend_accounts(address).await?;
    let feeds = obligations.iter().fold(Vec::new(), |mut acc, e| {
        let ids = e
            .borrows
            .iter()
            .zip(e.deposits.iter())
            .map(|(borrow, deposit)| {
                let borrow_name = borrow
                    .coin_type
                    .name
                    .split("::")
                    .last()
                    .unwrap()
                    .to_lowercase();
                println!("{}", &borrow_name);

                let deposit_name = deposit
                    .coin_type
                    .name
                    .split("::")
                    .last()
                    .unwrap()
                    .to_lowercase();
                println!("{}", deposit_name);

                let id1 = ChainFeedId::Sui(SuiFeedId::from_name(&borrow_name).unwrap());
                let id2 = ChainFeedId::Sui(SuiFeedId::from_name(&deposit_name).unwrap());
                vec![id1, id2]
            })
            .flatten()
            .collect::<Vec<ChainFeedId>>();

        acc.extend_from_slice(&ids);
        acc
    });

    // replace hard coded feeds
    let client = PythClient::new();
    client
        .stream_price_feeds(feeds, |event| async move {
            let json: PythSSE = serde_json::from_str(&event).unwrap();

            json.parsed.iter().for_each(|e| {
                let chain = ChainFeedId::from_str(&e.id).unwrap();
                let price = e.price.price.parse::<u128>().unwrap();
                println!("{:?}{}", chain, e.price);
                LATEST_PRICES.insert(chain, price);
            });
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
