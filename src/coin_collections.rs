use std::fmt::{Display, Formatter};
use crate::coins::Coin;

pub struct CoinCollection {
    coins: Vec<Coin>,
    fits: fn(&CoinCollection, &Coin) -> bool,
    add: fn(CoinCollection, coins: Vec<Coin>) -> CoinCollection
}

impl CoinCollection {

    pub fn get_coins(&self) -> &Vec<Coin> {
        &self.coins
    }

    pub fn add_coins(self, coins: Vec<Coin>) -> CoinCollection {
        (self.add)(self, coins)
    }
    pub fn fits_coin(&self, coin: &Coin) -> bool {
        (self.fits)(self, coin)
    }
    pub fn all_states_collection(coins: Vec<Coin>) -> CoinCollection {
        CoinCollection {
            coins,
            fits: |slf, coin| {
                matches!(coin, Coin::Quarter(_)) && !slf.coins.contains(coin)
            },
            add: | slf, coins | {
                let mut new_coins: Vec<Coin> = vec![];
                new_coins.extend(slf.coins);
                new_coins.extend(coins);
                CoinCollection {
                    coins: new_coins,
                    fits: slf.fits,
                    add: slf.add
                }
            }
        }
    }

}