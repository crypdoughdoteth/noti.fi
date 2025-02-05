use super::types::ChainFeedId;
use dashmap::DashMap;
use std::sync::LazyLock;

pub static LATEST_PRICES: LazyLock<DashMap<ChainFeedId, u128>> = LazyLock::new(|| DashMap::new());

