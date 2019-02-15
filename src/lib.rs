use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // Its better to return a Result variant Err than just calling a panic! as user gets better error.
            return Err("Not enough arguments.");
        }

        // we don't want to store references in struct because then we would have to deal with lifetimes,
        // and we can't move them as we dont own them, so we have to just clone them even if its a little inefficient
        let query = args[1].clone();
        let filename = args[2].clone();

        // if env var is present, is_err will return false, if its not var will give an error so is_err returns a true
        let mut case_sensitive = true;
        if args.len() >= 4 && args[3] == "CASE_INSENSITIVE" {
            case_sensitive = false;
        }

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(cli_args: Config) -> Result<(), Box<dyn std::error::Error>> {
    let file_contents = fs::read_to_string(cli_args.filename)?;
    //println!("Text of the file: \n{}", file_contents);

    // we have to pass references as the fns take slices and we have strings
    let results = if cli_args.case_sensitive {
        search_case_sensitive(&cli_args.query, &file_contents)
    } else {
        search_case_insensitive(&cli_args.query, &file_contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    let query = query.to_lowercase();
    for line in contents.lines() {
        // we need &query instead of
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// ------------------ TESTING ---------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\nRust:\nsafe, fast, productive.\nHave your cake and eat it too.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents =
            "\nRust:\nsafe, fast, productive.\nHave your cake and eat it too.\nTrust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
