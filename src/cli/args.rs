use clap::{Parser, Subcommand, ValueEnum};

use crate::config::template_spec::{TemplateSpec, TemplateConfig};

// ex: monsoon generate api-service "Datasource" "./Code/Projects" --transport http --db sqlite --auth jwt --repo --migration

#[derive(Parser, Debug)]
#[command(name = "monsoon", version = "1.0", about = "Backend scaffolding CLI tool")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Generate {
        archetype: Archetype,

        name: String,

        path: String,

        #[arg(long, value_enum)]
        transport: Option<Transport>,

        #[arg(long, value_enum)]
        db: Option<Db>,

        #[arg(long, value_enum)]
        auth: Option<Auth>,

        #[arg(long, default_value_t = false)]
        repo: bool,

        #[arg(long, default_value_t = false)]
        migration: bool,
    },
}

pub trait ConfigMapper {
    fn map_config(&self) -> &'static str;
    fn get_config<'a>(&self, spec: TemplateSpec) -> Result<&'a TemplateConfig, Error>;
}

#[derive(Clone, Debug, ValueEnum)]
enum Archetype {
    ApiService,
    WorkerService,
    SchedulerService,
}

impl ConfigMapper for Archetype {
    fn map_config(&self) -> &'static str {
        match self {
            Archetype::ApiService => "api_service",
            Archetype::WorkerService => "worker_service",
            Archetype::SchedulerService => "scheduler_service",
        }
    }

    fn get_config<'a>(&self, spec: TemplateSpec) -> &'a TemplateConfig {
        match spec.service_config.get(self.map_config()) {
            Some(value) => value,
            None => 
        }
    }
}

#[derive(Clone, Debug, ValueEnum)]
enum Transport {
    Http,
    Grpc,
}

impl ConfigMapper for Transport {
    fn map_config(&self) -> &'static str {
        match self {
            Transport::Http => "http_handler",
            Transport::Grpc => "grpc_handler",
        }
    }
}

#[derive(Clone, Debug, ValueEnum)]
enum Db {
    Postgres,
    Mysql,
    Sqlite,
    None,
}

impl ConfigMapper for Db {
    fn map_config(&self) -> &'static str {
        match self {
            Db::Postgres => "db_postgres",
            Db::Mysql => "db_mysql",
            Db::Sqlite => "db_sqlite",
            Db::None => "db_none",
        }
    }
}

#[derive(Clone, Debug, ValueEnum)]
enum Auth {
    Jwt,
    Oauth,
    Session,
}

impl ConfigMapper for Auth {
    fn map_config(&self) -> &'static str {
        match self {
            Auth::Jwt => "auth_jwt",
            Auth::Oauth => "auth_oauth",
            Auth::Session => "auth_session",
        }
    }
}