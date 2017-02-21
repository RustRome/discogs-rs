# Discogs API client

[![Build Status](https://travis-ci.org/afonso360/discogs-rs.svg?branch=master)](https://travis-ci.org/afonso360/discogs-rs) [![Build status](https://ci.appveyor.com/api/projects/status/7c3too95w1axgp7u?svg=true)](https://ci.appveyor.com/project/afonso360/discogs-rs) [![License: GPL-3.0+](https://img.shields.io/crates/l/discogs.svg)](https://www.gnu.org/licenses/gpl-3.0) [![Crates.io](https://img.shields.io/crates/v/discogs.svg)](https://crates.io/crates/discogs)

The API is slowly becoming more stabilized and probably won't change very much.

# Tasks to do before 1.0.0
 - [ ] Implement all the API
    - [ ] Database
      - [ ] Label
        - [ ] Get Releases
      - [ ] Release
        - [ ] Allow currency in requests
        - [ ] Community ratings
        - [ ] Get Ratings by username
        - [ ] Put Ratings by username
        - [ ] Delete Ratings by username
      - [ ] Master
        - [ ] Get Versions
      - [x] Artist
        - [x] Get Releases
      - [ ] Search
        - [ ] Search Tests
        - [ ] Search real world tests
        - [x] Perform search
        - [x] Implement filters
    - [ ] Users
      - [ ] Profile
      - [ ] Identity
      - [ ] Submissions
      - [ ] Contributions
      - [ ] Collections
      - [ ] Wantlist
      - [ ] Lists
    - [ ] Marketplace
      - [ ] Inventory
      - [ ] Listings
      - [ ] Orders
      - [ ] Fee
      - [ ] Price Suggestions

 - [ ] Handle query error codes that are not 200
 - [ ] Write better tests
 - [ ] Fully document all files
 - [ ] Write some example code


If you would like to contribute please file an issue or pull request, even if its
to tell me that I have no documentation or tests!
