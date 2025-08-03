pub mod secrets;
pub mod dangerous;
pub mod misconfig;
pub mod known_patterns;
pub mod permissions;
pub mod bad_practices;

use crate::matchers::Rule;

pub fn load_all_rules() -> Vec<Rule> {
    let mut all = vec![];
    all.extend(secrets::load());
    all.extend(dangerous::load());
    all.extend(misconfig::load());
    all.extend(known_patterns::load());
    all.extend(permissions::load());
    all.extend(bad_practices::load());
    all
}