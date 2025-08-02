use thefuck_rs::{Command, Shell};

fn main() {
    // Create a basic command
    let cmd = Command::new("git status --porcelain".to_string(), Shell::Bash);
    println!("Original command: {cmd}");

    // Basic parsing
    println!("Program: {:?}", cmd.program());
    println!("Arguments: {:?}", cmd.arguments());
    println!("Argument count: {}", cmd.argument_count());

    // Command matching
    println!("Starts with 'git': {}", cmd.starts_with("git"));
    println!("Contains 'status': {}", cmd.contains_argument("status"));
    println!(
        "Contains 'porcelain': {}",
        cmd.contains_argument("porcelain")
    );

    // Command validation
    match cmd.validate() {
        Ok(()) => println!("Command is valid"),
        Err(e) => println!("Command validation failed: {e}"),
    }

    // Parse command into structured format
    match cmd.parse() {
        Ok(parsed) => println!("Parsed command: {parsed}"),
        Err(e) => println!("Failed to parse command: {e}"),
    }

    // Command with environment variables
    let mut env = std::collections::HashMap::new();
    env.insert("PATH".to_string(), "/usr/bin:/usr/local/bin".to_string());
    env.insert("HOME".to_string(), "/home/user".to_string());

    let cmd_with_env = cmd.clone().with_env(env);
    println!("Command with PATH: {:?}", cmd_with_env.get_env("PATH"));

    // Command age
    println!("Command age in seconds: {}", cmd.age_seconds());
    println!("Command is recent: {}", cmd.is_recent());

    // Create a command with custom working directory
    let cmd_with_cwd = Command::new("ls -la".to_string(), Shell::Bash)
        .with_cwd("/home/user/documents".to_string());
    println!("Command with custom CWD: {}", cmd_with_cwd.cwd);

    // Test empty command validation
    let empty_cmd = Command::new("   ".to_string(), Shell::Bash);
    println!("Empty command is empty: {}", empty_cmd.is_empty());
    println!("Empty command validation: {:?}", empty_cmd.validate());

    // Test command modification
    let modified_cmd = cmd.with_text("git add .".to_string());
    println!("Modified command: {modified_cmd}");
    println!("Modified program: {:?}", modified_cmd.program());
}
