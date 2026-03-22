use clap::{Parser, Subcommand, ValueEnum};

// ex: monsoon generate api-service "Datasource" "./Code/Projects" --transport http --db sqlite --auth jwt --repo --migration

#[derive(Parser, Debug)]
#[command(name = "monsoon", version = "1.0", about = "Backend scaffolding CLI tool")]
struct Cli {
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

#[derive(Clone, Debug, ValueEnum)]
enum Archetype {
    ApiService,
    WorkerService,
    SchedulerService,
}

#[derive(Clone, Debug, ValueEnum)]
enum Transport {
    Http,
    Grpc,
}

#[derive(Clone, Debug, ValueEnum)]
enum Db {
    Postgres,
    Mysql,
    Sqlite,
    None,
}

#[derive(Clone, Debug, ValueEnum)]
enum Auth {
    Jwt,
    Oauth,
    Session,
}