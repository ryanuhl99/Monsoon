mod cli;
mod config;
mod generator;

use clap::Parser;
use cli::args::Cli;
use config::template_spec::TemplateSpec;
use std::fs;
use generator::generate_templates::get_templates;

fn main() {
    let _args = Cli::try_parse()
        .unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
    });
    let config_content = fs::read_to_string("./config/template_config.toml")
        .unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        });
    let config: TemplateSpec = toml::from_str(&config_content)
        .unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        });

    let templates = get_templates(&_args, &config)
        .unwrap_or_else(|e| {
            
        });
}
