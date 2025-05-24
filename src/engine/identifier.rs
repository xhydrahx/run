use super::types::Expr;
use std::{
    f64::consts,
    sync::{OnceLock, RwLock},
};

pub static VARIABLES: OnceLock<RwLock<Vec<Expr>>> = OnceLock::new();

pub fn get_variables() -> &'static RwLock<Vec<Expr>> {
    VARIABLES.get_or_init(|| {
        RwLock::new(vec![
            Expr::Variable("e".to_string(), Box::new(Expr::Num(consts::E))),
            Expr::Variable("pi".to_string(), Box::new(Expr::Num(consts::PI))),
            Expr::Variable(
                "phi".to_string(),
                Box::new(Expr::Num((1.0 + 5.0_f64.sqrt()) / 2.0)),
            ),
        ])
    })
}
