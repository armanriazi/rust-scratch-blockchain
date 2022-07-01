use library_utils::calculate::add;

// importing common module.
mod common;

#[cfg(test)]
mod tests {
    use super::*;
    use log::{debug, error, log_enabled, info, Level};
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn it_adds_one() {
        init();

        info!("can log from the test too");
        assert_eq!(3, add_one(2));
    }

    #[test]
    fn it_handles_negative_numbers() {
        init();

        info!("logging from another test");
        assert_eq!(-7, add_one(-8));
    }
    
   #[test]
    fn test_add() {
        // using common code.
        common::setup();
        assert_eq!(add(3, 2), 5);
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    #[should_panic]
    fn it_works() {
        assert!(false);
    }
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn it_works2() {
        assert_eq!("Hello", "world");
    }
}
