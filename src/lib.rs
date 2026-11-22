#![cfg_attr(target_family = "wasm", no_std)]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env};

#[contracttype]
enum Clave {
    CuposDisponibles,
    Reserva(Address),
}

#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    ReservaDuplicada = 1,
    SinCupos = 2,
}

#[contract]
pub struct LanParty;

#[contractimpl]
impl LanParty {
    pub fn __constructor(env: Env, cupo_maximo: u32) {
        env.storage()
            .instance()
            .set(&Clave::CuposDisponibles, &cupo_maximo);
    }

    pub fn reservar(env: Env, participante: Address) -> Result<u32, Error> {
        participante.require_auth();

        let reservas = env.storage().persistent();
        let clave = Clave::Reserva(participante);
        if reservas.has(&clave) {
            return Err(Error::ReservaDuplicada);
        }

        let restantes = Self::cupos_disponibles(env.clone())
            .checked_sub(1)
            .ok_or(Error::SinCupos)?;
        reservas.set(&clave, &true);
        env.storage()
            .instance()
            .set(&Clave::CuposDisponibles, &restantes);
        Ok(restantes)
    }

    pub fn cupos_disponibles(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&Clave::CuposDisponibles)
            .expect("el constructor inicializa los cupos")
    }
}
