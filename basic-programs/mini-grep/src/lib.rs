#[cfg(test)]
mod tests;
mod config;

pub fn search(query: &str, content: String) -> Vec<String>{

    let mut result = Vec::<String>::new();

    for i in content.split("\n"){
        if i.contains(query) {
            result.push(i.to_string());
        }
    }

    result
}