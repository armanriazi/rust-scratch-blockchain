mod common;
//mod test_lib;

#[cfg(test)]
mod tests {
    use super::*;    
    //use crate::lib::*;
    //use crate::common::*;    
    use pretty_assertions::assert_eq; 
    use log::{debug, error, log_enabled, info, Level};
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn it_adds_one() {
        init();

        /*
              println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
            
            // Two `Rc`s are equal if their inner values are equal
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));
        */

        info!("can log from the test too");
        //assert_eq!(3, add(2,1));
    }

    #[test]
    fn it_handles_negative_numbers() {
        init();

        info!("logging from another test");
        //assert_eq!(-7, add(-8,11));
    }
    
   #[test]
    fn test_add() {
        // using common code.
        common::setup();
        //assert_eq!(add(3, 2), 5);
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        //assert_eq!(add(0, 0), 0);
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
