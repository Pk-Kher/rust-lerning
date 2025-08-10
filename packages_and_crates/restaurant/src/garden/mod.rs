pub mod garden {
    #[derive(Debug)]
    pub struct GardenDetails {
        name: String,
        what_for: String,
    }
    impl GardenDetails {
        pub fn new(name: String, what_for: String) -> GardenDetails {
            GardenDetails { name, what_for }
        }
    }
}
