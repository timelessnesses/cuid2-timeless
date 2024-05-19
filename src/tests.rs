mod tests {
    
    use crate::cuid_wrapper;


    #[test]
    /// HOW THE FUCK DO I VERIFY IF THIS WORKS
    /// But anyways, this uses [`crate::generator::cuid_wrapper`] for generate a default [`crate::generator::Cuid`] for testing
    fn test_cuid() {
        let mut please_work = cuid_wrapper();
        println!("{}", please_work().unwrap());
    }

    #[test]
    /// AAHHHHHHHHHHHHHHHHHHHHHHHHHH
    fn test_same() {
        let mut please_work = cuid_wrapper();
        let why = please_work().unwrap();
        let god_damn_it = please_work().unwrap();
        assert_ne!(why, god_damn_it)
    }

    #[test]
    fn test_custom_generator() {
        let mut lol = crate::Cuid::new(Box::new(|| {4.0}), Box::new(|_| {
            Box::new(|| {
                5
            })
        }), 2, Box::new(crate::utils::create_fingerprint));
        println!("{}",lol.generate(None).is_err());
    }

    #[test]
    fn test_is_cuid() {
        assert!(crate::is_cuid("f9ovgvsnly7h6khwt4nx08kom".to_string(), None, None));
    }
}
