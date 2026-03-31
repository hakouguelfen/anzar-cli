use dialoguer::Select;

use crate::{models::strategy::AuthStrategy, theme::theme};

pub fn select_strategy() -> AuthStrategy {
    let strategies: Vec<AuthStrategy> = vec![AuthStrategy::Session, AuthStrategy::Jwt];
    let choice = Select::with_theme(&theme())
        .with_prompt("Select authentication strategy")
        .items(&strategies)
        .default(0)
        .interact()
        .unwrap();

    strategies[choice].clone()
}
