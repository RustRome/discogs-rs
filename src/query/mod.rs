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

pub mod query_error;
pub mod query_builder;
pub mod query_token_auth;
pub mod query_ks_auth;

pub use self::query_error::QueryError;
pub use self::query_builder::QueryBuilder;

//TODO: Put these under the module auth
pub use self::query_token_auth::DiscogsTokenAuth;
pub use self::query_ks_auth::DiscogsKSAuth;
