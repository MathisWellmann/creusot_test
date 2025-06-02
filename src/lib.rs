mod smol_currency;

use creusot_contracts::*;

#[requires(a@ < i64::MAX@)]
#[ensures(result@ == a@ + 1)]
pub fn add_one(a: i64) -> i64 {
    a + 1
}

#[ensures(result == v0 || result == v1)]
#[ensures(result <= v0 && result <= v1)]
pub(crate) fn min(v0: i32, v1: i32) -> i32 {
    if v0 < v1 { v0 } else { v1 }
}

#[ensures(result == v0 || result == v1)]
#[ensures(result >= v0 && result >= v1)]
pub(crate) fn max(v0: i32, v1: i32) -> i32 {
    if v0 > v1 { v0 } else { v1 }
}
