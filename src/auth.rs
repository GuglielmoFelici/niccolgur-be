use jsonwebtoken::{encode, Header, EncodingKey, Validation, decode, DecodingKey, Algorithm, TokenData};
use jsonwebtoken::errors::{Error, ErrorKind};
use serde::{Serialize, Deserialize};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket::http::Status;
use crate::views::TokenView;
use crate::responses::{ControllerResult, ControllerError};
use rocket_contrib::json::Json;

// TODO questo modulo è abbastanza da rivedere.

const ENCODE_KEY: &[u8] = b"secret";
// TODO secretare
const DECODE_KEY: &[u8] = b"secret"; // TODO secretare

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn generate_token(id: &String) -> ControllerResult<TokenView> {
    let expiration = 100000000;
    let my_claims =
        Claims { sub: id.to_owned(), exp: expiration }; // TODO token expiration
    let token = match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(ENCODE_KEY)) {
        Ok(tkn) => tkn,
        Err(_) => return Err(ControllerError(Status::InternalServerError))
    };
    Ok(
        Json(
            TokenView {
                payload: token,
                expiration,
            })
    )
}

pub fn verify_token(token: &String) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
    decode::<Claims>(token, &DecodingKey::from_secret(DECODE_KEY), &Validation::new(Algorithm::HS256)) // TODO RS256 ?
    // TODO proper error handling
    // esempio di error handling corretto
    //  match decode::<Claims>(token, &DecodingKey::from_secret(SECRET_KEY), &Validation::new(Algorithm::HS256)) {
    //     Ok(c) => c,
    //     Err(err) => {
    //         false
    //
    //         // match *err.kind() {
    //         // ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
    //         // ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
    //         // _ => panic!("Some other errors"),
    //     },
    // };
}

pub fn auth_user(token: &String, user_id: &String) -> bool {
    match verify_token(token) {
        Ok(data) => data.claims.sub.eq(user_id),
        Err(_) => false
    }
}

pub struct TokenVerifier();

pub struct AuthUser();

impl<'a, 'r> FromRequest<'a, 'r> for TokenVerifier {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        println!("{:?}", &request
            .headers()
            .get("Authorization").next());
        if verify_token(
            &request
                .headers()
                .get("Authorization")
                .find(|value| value.to_owned().split(" ").next().unwrap().eq("Bearer")).ok_or(Err((Status::Unauthorized, ())))? // TODO errore corretto
                .to_owned()
                .split(" ")
                .last().ok_or(Err((Status::Unauthorized, ())))?
                .to_owned(),
        ).is_ok() {
            Outcome::Success(TokenVerifier()) // TODO valore corretto
        } else {
            Outcome::Failure((Status::Unauthorized, ())) // TODO errore corretto
        }
    }
}

// impl<'a, 'r> FromRequest<'a, 'r> for AuthUser {
//     type Error = ();
//
//     fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
//         println!("{:?}", &request.uri().
//             .headers()
//             .get("Authorization").next());
//         let token = request
//             .headers()
//             .get("Authorization")
//             .find(|value| value.to_owned().split(" ").next().unwrap().eq("Bearer")).ok_or(Err((Status::Unauthorized, ())))? // TODO errore corretto
//             .to_owned()
//             .split(" ")
//             .last()
//             .ok_or(Err((Status::Unauthorized, ())))?;
//         if auth_user(&token.to_string(), "Dano").is_ok() { // TODO username
//             Outcome::Success(TokenVerifier()) // TODO valore corretto
//         } else {
//             Outcome::Failure((Status::Unauthorized, ())) // TODO errore corretto
//         }
//     }
// }
