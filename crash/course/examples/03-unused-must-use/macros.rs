pub mod xxx {
    #[macro_export]
    macro_rules! err_macro {
        () => {
            Err(String::from("error from macro")) as Result<(), String>
        };
    }

    #[macro_export]
    macro_rules! ok_macro {
        () => {
            Ok(()) as Result<(), String>
        };
    }
}

pub fn fred() {}
