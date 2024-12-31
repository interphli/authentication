use argon2::{Argon2, Version, Algorithm, ParamsBuilder, Params};
use crate::shared::{Request, Result};
use tokio::sync::OnceCell;
use std::env::var;
use argon2::Error;

const PEPPER: OnceCell<String> = OnceCell::const_new();


pub async fn new_argon2() -> Argon2<'static> {
    let mut t_cost = var("TIME_COST").unwrap_or(Params::DEFAULT_T_COST);
    let mut m_cost = var("MEMORY_COST").unwrap_or(Params::DEFAULT_M_COST);
    let mut p_cost = var("PARALLELISM").unwrap_or(Params::DEFAULT_P_COST);
    let mut output_len = var("OUTPUT_LENGTH").ok();
    let algorithm = match var("ALGORITHM") {
        Ok(value) => {
            match value.to_lowercase().replace(" ", "").as_str() {
                "argon2d" => Algorithm::Argon2d,
                "argon2i" => Algorithm::Argon2i,
                "argon2id" => Algorithm::Argon2id,
                _ => Default::default()
            }
        },
        _ => Algorithm::default()
    };
    let version = match var("") {
        Ok(value) => {
            let cleaned_value = value.to_lowercase().replace("version", "").replace("v", "").replace(" ", "");
            match cleaned_value.as_str() {
                "0x10" | "16" => Version::V0x10,
                "0x13" | "19" => Version::V0x13,
                _ => Default::default(),
            }
        },
        _ => Default::default()
    };
    let params = (0..10).find_map(|_|{
        match Params::new(m_cost, t_cost, p_cost, output_len) {
            Ok(params) => Some(params),
            Err(err) => {
                use Error::*;
                match err {
                    MemoryTooLittle | MemoryTooMuch => {m_cost = Params::DEFAULT_M_COST; None},
                    OutputTooShort | OutputTooLong => {output_len = None; None},
                    ThreadsTooFew | ThreadsTooMany => {p_cost = Params::DEFAULT_P_COST; None},
                    TimeTooSmall => {t_cost = Params::DEFAULT_T_COST; None},
                    _ => None
                }
            }
        }
    }).unwrap_or_default();
    match PEPPER.get_or_try_init(init).await {
        Ok(value) => {
            let secret = value.as_bytes();
            match Argon2::new_with_secret(secret, algorithm, version, params) {
                Ok(argon) => argon,
                _ => Argon2::new(algorithm, version, params)
            }
        },
        _ => {
            Argon2::new(algorithm, version, params)
        }
    }
}

async fn init() -> Result<String> {
    var("PEPPER")
}