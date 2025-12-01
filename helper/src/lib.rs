// How to use:
// in [dependencies] of Cargo.toml of the day** crate:
// helper ={ path = "../../helper"}

pub mod input; // use helper::input:

#[cfg(test)]
mod tests {}
