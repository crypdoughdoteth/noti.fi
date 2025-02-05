use async_sse::{decode, Event};
use futures::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::marker::{Send, Sync};
use stream::StreamExt;

use crate::errors::Errors;

pub struct PythClient;

impl PythClient {
    pub fn new() -> Self {
        PythClient
    }

    pub async fn stream_price_feed<T: Future<Output = ()> + Send + Sync + 'static>(
        self,
        feed: ChainFeedId,
        handler: impl Fn(String) -> T + Send + Clone + 'static + Copy,
    ) {
        PriceFeedStream::new(feed).open_stream(handler).await
    }

    pub async fn stream_price_feeds<T: Future<Output = ()> + Send + Sync + 'static>(
        self,
        feed: Vec<ChainFeedId>,
        handler: impl Fn(String) -> T + Send + Clone + Copy + 'static,
    ) {
        MultiFeedStream::new(feed).open_stream(handler).await
    }
}

pub struct MultiFeedStream(Vec<ChainFeedId>);

impl MultiFeedStream {
    const PRICE_FEED_STREAM_BASE_URL: &'static str =
        "https://hermes.pyth.network/v2/updates/price/stream?ids[]=";

    pub fn new(feeds: Vec<ChainFeedId>) -> Self {
        MultiFeedStream(feeds)
    }

    pub async fn open_stream<T: Future<Output = ()> + Send + Sync + 'static>(
        self,
        handler: impl Fn(String) -> T + Send + Clone + Copy + 'static,
    ) {
        let ids = &self.0;
        if ids.is_empty() {
            return;
        }

        let mut url = format!("{}{}", Self::PRICE_FEED_STREAM_BASE_URL, ids[0].feed_id().0);

        if ids.len() > 1 {
            for id in ids[1..].iter() {
                url.push_str(&format!("&ids[]={}", id.feed_id().0));
            }
        }

        let client = Client::builder().build().unwrap();
        let mut stream = decode(
            client
                .get(&url)
                .header("Content-Type", "text/event-stream")
                .send()
                .await
                .unwrap()
                .bytes_stream()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                .into_async_read(),
        );

        while let Ok(x) = stream.next().await.unwrap() {
            tokio::spawn(async move {
                if let Event::Message(msg) = x {
                    if let Some(Ok(l)) = msg.data().lines().next().await {
                        handler(l).await;
                    }
                }
            });
        }
    }
}

pub struct PriceFeedStream(ChainFeedId);

impl PriceFeedStream {
    const PRICE_FEED_STREAM_BASE_URL: &'static str =
        "https://hermes.pyth.network/v2/updates/price/stream?ids[]=";

    pub fn new(feed: ChainFeedId) -> Self {
        Self(feed)
    }

    pub async fn open_stream<T: Future<Output = ()> + Send + Sync + 'static>(
        self,
        handler: impl Fn(String) -> T + Send + 'static + Clone + Copy,
    ) {
        let client = Client::builder().build().unwrap();
        let mut stream = decode(
            client
                .get(format!(
                    "{}{}",
                    Self::PRICE_FEED_STREAM_BASE_URL,
                    self.0.feed_id().0
                ))
                .header("Content-Type", "text/event-stream")
                .send()
                .await
                .unwrap()
                .bytes_stream()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                .into_async_read(),
        );

        while let Ok(x) = stream.next().await.unwrap() {
            tokio::spawn(async move {
                if let Event::Message(msg) = x {
                    if let Some(Ok(l)) = msg.data().lines().next().await {
                        handler(l).await;
                    }
                }
            });
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum ChainFeedId {
    Sui(SuiFeedId),
}

impl ChainFeedId {
    pub fn from_str(string: &str) -> Option<ChainFeedId> {
        match string {
            SuiFeedId::ARB_ID => Some(ChainFeedId::Sui(SuiFeedId::Arb)),
            SuiFeedId::AVAX_ID => Some(ChainFeedId::Sui(SuiFeedId::Avax)),
            SuiFeedId::BTC_ID => Some(ChainFeedId::Sui(SuiFeedId::Btc)),
            SuiFeedId::CETUS_ID => Some(ChainFeedId::Sui(SuiFeedId::Cetus)),
            SuiFeedId::DOGE_ID => Some(ChainFeedId::Sui(SuiFeedId::Doge)),
            SuiFeedId::ETH_ID => Some(ChainFeedId::Sui(SuiFeedId::Eth)),
            SuiFeedId::LTC_ID => Some(ChainFeedId::Sui(SuiFeedId::Ltc)),
            SuiFeedId::OP_ID => Some(ChainFeedId::Sui(SuiFeedId::Op)),
            SuiFeedId::PEPE_ID => Some(ChainFeedId::Sui(SuiFeedId::Pepe)),
            SuiFeedId::SOL_ID => Some(ChainFeedId::Sui(SuiFeedId::Sol)),
            SuiFeedId::SUI_ID => Some(ChainFeedId::Sui(SuiFeedId::Sui)),
            SuiFeedId::USDC_ID => Some(ChainFeedId::Sui(SuiFeedId::Usdc)),
            SuiFeedId::USDT_ID => Some(ChainFeedId::Sui(SuiFeedId::Usdt)),
            SuiFeedId::WLD_ID => Some(ChainFeedId::Sui(SuiFeedId::Wld)),
            SuiFeedId::TIA_ID => Some(ChainFeedId::Sui(SuiFeedId::Tia)),
            SuiFeedId::APT_ID => Some(ChainFeedId::Sui(SuiFeedId::Apt)),
            SuiFeedId::SEI_ID => Some(ChainFeedId::Sui(SuiFeedId::Sei)),
            SuiFeedId::NAVX_ID => Some(ChainFeedId::Sui(SuiFeedId::Navx)),
            SuiFeedId::SCA_ID => Some(ChainFeedId::Sui(SuiFeedId::Sca)),
            SuiFeedId::AFSUI_ID => Some(ChainFeedId::Sui(SuiFeedId::Afsui)),
            SuiFeedId::HASUI_ID => Some(ChainFeedId::Sui(SuiFeedId::Hasui)),
            SuiFeedId::VSUI_ID => Some(ChainFeedId::Sui(SuiFeedId::Vsui)),
            SuiFeedId::FDUSD_ID => Some(ChainFeedId::Sui(SuiFeedId::Fdusd)),
            SuiFeedId::USDY_ID => Some(ChainFeedId::Sui(SuiFeedId::Usdy)),
            SuiFeedId::AUSD_ID => Some(ChainFeedId::Sui(SuiFeedId::Ausd)),
            SuiFeedId::POL_ID => Some(ChainFeedId::Sui(SuiFeedId::Pol)),
            SuiFeedId::FUD_ID => Some(ChainFeedId::Sui(SuiFeedId::Fud)),
            SuiFeedId::BUCK_ID => Some(ChainFeedId::Sui(SuiFeedId::Buck)),
            SuiFeedId::DEEP_ID => Some(ChainFeedId::Sui(SuiFeedId::Deep)),
            _ => None,
        }
    }

    pub const fn feed_id(&self) -> FeedId {
        match self {
            ChainFeedId::Sui(sui_feed_id) => sui_feed_id.as_id(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum SuiFeedId {
    Arb,
    Avax,
    Btc,
    Cetus,
    Doge,
    Eth,
    Ltc,
    Op,
    Pepe,
    Sol,
    Sui,
    Usdc,
    Usdt,
    Wld,
    Tia,
    Apt,
    Sei,
    Navx,
    Sca,
    Afsui,
    Hasui,
    Vsui,
    Fdusd,
    Usdy,
    Ausd,
    Pol,
    Fud,
    Buck,
    Deep,
}

#[derive(Debug, Clone, Copy)]
pub struct FeedId(&'static str);

impl SuiFeedId {
    pub const ARB_ID: &str = "3fa4252848f9f0a1480be62745a4629d9eb1322aebab8a791e344b3b9c1adcf5";
    pub const AVAX_ID: &str = "93da3352f9f1d105fdfe4971cfa80e9dd777bfc5d0f683ebb6e1294b92137bb7";
    pub const BTC_ID: &str = "e62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43";
    pub const CETUS_ID: &str = "e5b274b2611143df055d6e7cd8d93fe1961716bcd4dca1cad87a83bc1e78c1ef";
    pub const DOGE_ID: &str = "dcef50dd0a4cd2dcc17e45df1676dcb336a11a61c69df7a0299b0150c672d25c";
    pub const ETH_ID: &str = "ff61491a931112ddf1bd8147cd1b641375f79f5825126d665480874634fd0ace";
    pub const LTC_ID: &str = "6e3f3fa8253588df9326580180233eb791e03b443a3ba7a1d892e73874e19a54";
    pub const OP_ID: &str = "385f64d993f7b77d8182ed5003d97c60aa3361f3cecfe711544d2d59165e9bdf";
    pub const PEPE_ID: &str = "d69731a2e74ac1ce884fc3890f7ee324b6deb66147055249568869ed700882e4";
    pub const SOL_ID: &str = "ef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
    pub const SUI_ID: &str = "23d7315113f5b1d3ba7a83604c44b94d79f4fd69af77f804fc7f920a6dc65744";
    pub const USDC_ID: &str = "eaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a";
    pub const USDT_ID: &str = "2b89b9dc8fdf9f34709a5b106b472f0f39bb6ca9ce04b0fd7f2e971688e2e53b";
    pub const WLD_ID: &str = "d6835ad1f773de4a378115eb6824bd0c0e42d84d1c84d9750e853fb6b6c7794a";
    pub const TIA_ID: &str = "09f7c1d7dfbb7df2b8fe3d3d87ee94a2259d212da4f30c1f0540d066dfa44723";
    pub const APT_ID: &str = "03ae4db29ed4ae33d323568895aa00337e658e348b37509f5372ae51f0af00d5";
    pub const SEI_ID: &str = "53614f1cb0c031d4af66c04cb9c756234adad0e1cee85303795091499a4084eb";
    pub const NAVX_ID: &str = "88250f854c019ef4f88a5c073d52a18bb1c6ac437033f5932cd017d24917ab46";
    pub const SCA_ID: &str = "7e17f0ac105abe9214deb9944c30264f5986bf292869c6bd8e8da3ccd92d79bc";
    pub const AFSUI_ID: &str = "17cd845b16e874485b2684f8b8d1517d744105dbb904eec30222717f4bc9ee0d";
    pub const HASUI_ID: &str = "6120ffcf96395c70aa77e72dcb900bf9d40dccab228efca59a17b90ce423d5e8";
    pub const VSUI_ID: &str = "57ff7100a282e4af0c91154679c5dae2e5dcacb93fd467ea9cb7e58afdcfde27";
    pub const FDUSD_ID: &str = "ccdc1a08923e2e4f4b1e6ea89de6acbc5fe1948e9706f5604b8cb50bc1ed3979";
    pub const USDY_ID: &str = "e393449f6aff8a4b6d3e1165a7c9ebec103685f3b41e60db4277b5b6d10e7326";
    pub const AUSD_ID: &str = "d9912df360b5b7f21a122f15bdd5e27f62ce5e72bd316c291f7c86620e07fb2a";
    pub const POL_ID: &str = "ffd11c5a1cfd42f80afb2df4d9f264c15f956d68153335374ec10722edd70472";
    pub const FUD_ID: &str = "6a4090703da959247727f2b490eb21aea95c8684ecfac675f432008830890c75";
    pub const BUCK_ID: &str = "fdf28a46570252b25fd31cb257973f865afc5ca2f320439e45d95e0394bc7382";
    pub const DEEP_ID: &str = "29bdd5248234e33bd93d3b81100b5fa32eaa5997843847e2c2cb16d7c6d9f7ff";

    pub fn from_name(s: &str) -> Result<Self, Errors> {
        match s {
            "arb" => Ok(SuiFeedId::Arb),
            "avax" => Ok(SuiFeedId::Avax),
            "btc" => Ok(SuiFeedId::Btc),
            "cetus" => Ok(SuiFeedId::Cetus),
            "doge" => Ok(SuiFeedId::Doge),
            "eth" => Ok(SuiFeedId::Eth),
            "ltc" => Ok(SuiFeedId::Ltc),
            "op" => Ok(SuiFeedId::Op),
            "pepe" => Ok(SuiFeedId::Pepe),
            "sol" => Ok(SuiFeedId::Sol),
            "sui" => Ok(SuiFeedId::Sui),
            "usdc" => Ok(SuiFeedId::Usdc),
            "usdt" => Ok(SuiFeedId::Usdc),
            "wld" => Ok(SuiFeedId::Wld),
            "tia" => Ok(SuiFeedId::Tia),
            "apt" => Ok(SuiFeedId::Apt),
            "sei" => Ok(SuiFeedId::Sei),
            "navx" => Ok(SuiFeedId::Navx),
            "sca" => Ok(SuiFeedId::Sca),
            "afsui" => Ok(SuiFeedId::Afsui),
            "hasui" => Ok(SuiFeedId::Hasui),
            "vsui" => Ok(SuiFeedId::Vsui),
            "fdusd" => Ok(SuiFeedId::Fdusd),
            "usdy" => Ok(SuiFeedId::Usdy),
            "ausd" => Ok(SuiFeedId::Ausd),
            "pol" => Ok(SuiFeedId::Pol),
            "fud" => Ok(SuiFeedId::Fud),
            "buck" => Ok(SuiFeedId::Buck),
            "deep" => Ok(SuiFeedId::Deep),
            _ => Err(Errors::SuiFeedIdParsingError),
        }
    }

    pub const fn as_id(&self) -> FeedId {
        match self {
            SuiFeedId::Arb => FeedId(Self::ARB_ID),
            SuiFeedId::Avax => FeedId(Self::AVAX_ID),
            SuiFeedId::Btc => FeedId(Self::BTC_ID),
            SuiFeedId::Cetus => FeedId(Self::CETUS_ID),
            SuiFeedId::Doge => FeedId(Self::DOGE_ID),
            SuiFeedId::Eth => FeedId(Self::ETH_ID),
            SuiFeedId::Ltc => FeedId(Self::LTC_ID),
            SuiFeedId::Op => FeedId(Self::OP_ID),
            SuiFeedId::Pepe => FeedId(Self::PEPE_ID),
            SuiFeedId::Sol => FeedId(Self::SOL_ID),
            SuiFeedId::Sui => FeedId(Self::SUI_ID),
            SuiFeedId::Usdc => FeedId(Self::USDC_ID),
            SuiFeedId::Usdt => FeedId(Self::USDT_ID),
            SuiFeedId::Wld => FeedId(Self::WLD_ID),
            SuiFeedId::Tia => FeedId(Self::TIA_ID),
            SuiFeedId::Apt => FeedId(Self::APT_ID),
            SuiFeedId::Sei => FeedId(Self::SEI_ID),
            SuiFeedId::Navx => FeedId(Self::NAVX_ID),
            SuiFeedId::Sca => FeedId(Self::SCA_ID),
            SuiFeedId::Afsui => FeedId(Self::AFSUI_ID),
            SuiFeedId::Hasui => FeedId(Self::HASUI_ID),
            SuiFeedId::Vsui => FeedId(Self::VSUI_ID),
            SuiFeedId::Fdusd => FeedId(Self::FDUSD_ID),
            SuiFeedId::Usdy => FeedId(Self::USDY_ID),
            SuiFeedId::Ausd => FeedId(Self::AUSD_ID),
            SuiFeedId::Pol => FeedId(Self::POL_ID),
            SuiFeedId::Fud => FeedId(Self::FUD_ID),
            SuiFeedId::Buck => FeedId(Self::BUCK_ID),
            SuiFeedId::Deep => FeedId(Self::DEEP_ID),
        }
    }

    pub fn from_str(str: &str) -> Option<SuiFeedId> {
        match str {
            Self::ARB_ID => Some(SuiFeedId::Arb),
            Self::AVAX_ID => Some(SuiFeedId::Avax),
            Self::BTC_ID => Some(SuiFeedId::Btc),
            Self::CETUS_ID => Some(SuiFeedId::Cetus),
            Self::DOGE_ID => Some(SuiFeedId::Doge),
            Self::ETH_ID => Some(SuiFeedId::Eth),
            Self::LTC_ID => Some(SuiFeedId::Ltc),
            Self::OP_ID => Some(SuiFeedId::Op),
            Self::PEPE_ID => Some(SuiFeedId::Pepe),
            Self::SOL_ID => Some(SuiFeedId::Sol),
            Self::SUI_ID => Some(SuiFeedId::Sui),
            Self::USDC_ID => Some(SuiFeedId::Usdc),
            Self::USDT_ID => Some(SuiFeedId::Usdt),
            Self::WLD_ID => Some(SuiFeedId::Wld),
            Self::TIA_ID => Some(SuiFeedId::Tia),
            Self::APT_ID => Some(SuiFeedId::Apt),
            Self::SEI_ID => Some(SuiFeedId::Sei),
            Self::NAVX_ID => Some(SuiFeedId::Navx),
            Self::SCA_ID => Some(SuiFeedId::Sca),
            Self::AFSUI_ID => Some(SuiFeedId::Afsui),
            Self::HASUI_ID => Some(SuiFeedId::Hasui),
            Self::VSUI_ID => Some(SuiFeedId::Vsui),
            Self::FDUSD_ID => Some(SuiFeedId::Fdusd),
            Self::USDY_ID => Some(SuiFeedId::Usdy),
            Self::AUSD_ID => Some(SuiFeedId::Ausd),
            Self::POL_ID => Some(SuiFeedId::Pol),
            Self::FUD_ID => Some(SuiFeedId::Fud),
            Self::BUCK_ID => Some(SuiFeedId::Buck),
            Self::DEEP_ID => Some(SuiFeedId::Deep),
            _ => None,
        }
    }

    pub fn from_feed_id(feed_id: FeedId) -> Self {
        Self::from_str(feed_id.0).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PythSSE {
    pub binary: Binary,
    pub parsed: Vec<Parsed>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Binary {
    encoding: String,
    //    data: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parsed {
    pub id: String,
    pub ema_price: Price,
    pub price: Price,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Price {
    pub price: String,
    pub conf: String,
    pub expo: i8,
    pub publish_time: i64,
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let price_value = self.price.parse::<u128>().unwrap();
        let expo_abs = self.expo.unsigned_abs();
        let divisor = 10u128.pow(expo_abs as u32);
        let integer_part = price_value / divisor;
        let fractional_part = price_value % divisor;
        write!(
            f,
            "{}.{:0width$}",
            integer_part,
            fractional_part,
            width = expo_abs as usize
        )
    }
}
