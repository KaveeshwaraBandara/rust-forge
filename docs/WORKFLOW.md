# Workflow

This repo is run like a small professional project: issues track the work, branches isolate it, PRs review it and CI guards `main`.

## GitHub structure

| GitHub feature | Used for |
|---|---|
| **Milestones** | One per phase (`Phase 1 · Core language`, …) |
| **Issues** | One per lesson/project, with a checklist. Close it with the PR. |
| **Labels** | `lesson`, `project`, `capstone`, `question` |
| **Branches** | `lesson/01-basics`, `project/p1-detlog`, … (never commit straight to `main`) |
| **Pull requests** | One per lesson. CI (fmt + clippy + tests) must be green before merging. |
| **Releases** *(optional)* | Tag the end of each phase: `phase-1`, `phase-2`, … |

## Per-lesson flow

```bash
git switch main && git pull
git switch -c lesson/02-ownership       # or `git switch lesson/01-basics` if it already exists

# ...work...
cargo test -p l02_ownership
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all

git add -A
git commit -m "feat(l02): solve e03 borrow_longest"
git push -u origin lesson/02-ownership
gh pr create --fill                      # the PR template appears; write "Closes #2"
```

When CI is green, send me the PR link for review. Fix whatever comes up, then **squash-merge** and delete the branch.

## Commit messages (Conventional Commits)

```text
feat(l01): solve e05 battery_status
fix(l01): handle 0 in count_digits
docs(log): answer L01 concept check
chore: bump toolchain
```

Make small commits. Committing after each exercise is ideal because it lets you see your own progress with `git log --oneline`.

## A good 60–90 min session

| Time | Activity |
|---|---|
| 10–15 min | Read notes + Book section |
| 10 min | Compiler drills in `playground/`: **predict the error before running** |
| 40–60 min | Exercises: one test at a time (`cargo test -p l01_basics e03`) |
| 5 min | Write 3 lines in `LEARNING_LOG.md`: learned / confused / next |

## Getting unstuck (in this order)

1. Read the **whole** compiler error, including the `help:` and `note:` lines. Rust's errors are a tutor.
2. `rustc --explain E0XXX`
3. The std docs page for the type you're using: https://doc.rust-lang.org/std/
4. Ask me, and paste the exact error and what you expected.

## Editor setup (VS Code, also works over Remote-SSH on the Jetson)

- Install the **rust-analyzer** extension (not the deprecated "Rust" extension).
- Put this in `settings.json` to run clippy on every save:

```json
{
  "rust-analyzer.check.command": "clippy",
  "editor.formatOnSave": true
}
```
