use std::sync::Arc;

use clap::Clap;
use nlprule::tokenizer::{chunk::Chunker, Tokenizer};
use nlprule::tokenizer::{tag::Tagger, TokenizerOptions};

#[derive(Clap)]
#[clap(
    version = "1.0",
    author = "Benjamin Minixhofer <bminixhofer@gmail.com>"
)]
struct Opts {
    #[clap(long)]
    ids: Option<String>,
    #[clap(long)]
    ignore_ids: Option<String>,
    #[clap(long)]
    stop_at_error: bool,
}

fn main() {
    env_logger::init();
    let opts = Opts::parse();

    let tagger = Tagger::from_dumps(
        &[
            &format!(
                "data/dumps/{}/output.dump",
                std::env::var("RULE_LANG").unwrap()
            ),
            &format!(
                "data/dumps/{}/added.txt",
                std::env::var("RULE_LANG").unwrap()
            ),
        ],
        &[&format!(
            "data/dumps/{}/removed.txt",
            std::env::var("RULE_LANG").unwrap()
        )],
    )
    .unwrap();

    let tokenizer = Tokenizer::from_xml(
        format!(
            "data/disambiguation.{}.canonic.xml",
            std::env::var("RULE_LANG").unwrap()
        ),
        Arc::new(tagger),
        if std::env::var("RULE_LANG").unwrap() == "en" {
            Some(Chunker::new())
        } else {
            None
        },
        TokenizerOptions {
            allow_errors: true,
            ids: opts
                .ids
                .map(|x| x.split(' ').map(|x| x.to_string()).collect())
                .unwrap_or_else(Vec::new),
            ignore_ids: opts
                .ignore_ids
                .map(|x| x.split(' ').map(|x| x.to_string()).collect())
                .unwrap_or_else(Vec::new),
        },
    )
    .unwrap();
    let rules = tokenizer.rules();

    println!("Last ID: {}", rules[rules.len() - 1].id);
    println!("Runnable rules: {}", rules.len());

    let mut passes = 0;

    for rule in rules {
        if rule.test(&tokenizer) {
            passes += 1;
        } else if opts.stop_at_error {
            break;
        }
    }

    println!("Rules passing tests: {}", passes);
}
