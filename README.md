# fusion
An open-source first League of Legends companion client.

Fusion makes an effort to include features available in plenty of other League related apps, but
without ads and paid features. All features will __always__ be free, and though there aren't any
plans to have some money source for the client, in the case we end up creating one in the future,
it'll at most be cosmetic enhancements for donators.

## Getting Started

Fusion is currently in development, check back later if you want to try out the client! If you're
interested in trying out a development version, see below. Note that this process will be a bit
technical.

## Getting Started w/ Development

Fusion has 2 core components: the "frontend" and "backend." The frontend is written in TypeScript +
Svelte (+ Vite), and the backend is written in Rust. Fusion uses [Tauri](https://tauri.app/) to create the
full app.

### Working w/ the Frontend

Since the frontend is like any other web app, you can work on just the frontend and view changes in
any browser.

To view the frontend, `cd` into the [frontend/](./frontend/) directory and run `npm run dev`. That's
all there is to it!

### Working w/ the Backend

To work with the backend, we need to run both the frontend and Rust backend. Before running the
below command, ensure you do the below steps:
- Ensure you have the [prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)
- Install the Tauri CLI. You can find a variety of ways to install the CLI here:
  https://tauri.app/v1/guides/getting-started/setup/sveltekit (scroll down a bit until you find the
  section named "Install Tauri CLI")

  If you just want to quickly install the CLI and don't care too much about what you use:

  **npm**
  ```shell
  npm install --save-dev @tauri-apps/cli
  ```

  **Cargo** (preferred way, but takes a bit longer to install since it compiles the binary from
  scratch)
  ```shell
  cargo install tauri-cli
  ```

Once you've gone through the above steps, in the root level of this repository, you can run the
following to start up the whole app:

**npm**
```shell
npm run tauri dev
```

**Cargo**
```shell
cargo tauri dev
```

### Development w/ VS Code

If you use VS Code, use the below links to quickly get an environment ready to work with Fusion.

[VS Code](https://code.visualstudio.com/) +
[Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) +
[Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) +
[rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
