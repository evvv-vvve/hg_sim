pub mod simulation_settings;
pub mod data_trait;
pub mod simulation;
pub mod pronouns;
pub mod district;
pub mod tribute;
pub mod event;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
