use crate::config::Config;

#[test]
pub fn test_config_build_success(){
    
    let exp_path = "~/".to_string();
    
    let query = "args".to_string();
    
    let config = Config::build(exp_path.clone(), query.clone());

    assert_eq!(&exp_path, &config.file_path, "Expected path does not match with those in config");
    assert_eq!(&query, &config.query, "Expceted query does not match with those in config")
}


#[test]
pub fn test_config_parse_success(){
    let args = 
        ["mini_grep".to_string(), "~/".to_string(), "arg".to_string()];

    let res = Config::parse_config(&args);

    assert!(res.is_ok(), "Result value passed from config was error while ok was expected");
    
    let v = res.unwrap();
    
    assert_eq!(v.file_path, args[1],"The file path does not match");
    
    assert_eq!(v.query, args[2],"The query does not match")
}

#[test]
pub fn test_config_parse_extra_args(){
    let args = 
        ["mini_grep".to_string(), "~/".to_string(), "arg".to_string(), "extra".to_string()];

    let res = Config::parse_config(&args);

    assert!(res.is_err(), "Result value was expected to return error case");
}

#[test]
pub fn test_config_parse_ne_args(){
    let args = ["./mini_grep".to_string()];

    let res = Config::parse_config(&args);

    assert!(res.is_err(), "Result value was expected to return error case");
}