use totp_rs::{Algorithm, Secret, TOTP};

pub async fn get_totp(
    secret_string: String,
    issuer: String,
    account_name: String,
) -> TOTP {
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Encoded(secret_string).to_bytes().unwrap(),
        Some(issuer),
        account_name,
    )
    .unwrap();
    totp
}
