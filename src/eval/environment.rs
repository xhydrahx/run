use crate::eval::types::Expr;
use std::{
    f64::consts,
    sync::{Mutex, OnceLock},
};

pub static VARIABLES: OnceLock<Mutex<Vec<Expr>>> = OnceLock::new();

pub fn fetch_variables() -> &'static Mutex<Vec<Expr>> {
    VARIABLES.get_or_init(|| {
        Mutex::new(vec![
            Expr::Var("e".to_string(), Box::new(Expr::Num(consts::E))),
            Expr::Var("pi".to_string(), Box::new(Expr::Num(consts::PI))),
            Expr::Var(
                "phi".to_string(),
                Box::new(Expr::Num((1.0 + 5.0_f64.sqrt()) / 2.0)),
            ),
        ])
    })
}

