use colored::Colorize;
use clap::ArgMatches;
use libretranslate::{Translation, Language};

// Prints a Translation and uses ArgMatches to check if it should print verbosely.
pub fn print_data(data: Translation, matches: ArgMatches) {
    if matches.is_present("verbose") {
        println!("{}: \"{}\"\n{}: \"{}\"", data.source.as_pretty().green().bold(), data.input, data.target.as_pretty().green().bold(), data.output);
    } else {
        println!("{}", data.output);
    }
}

// Prints an imitation clap error that looks identical using colored.
pub fn trans_error(error: &str, details: &str, text: &str, matches: ArgMatches) {
    eprintln!("{} {}", "error:".red().bold(), error);

    if text == "" {
        eprintln!("    {}\n", details.red().bold());
    } else {
        eprintln!("\n{}\n", details);
    };

    println!("\n{} You can format languages like \":<OUTPUT>\" to detect the input language.\n", "Tip:".green().bold());

    let verbose: bool = matches.is_present("verbose");

    if verbose {
        eprint!("USAGE:\n    libretrans <INPUT>:<OUTPUT> <TEXT> --verbose\n\n");
    } else {
        eprint!("USAGE:\n    libretrans [FLAGS] <INPUT>:<OUTPUT> <TEXT>\n\n");
    };

    eprintln!("For more information try {}", "--help".green());
    std::process::exit(1);
}

// Parse our language input from the text and split them up into Languages.
pub fn get_languages(data: &str, matches: ArgMatches) -> (Language, Language) {
    let langs = vec!["en", "ar", "zh", "fr", "de", "it", "pt", "ru", "es", "ja"];
    let possible_langs = "Possible values: [\"en\", \"ar\", \"zh\", \"fr\", \"de\", \"it\", \"pt\", \"ru\", \"es\", \"ja\"]";

    if data.chars().count() != 5 {
        if data.chars().nth(0).unwrap() != ':' || data.chars().count() != 3 {
            trans_error("Malformed language argument", possible_langs, data, matches.clone());
        } else {
            let lang =  data.char_indices()
                .nth(1)
                .and_then(|(i, _)| data.get(i..))
                .unwrap_or("");

            if !langs.contains(&lang) {
                trans_error("Unknown languages", possible_langs, data, matches.clone());
            };

            let lang: Language = match lang.parse::<Language>() {
                Ok(lang) => lang,
                Err(_) => panic!("Unknown Language"),
            };

            return (Language::Detect, lang);
        }
    }

    if data.chars().nth(2).unwrap() != ':' {
        trans_error("Malformed language argument: not separated by an ':'", possible_langs, data, matches.clone());
    };

    let split: Vec<&str> = data.split(':').collect();

    if split.len() != 2 {
        trans_error("Malformed language argument", possible_langs, data, matches.clone());
    };

    for x in &split {
        if !langs.contains(&x) {
            trans_error("Unknown languages", possible_langs, data, matches.clone());
        };
    };

    let source = split.get(0).unwrap().parse::<Language>().unwrap();
    let target = split.get(1).unwrap().parse::<Language>().unwrap();

    return (source, target);
}