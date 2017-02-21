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

extern crate discogs;

use discogs::Discogs;

fn build_discogs() -> Discogs {
    let mut d = Discogs::new(env!("DISCOGS_USER_AGENT"));

    d.key(env!("DISCOGS_CLIENT_KEY"))
     .secret(env!("DISCOGS_CLIENT_KEY"));

    d
}

#[test]
fn test_request_artist() {
    let mut client = build_discogs();

    let artist_res = client.artist(555).get();

    match artist_res {
        Ok(artist_ok) => {
            println!("{:?}", artist_ok);
        },
        Err(artist_err) => {
            println!("{:?}", artist_err);
            panic!();
        }
    }
}
