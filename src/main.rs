use clap::{crate_version, App, Arg, ArgMatches};
use libretranslate::{translate, Language, Translation};
use colored::Colorize;

fn main() {
    // Set CLI application details through clap.
    let matches = App::new("libretrans")
        .version(crate_version!())
        .author("Grant Handy <grantshandy@gmail.com>")
        .about("Translates text from one language to another.")
        .arg(
            Arg::with_name("TEXT")
                .help("What text to translate.")
                .required(true)
                .takes_value(true)
                .index(2)
        )
        .arg(
            Arg::with_name("INPUT>:<OUTPUT")
                .help("Choose what languages to translate from.\n    Possible values: [\"en\", \"ar\", \"zh\", \"fr\", \"de\", \"it\", \"pt\", \"ru\", \"es\", \"ja\"]\n    Tip: You can format languages like \":<OUTPUT>\" to detect the input language.\n")
                .required(true)
                .takes_value(true)
                .index(1)
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .help("run with verbose output")
                .required(false)
                .takes_value(false)
        )
        .get_matches();

    let text = match matches.value_of("TEXT") {
        Some(data) => data,
        None => panic!("No value to TEXT..."),
    };

    if text == "" {
        trans_error("The following required arguments were not provided:", "<TEXT>", text, matches.clone());
    };

    let languages: (Option<Language>, Language) = match matches.value_of("INPUT>:<OUTPUT") {
        Some(data) => get_languages(data, matches.clone()),
        None => panic!("No value for languages..."),
    };

    match translate(languages.0, languages.1, text) {
        Ok(data) => print_data(data, matches.clone()),
        Err(error) => trans_error("Translation error:", &error.to_string(), text, matches.clone()),
    };
}

fn print_data(data: Translation, matches: ArgMatches) {
    if matches.is_present("verbose") {
        println!("{}: \"{}\"\n{}: \"{}\"", data.source.as_pretty().green().bold(), data.input, data.target.as_pretty().green().bold(), data.output);
    } else {
        println!("{}", data.output);
    }
}

fn trans_error(error: &str, details: &str, text: &str, matches: ArgMatches) {
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

fn get_languages(data: &str, matches: ArgMatches) -> (Option<Language>, Language) {
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

            return (None, lang);
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

    return (Some(source), target);
}