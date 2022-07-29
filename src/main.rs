use serde::{Deserialize, Serialize};

struct Config {
    input: String,
    output: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let input = args[1].clone();
        let output = args[2].clone();

        Ok(Config { input, output })
    }
}

#[derive(Serialize, Deserialize)]
struct Movies {
    movies: Vec<Movie>,
}

#[derive(Serialize, Deserialize)]
struct Movie {
    id: i32,
    title: String,
    searchable_title: String,
    format: Vec<String>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    let contents = std::fs::read_to_string(&config.input)
        .expect("Something went wrong reading the input file");

    let mut lines = contents.lines();
    let mut movies = Vec::new();
    let mut id = 1;
    while let Some(line) = lines.next() {
        let movie = Movie {
            id,
            title: line.trim().to_string(),
            searchable_title: line.trim().trim_start_matches("The ").to_string(),
            format: vec!["BD".to_string()],
        };

        movies.push(movie);
        id += 1;
    }

    movies.sort_by(|a, b| a.searchable_title.cmp(&b.searchable_title));

    let serialized = serde_json::to_string(&movies).unwrap();

    let output_string = format!("{{\"movies\":{}}}", serialized);

    let mut file = std::fs::File::create(config.output).unwrap();
    std::io::Write::write_all(&mut file, output_string.as_bytes()).unwrap();
}
