use std::io;
use std::error::Error;

use conduit::{Request, Response, Handler, Method};
use conduit_middleware::AroundMiddleware;

use util::RequestProxy;

pub struct Head {
    handler: Option<Box<Handler + Send + Sync>>,
}

impl Head {
    pub fn new() -> Head {
        Head { handler: None }
    }
}

impl AroundMiddleware for Head {
    fn with_handler(&mut self, handler: Box<Handler + Send + Sync>) {
        self.handler = Some(handler);
    }
}

impl Handler for Head {
    fn call(&self, req: &mut Request) -> Result<Response, Box<Error>> {
        if req.method() == Method::Head {
            let mut req = RequestProxy {
                other: req,
                path: None,
                method: Some(Method::Get),
            };
            self.handler.as_ref().unwrap().call(&mut req).map(|r| {
                Response {
                    body: Box::new(io::util::NullReader),
                    ..r
                }
            })
        } else {
            self.handler.as_ref().unwrap().call(req)
        }
    }
}
