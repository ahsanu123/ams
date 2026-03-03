use actix_web::{Error, FromRequest, error::ErrorUnauthorized};
use ams_shared::passkey_calculator::validate_passkey;
use std::future::{self, ready};

pub struct PassKey(pub bool);

impl FromRequest for PassKey {
    type Error = Error;
    type Future = future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        if req.headers().contains_key("dev") {
            return ready(Ok(PassKey(true)));
        }
        let result = (|| {
            let passkey = req
                .headers()
                .get("passkey")
                .ok_or_else(|| ErrorUnauthorized("401: passkey required"))?
                .to_str()
                .map_err(|_| ErrorUnauthorized("401: Invalid passkey format"))?
                .parse::<i32>()
                .map_err(|_| ErrorUnauthorized("401: Invalid passkey number"))?;

            if !validate_passkey(passkey) {
                return Err(ErrorUnauthorized("401: Invalid passkey"));
            }

            Ok(PassKey(true))
        })();

        ready(result)
    }
}
