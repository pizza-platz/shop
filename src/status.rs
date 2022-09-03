use anyhow::Result;
use chrono::prelude::*;
use rand::{thread_rng, Rng};
use rust_decimal::Decimal;
use std::sync::Arc;
use tokio::{sync::RwLock, time};

#[derive(Clone)]
pub struct ShopStatus {
    inner: Arc<RwLock<Inner>>,
}

#[derive(Default)]
struct Inner {
    is_open: bool,
    revenue: Decimal,
}

impl ShopStatus {
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    pub async fn start(&self) -> Result<()> {
        let mut interval = time::interval(time::Duration::from_secs(45));
        let mut rng = thread_rng();

        loop {
            interval.tick().await;
            let mut inner = self.inner.write().await;
            let now = Utc::now();
            if now.hour() >= 9 && now.hour() <= 22 {
                if !inner.is_open {
                    inner.is_open = true;
                    inner.revenue = Decimal::ZERO;
                }
                if rng.gen::<f64>() < 0.05 {
                    let price: Decimal = rng.gen();
                    let price = price * Decimal::ONE_HUNDRED;
                    let price = price.round_dp(2);
                    inner.revenue += price;
                }
            } else {
                inner.is_open = false;
            }
        }
    }

    pub async fn is_open(&self) -> bool {
        self.inner.read().await.is_open
    }

    pub async fn revenue(&self) -> Decimal {
        self.inner.read().await.revenue
    }
}
