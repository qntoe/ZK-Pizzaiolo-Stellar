#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

#[contract]
pub struct PizzaioloContract;

const SCORE_KEY: Symbol = symbol_short!("SCORE");

#[contractimpl]
impl PizzaioloContract {
    /// Envía un nuevo puntaje. 
    /// En el futuro, esto validará una prueba ZK de Noir.
    pub fn submit_score(env: Env, user: Address, score: u32) {
        // Por seguridad, el usuario debe autorizar la transacción
        user.require_auth();

        let key = (SCORE_KEY, user.clone());
        let current_best: u32 = env.storage().persistent().get(&key).unwrap_or(0);

        if score > current_best {
            env.storage().persistent().set(&key, &score);
        }
    }

    /// Obtiene el mejor puntaje de un usuario.
    pub fn get_score(env: Env, user: Address) -> u32 {
        let key = (SCORE_KEY, user);
        env.storage().persistent().get(&key).unwrap_or(0)
    }
}

mod test;
