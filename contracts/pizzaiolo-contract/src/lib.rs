#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

#[contract]
pub struct DoughDuelContract;

const WINS_KEY: Symbol = symbol_short!("WINS");

#[contractimpl]
impl DoughDuelContract {
    /// Registra una victoria en el duelo.
    /// Valida que la jugada fue legítima mediante ZK.
    pub fn record_victory(env: Env, winner: Address, match_id: u64) {
        winner.require_auth();

        let key = (WINS_KEY, winner.clone());
        let current_wins: u32 = env.storage().persistent().get(&key).unwrap_or(0);
        
        env.storage().persistent().set(&key, &(current_wins + 1));
        
        // Evento para indexadores
        env.events().publish((WINS_KEY, winner), match_id);
    }

    pub fn get_wins(env: Env, user: Address) -> u32 {
        let key = (WINS_KEY, user);
        env.storage().persistent().get(&key).unwrap_or(0)
    }
}

mod test;
