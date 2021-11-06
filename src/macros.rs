#[macro_export]
macro_rules! base_path {
    () => {
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    }; // () => {
       //     base_path!("")
       // };

       // ($input: literal) => {{
       //     let mut path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
       //     if $input != "" {
       //         let starts_with = $input.chars().next().unwrap();
       //         if starts_with != '/' {
       //             path.push_str("/");
       //         }
       //         path.push_str($input);
       //     }
       //     path
       // }};
}

#[macro_export]
macro_rules! config {
    ($input: literal) => {
        std::env::var($input).expect(&format!("Failed to find environment variable: {}", $input))
    };
    // default value provided
    ($input: literal, $default: literal) => {
        std::env::var($input).unwrap_or($default.to_string())
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_base_path() {
        // make sure this doesn't panic.
        // let base_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();

        assert_eq!(base_path!(), std::env::var("PWD").unwrap());
        // assert_eq!(base_path!("bacon"), "/Users/zach/Sites/forge/bacon");
        // assert_eq!(base_path!("/bacon"), "/Users/zach/Sites/forge/bacon");
    }

    #[test]
    fn it_config() {
        // make sure this doesn't panic.
        assert_ne!(config!("CARGO_HOME"), "");
    }

    #[test]
    #[should_panic]
    fn it_config_with_no_default_and_no_env_var_will_panic() {
        let _ = config!("NOT_REAL_ENV_VAR");
    }

    #[test]
    fn it_config_constructor_accepts_2_params() {
        assert_eq!(config!("NOT_REAL_ENV_VAR", "::default::"), "::default::");

        assert_ne!(config!("CARGO_HOME", "::default::"), "::default::")
    }
}
