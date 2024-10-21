## Development Guide

### Adding a New Route

Here’s how you can add routes:

1. **Create a Route**: Add a new route in the `src/routes` folder.
2. **Update `src/startup.rs`**: Import the new route and include it under the `/api/v1` path in the `src/startup.rs` file.
3. **Document with `utoipa`**: To ensure API documentation is generated, annotate the exposed service function with the `#[utoipa]` attribute. Also, make sure the path is registered in `src/routes/api/v1/mod.rs` so it appears in the API documentation.
4. **Handle Migrations (if necessary)**: If the route involves schema or database changes, create a migration. Use `make migrate-create <migration_info>` to generate a new migration file and then modify it as needed. Run the migration using `make migrate-run`.
5. **Write Tests**: It’s good practice to add tests for your route. Create a new test file in the `tests/api` folder, and refer to existing tests for guidance.
