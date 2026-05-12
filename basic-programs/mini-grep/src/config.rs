pub struct Config {
    pub file_path: String,
    pub query: String 
} 

impl Config {
    pub fn build (path: String, query: String) -> Self {
        Self {
            file_path: path,
            query: query
        }
    }


    pub fn parse_config(args: &[String]) -> Result<Self, &str>{
        if args.len() != 3 {
            return Err("a program must be provided with 3 arguments, use mini-grep ./path (string searching for) ");
        }

        let path = args[1].clone();
        let query = args[2].clone();
        Ok(Config::build(path, query))
    }
}
