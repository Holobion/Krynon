# Krynon

## What is Krynon?

Krynon is an analytical classification engine designed to help users find the exact food or object that fits their specific needs. Instead of relying on generic five-star reviews, it evaluates items against a custom set of detailed characteristics.

By breaking down products into their fundamental attributes, Krynon empowers users to make objective, data-driven decisions that align perfectly with their priorities.

The name is derived from the ancient Greek verb *krinein*, meaning "to separate," "to sort," or "to decide"—which is also the etymological root of the word "criterion." It perfectly encapsulates the project's mission: providing a precise, analytical tool to filter out the noise and highlight what truly matters.

## Core Mechanics

- Multi-Criteria Sorting: Classify and compare items based on a wide range of specific features (e.g., build quality, reparability index, price, environmental impact, or taste).
- Custom Weighting: Users can assign different levels of importance to each criterion. If reparability matters more than price, the ranking dynamically adjusts to reflect that preference.
- Tailored Discovery: The engine processes the data to cut through marketing noise, delivering a personalized ranking that highlights the most logical choice.

## Development

Your new jumpstart project includes basic organization with an organized `assets` folder and a `components` folder.
If you chose to develop with the router feature, you will also have a `views` folder.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # The entrypoint for the app. It also defines the routes for the app.
│  ├─ components/
│  │  ├─ mod.rs # Defines the components module
│  │  ├─ hero.rs # The Hero component for use in the home page
│  │  ├─ echo.rs # The echo component uses server functions to communicate with the server
│  ├─ views/ # The views each route will render in the app.
│  │  ├─ mod.rs # Defines the module for the views route and re-exports the components for each route
│  │  ├─ blog.rs # The component that will render at the /blog/:id route
│  │  ├─ home.rs # The component that will render at the / route
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Automatic Tailwind (Dioxus 0.7+)

As of Dioxus 0.7, there no longer is a need to manually install tailwind. Simply `dx serve` and you're good to go!

Automatic tailwind is supported by checking for a file called `tailwind.css` in your app's manifest directory (next to Cargo.toml). To customize the file, use the dioxus.toml:

```toml
[application]
tailwind_input = "my.css"
tailwind_output = "assets/out.css"
```

### Tailwind Manual Install

To use tailwind plugins or manually customize tailwind, you can can install the Tailwind CLI and use it directly.

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation/tailwind-cli
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --platform web
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

