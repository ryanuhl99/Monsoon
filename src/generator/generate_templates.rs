use crate::cli::args::Cli;
use crate::config::template_spec::TemplateSpec;

pub fn build_scaffold(cli: &Cli, config: &TemplateSpec) -> Result<(), Box<dyn std::error::Error>> {
    
}

pub fn get_templates<'a>(cli: &Cli, config: &'a TemplateSpec) -> Result<Vec<&'a TemplateSpec>, Box<dyn std::error::Error>> {

}