use serde::{Deserialize, Serialize};

/// User position information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    #[serde(rename = "proxyWallet")]
    pub proxy_wallet: String,
    pub asset: String,
    #[serde(rename = "conditionId")]
    pub condition_id: String,
    pub size: f64,
    #[serde(rename = "avgPrice")]
    pub avg_price: f64,
    #[serde(rename = "initialValue")]
    pub initial_value: f64,
    #[serde(rename = "currentValue")]
    pub current_value: f64,
    #[serde(rename = "cashPnl")]
    pub cash_pnl: f64,
    #[serde(rename = "percentPnl")]
    pub percent_pnl: f64,
    #[serde(rename = "totalBought")]
    pub total_bought: f64,
    #[serde(rename = "realizedPnl")]
    pub realized_pnl: f64,
    #[serde(rename = "percentRealizedPnl")]
    pub percent_realized_pnl: f64,
    #[serde(rename = "curPrice")]
    pub cur_price: f64,
    pub redeemable: bool,
    pub mergeable: bool,
    pub title: String,
    #[serde(rename = "eventId")]
    pub event_id: String,
    pub outcome: String,
    #[serde(rename = "outcomeIndex")]
    pub outcome_index: u32,
    #[serde(rename = "oppositeOutcome")]
    pub opposite_outcome: String,
    #[serde(rename = "oppositeAsset")]
    pub opposite_asset: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    #[serde(rename = "negativeRisk")]
    pub negative_risk: bool,
}

/// User position value summary
#[derive(Debug, Serialize, Deserialize)]
pub struct PositionValue {
    pub user: String,
    pub value: f64,
}

/// Parameters for querying trades
#[derive(Debug, Clone, Default)]
pub struct TradeParams {
    pub id: Option<String>,
    pub maker_address: Option<String>,
    pub market: Option<String>,
    pub asset_id: Option<String>,
    pub before: Option<u64>,
    pub after: Option<u64>,
}

impl TradeParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn maker_address(mut self, maker_address: impl Into<String>) -> Self {
        self.maker_address = Some(maker_address.into());
        self
    }

    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    pub fn asset_id(mut self, asset_id: impl Into<String>) -> Self {
        self.asset_id = Some(asset_id.into());
        self
    }

    pub fn before(mut self, before: u64) -> Self {
        self.before = Some(before);
        self
    }

    pub fn after(mut self, after: u64) -> Self {
        self.after = Some(after);
        self
    }

    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::with_capacity(6);

        if let Some(ref id) = self.id {
            params.push(("id", id.clone()));
        }

        if let Some(ref asset_id) = self.asset_id {
            params.push(("asset_id", asset_id.clone()));
        }

        if let Some(ref market) = self.market {
            params.push(("market", market.clone()));
        }

        if let Some(before) = self.before {
            params.push(("before", before.to_string()));
        }

        if let Some(after) = self.after {
            params.push(("after", after.to_string()));
        }

        if let Some(ref maker_address) = self.maker_address {
            params.push(("maker_address", maker_address.clone()));
        }

        params
    }
}
