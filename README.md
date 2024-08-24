# Custom Shell

Welcome to the **Custom Shell**, a simple, customizable shell environment built with Rust. This shell provides various features such as command execution, alias management, job control, and script execution, all with a user-friendly interface and color-coded output.

## Features

- **Command Execution**: Supports standard command execution with chaining (`&&`, `||`), piping (`|`), and background execution (`&`).
- **Alias Management**: Create and remove command aliases with `alias` and `unalias`.
- **Job Control**: Manage background jobs with `jobs`, `fg`, and `bg`.
- **Script Execution**: Run shell scripts using the `source` command.
- **Environment Variable Management**: Manage environment variables with `set`, `unset`, and `env`.
- **Directory Navigation**: Navigate directories using `cd` and `pwd`.
- **Themed Output**: Color-coded output for a visually appealing user experience.

## Installation

To install and run the Custom Shell, you need to have Rust installed on your machine. If you don't have Rust installed, follow the [official installation guide](https://www.rust-lang.org/tools/install).

1. Clone the repository:

   ```bash
   git clone https://github.com/papaaannn/custom-shell.git
   cd custom-shell
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Run the shell:
   ```bash
   cargo run
   ```

## Usage

### Basic Commands

- **Run a Command**: Simply type a command and press Enter.

  ```bash
  echo "Hello, World!"
  ```

- **Set Environment Variables**:

  ```bash
  set MY_VAR "some value"
  ```

- **Navigate Directories**:
  ```bash
  cd /path/to/directory
  pwd
  ```

### Alias Management

- **Add an Alias**:

  ```bash
  alias ll "ls -la"
  ```

- **Remove an Alias**:
  ```bash
  unalias ll
  ```

### Job Control

- **Run a Command in the Background**:

  ```bash
  some_long_running_command &
  ```

- **List Background Jobs**:

  ```bash
  jobs
  ```

- **Bring a Job to the Foreground**:
  ```bash
  fg 1
  ```

### Script Execution

- **Execute a Script**:
  ```bash
  source script.sh
  ```

### Running Executable Files

- **Execute a Binary**:
  ```bash
  ./myprogram arg1 arg2
  ```

## Customization

You can customize the look and feel of the shell by modifying the `Theme` module in the `theme.rs` file. Change the colors and styles to suit your preferences.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
