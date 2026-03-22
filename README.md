TEMPLATES:

  - handlers/routes
    - options:
      - transport (http, grpc)
      - protocol (rest, soap, etc)

  - logging
    - [idk professional logging patterns; recommendations?]

  - error handling
    - [idk professional error handling patterns; recommendations?]

  - repository
    - options:
      - schema (this could take a json file to build out schema)
      - migration
      - db type (postgres, mysql, etc)
      - with correspondent logic/domain layer? maybe?

  - service
    - options:
      - with dependency injection? maybe?
      - anything else? recommendations?

  - middleware
    - [idk professional middleware patterns; recommendations?]

  - auth
    - options:
      - protocol (jwt, oauth, etc)

  - config

  - scheduled jobs
    - options:
      - library ? or maybe we just pick one idk

  - background worker
  

scaffold-cli/
  Cargo.toml
  scaffold.toml
  templates/
  src/
    main.rs
    cli/
      mod.rs
      args.rs
    config/
      mod.rs
      loader.rs
      model.rs
    domain/
      mod.rs
      project_spec.rs
    generator/
      mod.rs
      api_service.rs
      worker_service.rs
      scheduler_service.rs
    render/
      mod.rs
      template_engine.rs
    fs/
      mod.rs
      writer.rs


COMMAND STRUCTURE v1

 - generate is the main subcommand

<archetype> is the type of thing to scaffold

<name> is the service/project name

[options] are flags that modify generation

ie. scaffold generate <archetype> <name> [options]

1. API-SERVICE

  - ex. scaffold generate api-service orders
  
Options:

--transport <http|grpc>

--database <postgres|none>

--auth <none|jwt>

--repository

--migrations

--register-workspace

ex. scaffold generate api-service orders --transport http --database postgres --auth jwt --repository --migrations --output ./Code

2. Worker service
scaffold generate worker-service email_dispatch

Options:

--database <postgres|none>

--register-workspace

Example:

scaffold generate worker-service email_dispatch --database postgres

3. Scheduler service
scaffold generate scheduler-service nightly_reconcile

Options:

--database <postgres|none>

--register-workspace

Example:

scaffold generate scheduler-service nightly_reconcile --database postgres

nternal parsing shape

Conceptually, this is what your parser is supporting:

scaffold
  generate
    api-service <name> [flags]
    worker-service <name> [flags]
    scheduler-service <name> [flags]

ALL PROJ TYPES REQUIRE A --output flag, which denotes the parent dir path of where they want their proj to be built

templates/
  api/
    core/
      Cargo.toml
      src/main.rs
      src/lib.rs
      src/error.rs
      src/config.rs

    http/
      src/router.rs
      src/handlers/mod.rs

    grpc/
      proto/service.proto
      src/grpc/server.rs

    db/
      src/db/mod.rs

    repository/
      src/repositories/mod.rs
      src/repositories/repository.rs

    migrations/
      migrations/.keep

  shared/
    logging/
      src/telemetry.rs

    middleware/
      src/middleware/mod.rs

    auth-jwt/
      src/auth/mod.rs