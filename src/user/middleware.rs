use std::error::Error;

use conduit_middleware;
use conduit::Request;
use conduit_cookie::RequestSession;

use db::RequestTransaction;
use super::User;
use util::errors::{CargoResult, Unauthorized, ChainError, std_error};

pub struct Middleware;

impl conduit_middleware::Middleware for Middleware {
    fn before(&self, req: &mut Request) -> Result<(), Box<Error>> {
        let user = match req.session().get("user_id").and_then(|s| s.parse()) {
            Some(id) => {
                match User::find(try!(req.tx().map_err(std_error)), id) {
                    Ok(user) => user,
                    Err(..) => return Ok(()),
                }
            }
            None => {
                let tx = try!(req.tx().map_err(std_error));
                match req.headers().find("Authorization") {
                    Some(headers) => {
                        match User::find_by_api_token(tx, headers[0].as_slice()) {
                            Ok(user) => user,
                            Err(..) => return Ok(())
                        }
                    }
                    None => return Ok(())
                }
            }
        };

        req.mut_extensions().insert(user);
        Ok(())
    }
}

pub trait RequestUser<'a> {
    fn user(self) -> CargoResult<&'a User>;
}

impl<'a> RequestUser<'a> for &'a (Request + 'a) {
    fn user(self) -> CargoResult<&'a User> {
        self.extensions().find::<User>().chain_error(|| Unauthorized)
    }
}
