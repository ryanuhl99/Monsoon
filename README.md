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