//! Keeps `lint-data/` in sync with the workspace sources it snapshots.
//!
//! `build.rs` reads `lint-data/` only when the crate is built outside the
//! workspace (e.g. from a `cargo package` tarball), so drift would otherwise go
//! unnoticed until a packaged build shipped stale lint metadata.
//!
//! Run `make sync-lint-data` (or this test with `UPDATE_LINT_DATA=1`) to
//! refresh the snapshot.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Maps each snapshot path (relative to `lint-data/`) to its workspace source.
fn expected_files(workspace: &Path) -> BTreeMap<PathBuf, PathBuf> {
    let mut files = BTreeMap::new();
    files.insert(
        PathBuf::from("lib.rs"),
        workspace.join("soroban_cost_lints/src/lib.rs"),
    );
    files.insert(
        PathBuf::from("rust-toolchain"),
        workspace.join("rust-toolchain"),
    );
    for doc in md_files(&workspace.join("docs/lints")) {
        files.insert(Path::new("docs").join(doc.file_name().unwrap()), doc);
    }
    files
}

/// Lint pages in `dir`, excluding the generated `README.md` index.
fn md_files(dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut files: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.extension().and_then(|e| e.to_str()) == Some("md")
                && p.file_name().and_then(|n| n.to_str()) != Some("README.md")
        })
        .collect();
    files.sort();
    files
}

#[test]
fn lint_data_snapshot_matches_workspace() {
    let manifest = manifest_dir();
    let workspace = manifest.join("..");
    if !workspace.join("soroban_cost_lints/src/lib.rs").exists() {
        // Built from a packaged crate: the snapshot is the only source.
        return;
    }

    let snapshot = manifest.join("lint-data");
    let expected = expected_files(&workspace);
    let update = std::env::var_os("UPDATE_LINT_DATA").is_some();

    if update {
        let _ = fs::remove_dir_all(&snapshot);
        for (rel, src) in &expected {
            let dest = snapshot.join(rel);
            fs::create_dir_all(dest.parent().unwrap()).unwrap();
            fs::copy(src, &dest).unwrap_or_else(|e| panic!("copy {}: {e}", src.display()));
        }
        return;
    }

    let mut problems = Vec::new();
    for (rel, src) in &expected {
        let want = fs::read(src).unwrap_or_else(|e| panic!("read {}: {e}", src.display()));
        match fs::read(snapshot.join(rel)) {
            Ok(got) if got == want => {}
            Ok(_) => problems.push(format!("out of date: lint-data/{}", rel.display())),
            Err(_) => problems.push(format!("missing: lint-data/{}", rel.display())),
        }
    }
    for doc in md_files(&snapshot.join("docs")) {
        let rel = Path::new("docs").join(doc.file_name().unwrap());
        if !expected.contains_key(&rel) {
            problems.push(format!("stale: lint-data/{}", rel.display()));
        }
    }

    assert!(
        problems.is_empty(),
        "cargo-cost-lint/lint-data/ is out of sync with the workspace:\n  {}\n\
         Run `make sync-lint-data` from the workspace root to refresh it.",
        problems.join("\n  ")
    );
}
