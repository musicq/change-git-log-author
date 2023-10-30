use std::process::Command;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// old email
    #[arg(long)]
    old_email: String,

    /// correct name
    #[arg(long)]
    new_name: String,

    /// correct email
    #[arg(long)]
    new_email: String,
}

fn main() {
    let args = Args::parse();

    let filter_script = format!(
        r#"OLD_EMAIL="{}"
    CORRECT_NAME="{}"
    CORRECT_EMAIL="{}"
    if [ "$GIT_COMMITTER_EMAIL" = "$OLD_EMAIL" ]
    then
        export GIT_COMMITTER_NAME="$CORRECT_NAME"
        export GIT_COMMITTER_EMAIL="$CORRECT_EMAIL"
    fi
    if [ "$GIT_AUTHOR_EMAIL" = "$OLD_EMAIL" ]
    then
        export GIT_AUTHOR_NAME="$CORRECT_NAME"
        export GIT_AUTHOR_EMAIL="$CORRECT_EMAIL"
    fi"#,
        args.old_email, args.new_name, args.new_email
    );

    let command = Command::new("git")
        .args([
            "filter-branch",
            "-f",
            "--env-filter",
            &filter_script,
            "--tag-name-filter",
            "cat",
            "--",
            "--branches",
            "--tags",
        ])
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()
        .expect("Failed to execute git filter-branch");

    if command.status.success() {
        println!("Git log rewritten successfully");
    } else {
        println!("Git log rewrite failed");
    }
}
