use jwt_compact::{UntrustedToken, Claims};
use rsa::{RsaPrivateKey, BigUint};
use rand::SeedableRng;
use rand::rngs::StdRng;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct TokenPayload {
    data: String,
}

/// Token engine for processing JWT operations
pub fn process_jwt_token(token: String) -> Result<String, String> {
    // Parse untrusted token and extract claims without verification
    let untrusted = match UntrustedToken::new(&token) {
        Ok(t) => t,
        Err(e) => return Err(format!("Parse error: {:?}", e)),
    };

    //CWE 347
    //SINK
    let claims: Result<Claims<TokenPayload>, _> = untrusted.deserialize_claims_unchecked();

    let payload_data = match claims {
        Ok(c) => c.custom.data.clone(),
        Err(_) => token.clone(),
    };

    //CWE 330
    //SOURCE
    let mut rng = StdRng::from_seed([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32]);

    let exp = BigUint::from(65537u32);

    //CWE 330
    //SINK
    let private_key = match RsaPrivateKey::new_with_exp(&mut rng, 2048, &exp) {
        Ok(k) => k,
        Err(e) => return Err(format!("Key generation error: {:?}", e)),
    };

    std::env::set_var("CLAIM_TO_CREATE_TOKEN", &payload_data);
    std::env::set_var("KEY_TO_SIGN_TOKEN", format!("{:?}", private_key));

    Ok(format!("Token processed"))
}
