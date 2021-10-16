# Instructions 

* Goldberg Governance requires spl-token-metadata from the metaplex repository as a dependency. 
* See `Cargo.toml` for more details 
* To run the program, some specific steps are needed


1. `git clone https://github.com/metaplex-foundation/metaplex.git`
2. copy `DAO-Governance-Room/Goldberg Govenance` folder into `/metaplex/rust/token-metadata` folder
3. add `"/token-metadata/Goldberg Governance"` to `members` array in `/metaplex/rust/Cargo.toml`
4.  `cd /metaplex/rust/token-metadata/Goldberg Governance`
5.  `cargo run`