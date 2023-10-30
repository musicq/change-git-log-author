# change-git-log-author

`change-git-log-author` is used to change the target author and email of git
log.

## Getting Started

```shell
cargo install change-git-log-author
```

## Usage

Run the command inside your git repository.

```bash
cgla --old-email="your_old@email.com" --new-name="New Author Name" --new-email="new_email@email.com"
```

After log changed, you can use `git push --force` to push your commit to remote
