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

mod artist;
mod contributor;
mod label;
mod release;
mod company;
mod image;
mod master;
mod others;

pub use self::artist::*;
pub use self::contributor::*;
pub use self::label::*;
pub use self::release::*;
pub use self::company::*;
pub use self::image::*;
pub use self::master::*;
pub use self::others::*;
