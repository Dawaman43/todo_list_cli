# Todo List CLI

A simple command-line todo list manager written in Rust.

## Features

- Add new todos
- View all todos in a table format
- Mark todos as completed
- Delete todos
- Persistent storage in JSON format

## Installation

1. Ensure you have Rust installed. If not, install from [rustup.rs](https://rustup.rs/).
2. Clone or download this repository.
3. Navigate to the project directory and run:

   ```bash
   cargo build --release
   ```

4. The binary will be in `target/release/todo_cli`.

## Usage

Run the application:

```bash
./target/release/todo_cli
```

Follow the interactive menu:

1. **Add todo**: Enter the task text when prompted.
2. **Show Todo lists**: Displays all todos in a table (completed in green, pending in yellow).
3. **Mark completed**: Enter the ID of the todo to mark as completed.
4. **Delete todo**: Enter the ID of the todo to delete.
5. **Quit**: Exit the application.

## Data Storage

Todos are stored in `todo.json` in the current working directory. The file is a JSON array of objects with `id` (integer), `text` (string), and `completed` (boolean) fields.

Example `todo.json`:

```json
[{"id": 1, "text": "Buy groceries", "completed": false}, {"id": 2, "text": "Walk the dog", "completed": true}]
```

If `todo.json` doesn't exist, it will be created automatically when adding the first todo.

## Troubleshooting

- Ensure you have write permissions in the directory where you run the CLI.
- If you encounter JSON parsing errors, check the `todo.json` file for manual edits or corruption.