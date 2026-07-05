# Krynon

An open-source analytical classification engine from the [Holobion](https://github.com/Holobion) organisation.

## What is Krynon?

Krynon is an analytical classification engine designed to help users find the exact food or object that fits their specific needs. Instead of relying on generic five-star reviews, it evaluates items against a custom set of detailed characteristics.

By breaking down products into their fundamental attributes, Krynon empowers users to make objective, data-driven decisions that align perfectly with their priorities.

The name is derived from the ancient Greek verb *krinein*, meaning "to separate," "to sort," or "to decide"—which is also the etymological root of the word "criterion." It perfectly encapsulates the project's mission: providing a precise, analytical tool to filter out the noise and highlight what truly matters.

## Core Mechanics

- Multi-Criteria Sorting: Classify and compare items based on a wide range of specific features (e.g., build quality, reparability index, price, environmental impact, or taste).
- Custom Weighting: Users can assign different levels of importance to each criterion. If reparability matters more than price, the ranking dynamically adjusts to reflect that preference.
- Tailored Discovery: The engine processes the data to cut through marketing noise, delivering a personalized ranking that highlights the most logical choice.

## Getting Started

### Prerequisites

- [List any prerequisites here, e.g., Rust, Docker, etc.]

### Installation

1. **Clone the repository:**

    ```bash
    git clone [repository_url]
    cd Krynon
    ```

2. **Set up environment variables:**
    Copy the example environment file and fill in your database credentials:

    ```bash
    cp .env.example .env
    ```

    Edit the `.env` file with your PostgreSQL connection details.

3. **Set up the database:**
    This project uses Docker Compose to manage the PostgreSQL database.
    Ensure you have Docker and Docker Compose installed.
    Run the following command to start the PostgreSQL container:

    ```bash
    docker-compose up -d db
    ```

    This will create a PostgreSQL database instance. You may need to run migrations to set up the schema.

4. **Run migrations:**
    Apply the database migrations to set up the schema:

    ```bash
    cargo migrate
    ```

    *(Note: Ensure `cargo-migrate` is installed or adjust command as needed)*

### Running the Project

To run the Dioxus application:

```bash
cargo run
```

This command will build and start the development server.

Using the Dioxus CLI, you can also serve the application for different platforms. The default platform is `web`.

Run the following command in the root of the project to start developing with the default platform:

```bash
dx serve --platform web
```

To run for a different platform, use the `--platform platform` flag. E.g.

```bash
dx serve --platform desktop
```

## Project Structure

- `src/`: Contains the main application code.
  - `db.rs`: Database connection and interaction logic.
  - `model.rs`: Data models.
  - `main.rs`: Application entry point.
  - `components/`: Dioxus UI components.
  - `views/`: Application views/pages.
- `migrations/`: SQL migration files for the database schema.
- `docker-compose.yml`: Docker Compose configuration for the PostgreSQL database.
- `.env.example`: Example environment variables for database configuration.


## Resources & Links

- **Repository**: [https://github.com/Holobion/Krynon](https://github.com/Holobion/Krynon)
- **Organisation**: [https://github.com/Holobion](https://github.com/Holobion)

## License

This project is licensed under the Mozilla Public License 2.0.  
See the [LICENSE](LICENSE) file for details.
