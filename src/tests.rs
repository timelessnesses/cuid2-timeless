mod tests {
    
    use std::{collections::HashSet, sync::Mutex};
    use rayon::prelude::*;
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
    #[should_panic]
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

    #[test]
    fn test_large_ids() {
        let mut thing = cuid_wrapper();
        let a = (0..10_000).into_iter().map(|_| thing().unwrap()).collect::<Vec<String>>();
        let saw = Mutex::new(HashSet::new());
        assert!(a.par_iter().filter(|&id| {
            let mut saw = saw.lock().unwrap();
            saw.insert(id)
        }).count() > 0);
    }
}
