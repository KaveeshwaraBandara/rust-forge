#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────
#  One-time setup: git history → GitHub repo → labels, milestones, issues.
#
#  Usage (from anywhere):
#     bash scripts/bootstrap.sh            # uses your GitHub repo "rust-forge",
#                                          # creating it (public) if it doesn't exist
#     VISIBILITY=private bash scripts/bootstrap.sh
#     REPO_NAME=my-other-name bash scripts/bootstrap.sh
#
#  Needs: git, GitHub CLI (`gh auth login` done once), cargo.
#  Safe to re-run: every step checks whether it already happened.
# ─────────────────────────────────────────────────────────────
set -euo pipefail

REPO_NAME="${REPO_NAME:-rust-forge}"
VISIBILITY="${VISIBILITY:-public}"
DESCRIPTION="Hands-on Rust from zero to production: test-driven lessons, mini-projects and a robotics/edge-AI capstone on Jetson. 🦀"
TOPICS="rust,rust-lang,learning-rust,rust-exercises,cargo-workspace,systems-programming,robotics,edge-ai,jetson"

cd "$(dirname "$0")/.."

say() { printf '\n\033[1;36m▶ %s\033[0m\n' "$*"; }

for cmd in git gh cargo; do
  command -v "$cmd" >/dev/null || { echo "Missing '$cmd'. Install it first."; exit 1; }
done
gh auth status >/dev/null 2>&1 || { echo "Run 'gh auth login' first."; exit 1; }

# ── 1. Local git history ─────────────────────────────────────
# main = scaffold only (CI green). Lesson 01 lives on its own branch,
# so its failing-until-solved tests never turn main red.
if [ ! -d .git ]; then
  say "Creating local git history"
  git config user.email >/dev/null || {
    echo "Set your git identity first: git config --global user.name/user.email"; exit 1; }
  # Each branch gets a Cargo.lock that matches its own crates, so building
  # on one branch never leaves changes that block switching to another.
  stash=".bootstrap-l01"
  restore() { if [ -d "$stash" ]; then mv "$stash" lessons/l01_basics; fi; }
  trap restore EXIT
  mv lessons/l01_basics "$stash"
  cargo generate-lockfile --quiet          # main's lockfile: workspace without l01
  git init -q -b main
  git add -A -- . ":!$stash"
  git commit -q -m "chore: scaffold rust-forge workspace"
  restore; trap - EXIT
  git switch -q -c lesson/01-basics
  cargo generate-lockfile --quiet          # branch's lockfile: now includes l01_basics
  git add -A lessons/l01_basics Cargo.lock
  git commit -q -m "chore(l01): add lesson 01 starter (basics)"
  git switch -q main
else
  say "Git repo already exists, using it as-is"
  git show-ref --verify --quiet refs/heads/main || {
    echo "No 'main' branch found. Rename yours with: git branch -M main"; exit 1; }
fi

# ── 2. GitHub repo: connect to an existing one, or create it ─
if git remote get-url origin >/dev/null 2>&1; then
  say "Remote 'origin' already set: $(git remote get-url origin)"
elif gh repo view "$REPO_NAME" >/dev/null 2>&1; then
  say "Repo ${REPO_NAME} already exists on GitHub, connecting to it"
  if [ "$(gh config get git_protocol 2>/dev/null)" = "ssh" ]; then
    url="$(gh repo view "$REPO_NAME" --json sshUrl --jq .sshUrl)"
  else
    url="$(gh repo view "$REPO_NAME" --json url --jq .url).git"
  fi
  git remote add origin "$url"
else
  say "Creating GitHub repo ${REPO_NAME} (${VISIBILITY})"
  gh repo create "$REPO_NAME" "--${VISIBILITY}" --description "$DESCRIPTION" --source . --remote origin
fi

say "Pushing branches"
push_branch() {
  local b="$1" out
  git show-ref --verify --quiet "refs/heads/$b" || return 0
  if out="$(git push -u origin "$b" 2>&1)"; then echo "  pushed: $b"; return 0; fi
  if [ "${FORCE_PUSH:-0}" = 1 ]; then
    git push -q -u --force origin "$b" && echo "  force-pushed: $b"; return 0
  fi
  echo "$out"
  cat <<EOF

  ✋ GitHub rejected the push of '$b'.
     If the error above says "rejected ... (fetch first)", the repo was probably created on
     the website with "Add a README / .gitignore / license", so it already has a commit.
     If that auto-generated commit is ALL it contains, overwrite it with:
         FORCE_PUSH=1 bash scripts/bootstrap.sh
EOF
  exit 1
}
push_branch main
push_branch lesson/01-basics
gh repo edit --description "$DESCRIPTION" --add-topic "$TOPICS" >/dev/null

# ── 3. Labels ────────────────────────────────────────────────
say "Labels"
gh label create lesson   --color 1D76DB --description "A lesson crate" --force
gh label create project  --color 0E8A16 --description "A mini-project" --force
gh label create capstone --color B60205 --description "Final project" --force
gh label create question --color D876E3 --description "Something to clarify" --force

# ── 4. Milestones (one per phase) ────────────────────────────
say "Milestones"
existing_ms="$(gh api 'repos/{owner}/{repo}/milestones?state=all&per_page=100' --jq '.[].title')"
create_ms() {
  if grep -qxF "$1" <<<"$existing_ms"; then echo "  exists: $1"; return; fi
  gh api --method POST 'repos/{owner}/{repo}/milestones' \
    -f title="$1" -f description="$2" --silent
  echo "  created: $1"
}
create_ms "Phase 1 · Core language" "L01–L05 + P1 detlog"
create_ms "Phase 2 · Abstraction"   "L06–L09 + P2 sensorkit"
create_ms "Phase 3 · Systems Rust"  "L10–L14 + P3 framepipe"
create_ms "Phase 4 · Capstone"      "ROS 2 node / VLM search API / ONNX on Jetson"

# ── 5. Issues (one per lesson/project) ───────────────────────
say "Issues"
existing_issues="$(gh issue list --state all --limit 200 --json title --jq '.[].title')"
create_issue() { # title | milestone | label | crate | branch | book
  local title="$1" ms="$2" label="$3" crate="$4" branch="$5" book="$6"
  if grep -qxF "$title" <<<"$existing_issues"; then echo "  exists: $title"; return; fi
  local body
  body=$(cat <<EOF
**Branch:** \`$branch\`  ·  **Crate:** \`$crate\`  ·  **Book:** $book

- [ ] Read the notes + linked Book sections
- [ ] Compiler drills in \`playground/\` (predict first!)
- [ ] All tests green: \`cargo test -p $crate\`
- [ ] \`cargo clippy --workspace --all-targets -- -D warnings\` clean, \`cargo fmt --all\` applied
- [ ] Concept check answered in \`LEARNING_LOG.md\`
- [ ] PR reviewed and merged
EOF
)
  gh issue create --title "$title" --milestone "$ms" --label "$label" --body "$body" >/dev/null
  echo "  created: $title"
}

P1="Phase 1 · Core language"; P2="Phase 2 · Abstraction"; P3="Phase 3 · Systems Rust"; P4="Phase 4 · Capstone"
B="https://doc.rust-lang.org/book"
create_issue "L01 · Basics: variables, types, functions, control flow" "$P1" lesson   l01_basics      lesson/01-basics      "[ch 3]($B/ch03-00-common-programming-concepts.html)"
create_issue "L02 · Ownership & borrowing"                             "$P1" lesson   l02_ownership   lesson/02-ownership   "[ch 4]($B/ch04-00-understanding-ownership.html)"
create_issue "L03 · Structs, enums & pattern matching"                 "$P1" lesson   l03_types       lesson/03-types       "[ch 5]($B/ch05-00-structs.html), [ch 6]($B/ch06-00-enums.html)"
create_issue "L04 · Collections & text"                                "$P1" lesson   l04_collections lesson/04-collections "[ch 8]($B/ch08-00-common-collections.html)"
create_issue "L05 · Error handling"                                    "$P1" lesson   l05_errors      lesson/05-errors      "[ch 9]($B/ch09-00-error-handling.html)"
create_issue "P1 · detlog: detection-log CLI"                          "$P1" project  detlog          project/p1-detlog     "[ch 12]($B/ch12-00-an-io-project.html)"
create_issue "L06 · Modules, crates & testing"                         "$P2" lesson   l06_modules     lesson/06-modules     "[ch 7]($B/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html), [ch 11]($B/ch11-00-testing.html)"
create_issue "L07 · Generics & traits"                                 "$P2" lesson   l07_traits      lesson/07-traits      "[ch 10]($B/ch10-00-generics.html), [ch 18.2]($B/ch18-02-trait-objects.html)"
create_issue "L08 · Lifetimes"                                         "$P2" lesson   l08_lifetimes   lesson/08-lifetimes   "[ch 10.3]($B/ch10-03-lifetime-syntax.html)"
create_issue "L09 · Closures & iterators"                              "$P2" lesson   l09_iterators   lesson/09-iterators   "[ch 13]($B/ch13-00-functional-features.html)"
create_issue "P2 · sensorkit: sensor-filter library"                   "$P2" project  sensorkit       project/p2-sensorkit  "[ch 10]($B/ch10-00-generics.html), [ch 13]($B/ch13-00-functional-features.html)"
create_issue "L10 · Smart pointers"                                    "$P3" lesson   l10_pointers    lesson/10-pointers    "[ch 15]($B/ch15-00-smart-pointers.html)"
create_issue "L11 · Fearless concurrency"                              "$P3" lesson   l11_concurrency lesson/11-concurrency "[ch 16]($B/ch16-00-concurrency.html)"
create_issue "L12 · Async Rust"                                        "$P3" lesson   l12_async       lesson/12-async       "[ch 17]($B/ch17-00-async-await.html)"
create_issue "L13 · Patterns & advanced features"                      "$P3" lesson   l13_advanced    lesson/13-advanced    "[ch 19]($B/ch19-00-patterns.html), [ch 20]($B/ch20-00-advanced-features.html)"
create_issue "L14 · Unsafe & FFI"                                      "$P3" lesson   l14_ffi         lesson/14-ffi         "[ch 20.1]($B/ch20-01-unsafe-rust.html)"
create_issue "P3 · framepipe: concurrent frame pipeline"               "$P3" project  framepipe       project/p3-framepipe  "[ch 16]($B/ch16-00-concurrency.html), [ch 17]($B/ch17-00-async-await.html)"
create_issue "🎯 Capstone"                                             "$P4" capstone capstone        project/capstone      "see ROADMAP.md"

if git show-ref --verify --quiet refs/heads/lesson/01-basics; then
  git switch -q lesson/01-basics
  say "Done! You're on branch lesson/01-basics. Open lessons/l01_basics/README.md and start."
else
  say "Done! Create your lesson branch with: git switch -c lesson/01-basics"
fi
gh repo view --web >/dev/null 2>&1 || true
