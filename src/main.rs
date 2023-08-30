mod coins;
mod coin_collections;
mod coin_distributor;

use std::ops::Deref;
use coins::*;
use crate::coin_collections::{ CoinCollection };
use crate::coin_distributor::distribute;

fn main() {

    let result = distribute(
        vec![
            Coin::Dime,
            Coin::Nickel,
            Coin::Quarter(UsState::Alaska),
            Coin::Quarter(UsState::Colorado),
        ],
        vec![
            CoinCollection::all_states_collection(vec![
                Coin::Quarter(UsState::Alaska),
                Coin::Quarter(UsState::Alabama),
                Coin::Quarter(UsState::California),
                Coin::Quarter(UsState::Washington),
            ])
        ]
    );

    result.updated_collections.iter().for_each(|col| {
        println!("{:?}", col.get_coins());
    });

    println!("REST: {:?}", result.undistributed_coins);
}

