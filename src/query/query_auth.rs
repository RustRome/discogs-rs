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
use base64::{encode, decode};
use hyper::header::*;
use hyper;

#[derive(Clone, PartialEq, Debug)]
pub struct DiscogsKSAuth {
    pub key: Option<String>,
    pub secret: Option<String>
}

impl Scheme for DiscogsKSAuth {
    fn scheme() -> Option<&'static str> {
        Some("Discogs")
    }

    fn fmt_scheme(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut text: String = "".to_string();

        if let Some(key) = self.key.clone() {
            text.push_str(format!("key={}", key).as_str());
        }
        text.push(' ');
        if let Some(secret) = self.secret.clone() {
            text.push_str(format!("secret={}", secret).as_str());
        }
        f.write_str(&encode(text.as_ref()))
    }
}

impl FromStr for DiscogsKSAuth {
    type Err = hyper::error::Error;
    fn from_str(s: &str) -> hyper::Result<DiscogsKSAuth> {
        match decode(s) {
            Ok(decoded) => match String::from_utf8(decoded) {
                Ok(text) => {
                    let mut parts = &mut text.split(' ');
                    let key = match parts.next() {
                        Some(key_part) => {
                            let mut kp = &mut key_part.split('=');
                            kp.next();
                            match kp.next() {
                                Some(t) => Some(t.to_owned()),
                                None => None
                            }
                        }
                        None => return Err(hyper::error::Error::Header)
                    };
                    let secret = match parts.next() {
                        Some(secret_part) => {
                            let mut sp = &mut secret_part.split('=');
                            sp.next();
                            match sp.next() {
                                Some(t) => Some(t.to_owned()),
                                None => None
                            }
                        }
                        None => return Err(hyper::error::Error::Header)
                    };
                    Ok(DiscogsKSAuth {
                        key: key,
                        secret: secret
                    })
                },
                Err(e) => {
                    println!("DiscogsKSAuth::from_utf8 error={:?}", e);
                    Err(hyper::error::Error::Header)
                }
            },
            Err(e) => {
                println!("DiscogsKSAuth::from_base64 error={:?}", e);
                Err(hyper::error::Error::Header)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use hyper::header::{Authorization, Basic, Bearer};
    use hyper::header::{Headers, Header};
    use query::query_auth::DiscogsKSAuth;

    #[test]
    fn test_discogsks_auth() {
        let mut headers = Headers::new();
        headers.set(Authorization(DiscogsKSAuth {
            key: Some("Aladdin".to_owned()),
            secret: Some("sesame".to_owned())
        }));
        assert_eq!(
            headers.to_string(),
            "Authorization: Discogs a2V5PUFsYWRkaW4gc2VjcmV0PXNlc2FtZQ==\r\n".to_owned());
    }

    #[test]
    fn test_discogsks_auth_no_secret() {
        let mut headers = Headers::new();
        headers.set(Authorization(DiscogsKSAuth {
            key: Some("Aladdin".to_owned()),
            secret: None
        }));
        assert_eq!(headers.to_string(), "Authorization: Discogs a2V5PUFsYWRkaW4g\r\n".to_owned());
    }

    #[test]
    fn test_discogsks_auth_no_key() {
        let mut headers = Headers::new();
        headers.set(Authorization(DiscogsKSAuth {
            key: None,
            secret: Some("sesame".to_owned())
        }));
        assert_eq!(headers.to_string(), "Authorization: Discogs IHNlY3JldD1zZXNhbWU=\r\n".to_owned());
    }

    #[test]
    fn test_discogsks_auth_parse() {
        let auth: Authorization<DiscogsKSAuth> = Header::parse_header(
            &[b"Discogs a2V5PUFsYWRkaW4gc2VjcmV0PXNlc2FtZQ==".to_vec()])
            .unwrap();
        assert_eq!(auth.0.key, Some("Aladdin".to_string()));
        assert_eq!(auth.0.secret, Some("sesame".to_string()));
    }

    #[test]
    fn test_discogsks_auth_parse_no_secret() {
        let auth: Authorization<DiscogsKSAuth> = Header::parse_header(
            &[b"Discogs a2V5PUFsYWRkaW4g".to_vec()])
            .unwrap();
        assert_eq!(auth.0.key, Some("Aladdin".to_string()));
        assert_eq!(auth.0.secret, None);
    }

    #[test]
    fn test_discogsks_auth_parse_no_key() {
        let auth: Authorization<DiscogsKSAuth> = Header::parse_header(
            &[b"Discogs IHNlY3JldD1zZXNhbWU=".to_vec()])
            .unwrap();
        assert_eq!(auth.0.key, None);
        assert_eq!(auth.0.secret, Some("sesame".to_string()));
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct DiscogsTokenAuth {
    token: Option<String>,
}
