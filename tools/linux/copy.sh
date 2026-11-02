# The Linux copy of a Windows source tree (OpenSpec change linux-sync-per-tree, design D1): one copy per tree,
# named from the tree's folder in lower case, so that `~/qb64rust` follows the checkout `QB64Rust` and
# `~/qb64rust-full` the worktree `QB64Rust-full`. Sourced by sync.sh and run.sh, which must agree on the rule.
#
#   linux_copy <tree>    prints the copy's path

linux_copy() {
    local name
    name="$(basename "$1" | tr '[:upper:]' '[:lower:]')"
    printf '%s/%s\n' "$HOME" "$name"
}
