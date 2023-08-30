use crate::coin_collections::CoinCollection;
use crate::coins::Coin;

pub fn distribute(coins: Vec<Coin>, collections: Vec<CoinCollection>) -> DistributionResult {
    collections
        .into_iter()
        .fold(DistributionResult::undistributed(coins), |result, collection| {
            let separated = separate_collector_coins(result.undistributed_coins, &collection);

            DistributionResult {
                updated_collections: add(result.updated_collections, collection.add_coins(separated.coins_for_collection)),
                undistributed_coins: separated.remaining_coins
            }
        })
}

pub struct DistributionResult {
    pub updated_collections: Vec<CoinCollection>,
    pub undistributed_coins: Vec<Coin>
}

impl DistributionResult {
    fn undistributed(coins: Vec<Coin>) -> DistributionResult {
        DistributionResult {
            updated_collections: vec![],
            undistributed_coins: coins,
        }
    }
}

struct SeparationResult {
    coins_for_collection: Vec<Coin>,
    remaining_coins: Vec<Coin>
}

fn separate_collector_coins(coins: Vec<Coin>, collection: &CoinCollection) -> SeparationResult {
    let mut remaining_coins = vec![];
    let mut coins_for_collection = vec![];

    for coin in coins.into_iter() {
        if collection.fits_coin(&coin) {
            coins_for_collection.push(coin)
        } else {
            remaining_coins.push(coin)
        }
    };

    SeparationResult { coins_for_collection, remaining_coins }
}

fn add<T>(vec1: Vec<T>, value: T) -> Vec<T> {
    let mut new_vec = vec![];

    new_vec.extend(vec1);
    new_vec.push(value);

    new_vec
}