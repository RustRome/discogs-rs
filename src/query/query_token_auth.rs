// Library that eases the use of discogs API
// Copyright (C) 2016  Afonso Bordado <afonsobordado@az8.co>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::any::Any;
use std::fmt::{self, Display};
use std::str::{FromStr, from_utf8};
use std::ops::{Deref, DerefMut};
use hyper::header::*;
use hyper;

#[derive(Clone, PartialEq, Debug)]
pub struct DiscogsTokenAuth {
    token: String,
}

impl Scheme for DiscogsTokenAuth {
    fn scheme() -> Option<&'static str> {
        Some("Discogs")
    }

    fn fmt_scheme(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut text: String = format!("token={}", self.token.clone());
        f.write_str(text.as_ref())
    }
}

//TODO: Make a neater implementation of this
impl FromStr for DiscogsTokenAuth {
    type Err = hyper::error::Error;
    fn from_str(s: &str) -> hyper::Result<DiscogsTokenAuth> {
        match String::from_utf8(s.into()) {
            Ok(text) => {
                let mut parts = &mut text.split('=');
                parts.next();
                let token = match parts.next() {
                    Some(token_part) => token_part.to_owned(),
                    None => return Err(hyper::error::Error::Header)
                };
                Ok(DiscogsTokenAuth {
                    token: token,
                })
            },
            Err(e) => {
                println!("DiscogsTokenAuth::from_utf8 error={:?}", e);
                Err(hyper::error::Error::Header)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use hyper::header::{Authorization, Basic, Bearer};
    use hyper::header::{Headers, Header};
    use query::DiscogsTokenAuth;

    #[test]
    fn test_discogs_token_auth() {
        let mut headers = Headers::new();
        headers.set(Authorization(DiscogsTokenAuth {
            token: "fghcvkbaskj,dabsd".to_owned(),
        }));
        assert_eq!(
            headers.to_string(),
            "Authorization: Discogs token=fghcvkbaskj,dabsd\r\n".to_owned());
    }

    #[test]
    fn test_discogs_token_auth_parse() {
        let auth: Authorization<DiscogsTokenAuth> = Header::parse_header(
            &[b"Discogs token=fghcvkbaskj,dabsd".to_vec()])
            .unwrap();
        assert_eq!(auth.0.token, "fghcvkbaskj,dabsd".to_string());
    }
}
