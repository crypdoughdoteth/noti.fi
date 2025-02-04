pub struct Loan {
    pub asset: Asset,
    pub amount: u128,
}

pub struct Deposit {
    pub asset: Asset,
    pub amount: u128,
}

pub enum Asset {
    Sui,
    WUSDC,
}

pub struct Position {
    pub loan: Loan,
    pub deposit: Deposit,
}

impl Position {
    // pub fn weighted_borrow(&self) {
    //     let wb = WeightedBorrow {
    //         position: self.loan.asset,
    //         price: self.loan.price,
    //         borrow_weight: self.loan.asset.borrow_weight(),
    //     };
    // }
}

pub struct BasisPoints(u32);
pub struct OpenLtv(BasisPoints);
pub struct CloseLtv(BasisPoints);
pub struct Bw(u8);

/// Position (borrowed) * Price ( Max(EMA, Latest Price) ) * Borrow Weight
pub struct WeightedBorrow {
    asset: Asset,
    amount: u128,
    price: u128,
    borrow_weight: Bw,
}

impl WeightedBorrow {
    pub fn calculate_weighted_borrow(&self) -> u128 {
        self.amount * self.borrow_weight.0 as u128 * self.price
    }
}

/// Position (deposited) * Price ( Max(EMA, Latest Price) ) * Open LTV = Total
pub struct BorrowLimit {
    position: Position,
    price: u128,
    open_ltv: OpenLtv,
}

impl BorrowLimit {
    pub fn get_limit(&self) -> u128 {
        self.position.deposit.amount * self.price * (self.open_ltv.0 .0 as u128 / 10000)
    }
}

/// Position (deposited) * Price * Close LTV
pub struct LiquidationThreshold {
    position: Asset,
    price: u128,
    close_ltv: CloseLtv,
}
