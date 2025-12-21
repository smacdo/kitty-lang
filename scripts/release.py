#!/usr/bin/env python3
"""
Release management script for kitty-lang.

This script automates the release process by:
1. Verifying no uncommitted changes exist
2. Running formatting check (cargo fmt --all -- --check)
3. Running Clippy linter (cargo clippy --all-targets --all-features -- -D warnings)
4. Running build (cargo build --workspace)
5. Running all tests (cargo test --workspace)
6. Updating version in workspace Cargo.toml
7. Updating dependency versions in workspace members
8. Updating Cargo.lock
9. Committing the version bump
10. Creating a git tag

These checks match the CI workflow to ensure releases won't fail in CI.

Usage:
    python scripts/release.py
    python scripts/release.py --version 0.1.0
    python scripts/release.py --dry-run

NOTE: This script was generated with Claude AI.
"""

import argparse
import re
import subprocess
import sys
from pathlib import Path
from typing import Optional


def run_command(cmd: list[str], check: bool = True, capture: bool = False) -> subprocess.CompletedProcess:
    """Run a shell command and return the result."""
    if capture:
        result = subprocess.run(cmd, capture_output=True, text=True, check=check)
    else:
        result = subprocess.run(cmd, check=check)
    return result


def get_repo_root() -> Path:
    """Get the repository root directory."""
    result = run_command(
        ["git", "rev-parse", "--show-toplevel"],
        capture=True
    )
    return Path(result.stdout.strip())


def check_git_clean() -> bool:
    """Verify there are no uncommitted changes."""
    print("Checking for uncommitted changes...")
    result = run_command(
        ["git", "status", "--porcelain"],
        capture=True
    )
    if result.stdout.strip():
        print("❌ Error: There are uncommitted changes:")
        print(result.stdout)
        return False
    print("✓ Working directory is clean")
    return True


def get_current_version(cargo_toml: Path) -> str:
    """Extract current version from Cargo.toml."""
    content = cargo_toml.read_text()
    match = re.search(r'^\s*version\s*=\s*"([^"]+)"', content, re.MULTILINE)
    if not match:
        raise ValueError("Could not find version in Cargo.toml")
    return match.group(1)


def validate_version(version: str) -> bool:
    """Validate that version follows semver format."""
    pattern = r'^\d+\.\d+\.\d+(-[\w.]+)?(\+[\w.]+)?$'
    return bool(re.match(pattern, version))


def compare_versions(v1: str, v2: str) -> int:
    """Compare two semver versions. Returns -1 if v1 < v2, 0 if equal, 1 if v1 > v2."""
    def parse_version(v: str) -> tuple:
        # Simple comparison, just handle base version
        base = v.split('-')[0].split('+')[0]
        return tuple(map(int, base.split('.')))

    p1 = parse_version(v1)
    p2 = parse_version(v2)

    if p1 < p2:
        return -1
    elif p1 > p2:
        return 1
    return 0


def run_tests() -> bool:
    """Run all workspace tests."""
    print("\nRunning tests...")
    try:
        run_command(["cargo", "test", "--workspace"])
        print("✓ All tests passed")
        return True
    except subprocess.CalledProcessError:
        print("❌ Tests failed")
        return False


def run_build() -> bool:
    """Run a build to verify everything compiles."""
    print("\nVerifying build...")
    try:
        run_command(["cargo", "build", "--workspace"])
        print("✓ Build successful")
        return True
    except subprocess.CalledProcessError:
        print("❌ Build failed")
        return False


def run_fmt_check() -> bool:
    """Check that code is properly formatted."""
    print("\nChecking code formatting...")
    try:
        run_command(["cargo", "fmt", "--all", "--", "--check"])
        print("✓ Code is properly formatted")
        return True
    except subprocess.CalledProcessError:
        print("❌ Code formatting check failed")
        print("Run 'cargo fmt --all' to fix formatting issues")
        return False


def run_clippy() -> bool:
    """Run Clippy linter checks."""
    print("\nRunning Clippy linter...")
    try:
        run_command(["cargo", "clippy", "--all-targets", "--all-features", "--", "-D", "warnings"])
        print("✓ Clippy checks passed")
        return True
    except subprocess.CalledProcessError:
        print("❌ Clippy checks failed")
        return False


def update_version_in_file(file_path: Path, old_version: str, new_version: str, dry_run: bool = False) -> bool:
    """Update version in a Cargo.toml file."""
    content = file_path.read_text()

    # Update workspace version
    updated = re.sub(
        r'(^\s*version\s*=\s*)"' + re.escape(old_version) + '"',
        r'\1"' + new_version + '"',
        content,
        flags=re.MULTILINE
    )

    # Update dependency versions for workspace crates
    updated = re.sub(
        r'(kitty-lang(?:-[\w]+)?\s*=\s*\{[^}]*version\s*=\s*)"' + re.escape(old_version) + '"',
        r'\1"' + new_version + '"',
        updated
    )

    if updated != content:
        if not dry_run:
            file_path.write_text(updated)
        return True
    return False


def update_cargo_lock(dry_run: bool = False) -> bool:
    """Update Cargo.lock to reflect new versions."""
    print("\nUpdating Cargo.lock...")
    if dry_run:
        print("  [DRY RUN] Would run: cargo update -w")
        return True

    try:
        run_command(["cargo", "update", "-w"])
        print("✓ Cargo.lock updated")
        return True
    except subprocess.CalledProcessError:
        print("❌ Failed to update Cargo.lock")
        return False


def commit_version_bump(version: str, dry_run: bool = False) -> bool:
    """Commit the version bump changes."""
    print(f"\nCommitting version bump to {version}...")
    if dry_run:
        print("  [DRY RUN] Would run:")
        print(f"    git add Cargo.toml Cargo.lock crates/*/Cargo.toml")
        print(f'    git commit -m "version bump to {version}"')
        return True

    try:
        # Add all Cargo.toml files and Cargo.lock
        run_command(["git", "add", "Cargo.toml", "Cargo.lock"])
        run_command(["git", "add", "crates/*/Cargo.toml"], check=False)  # May not exist in some repos

        run_command([
            "git", "commit", "-m", f"version bump to {version}\n\n🤖 Generated with release script"
        ])
        print("✓ Version bump committed")
        return True
    except subprocess.CalledProcessError:
        print("❌ Failed to commit changes")
        return False


def create_git_tag(version: str, dry_run: bool = False) -> bool:
    """Create a git tag for the release."""
    tag_name = f"v{version}"
    print(f"\nCreating git tag {tag_name}...")
    if dry_run:
        print(f"  [DRY RUN] Would run: git tag -a {tag_name} -m 'Release {version}'")
        return True

    try:
        run_command([
            "git", "tag", "-a", tag_name,
            "-m", f"Release {version}"
        ])
        print(f"✓ Tag {tag_name} created")
        return True
    except subprocess.CalledProcessError:
        print(f"❌ Failed to create tag {tag_name}")
        return False


def prompt_version(current_version: str) -> Optional[str]:
    """Prompt user for new version number."""
    print(f"\nCurrent version: {current_version}")
    print("\nEnter new version (e.g., 0.1.0, 1.0.0):")
    new_version = input("> ").strip()

    if not new_version:
        print("No version provided, aborting.")
        return None

    return new_version


def main():
    parser = argparse.ArgumentParser(description="Manage kitty-lang releases")
    parser.add_argument(
        "--version",
        help="New version number (e.g., 0.1.0). If not provided, will prompt interactively."
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Preview changes without applying them"
    )
    parser.add_argument(
        "--skip-tests",
        action="store_true",
        help="Skip running tests (not recommended)"
    )
    parser.add_argument(
        "--skip-build",
        action="store_true",
        help="Skip build verification (not recommended)"
    )
    parser.add_argument(
        "--skip-fmt",
        action="store_true",
        help="Skip formatting check (not recommended)"
    )
    parser.add_argument(
        "--skip-clippy",
        action="store_true",
        help="Skip Clippy linting (not recommended)"
    )

    args = parser.parse_args()

    # Get repository root
    try:
        repo_root = get_repo_root()
    except subprocess.CalledProcessError:
        print("❌ Error: Not in a git repository")
        sys.exit(1)

    cargo_toml = repo_root / "Cargo.toml"
    if not cargo_toml.exists():
        print(f"❌ Error: Could not find {cargo_toml}")
        sys.exit(1)

    # Get current version
    try:
        current_version = get_current_version(cargo_toml)
    except ValueError as e:
        print(f"❌ Error: {e}")
        sys.exit(1)

    # Get new version
    if args.version:
        new_version = args.version
    else:
        new_version = prompt_version(current_version)
        if not new_version:
            sys.exit(1)

    # Validate version format
    if not validate_version(new_version):
        print(f"❌ Error: Invalid version format: {new_version}")
        print("Version must follow semver format (e.g., 1.2.3, 1.0.0-beta.1)")
        sys.exit(1)

    # Check version is newer
    if compare_versions(new_version, current_version) <= 0:
        print(f"⚠️  Warning: New version {new_version} is not greater than current version {current_version}")
        if not args.dry_run:
            response = input("Continue anyway? [y/N]: ").strip().lower()
            if response != 'y':
                print("Aborting.")
                sys.exit(1)

    if args.dry_run:
        print(f"\n{'='*60}")
        print("DRY RUN MODE - No changes will be made")
        print(f"{'='*60}")

    print(f"\n📦 Release Plan")
    print(f"{'='*60}")
    print(f"Current version: {current_version}")
    print(f"New version:     {new_version}")
    print(f"{'='*60}\n")

    # Step 1: Check for uncommitted changes
    if not check_git_clean():
        sys.exit(1)

    # Step 2: Run formatting check
    if not args.skip_fmt:
        if not run_fmt_check():
            sys.exit(1)
    else:
        print("\n⚠️  Skipping formatting check (--skip-fmt)")

    # Step 3: Run Clippy
    if not args.skip_clippy:
        if not run_clippy():
            sys.exit(1)
    else:
        print("\n⚠️  Skipping Clippy (--skip-clippy)")

    # Step 4: Run build
    if not args.skip_build:
        if not run_build():
            sys.exit(1)
    else:
        print("\n⚠️  Skipping build (--skip-build)")

    # Step 5: Run tests
    if not args.skip_tests:
        if not run_tests():
            sys.exit(1)
    else:
        print("\n⚠️  Skipping tests (--skip-tests)")

    # Step 6: Update versions in Cargo.toml files
    print(f"\nUpdating versions from {current_version} to {new_version}...")

    files_to_update = [
        cargo_toml,
        repo_root / "crates" / "kitty-lang-ast" / "Cargo.toml",
        repo_root / "crates" / "kitty-lang-interpreter" / "Cargo.toml",
    ]

    for file_path in files_to_update:
        if file_path.exists():
            if update_version_in_file(file_path, current_version, new_version, args.dry_run):
                status = "[DRY RUN] " if args.dry_run else ""
                print(f"  {status}✓ Updated {file_path.relative_to(repo_root)}")

    # Step 7: Update Cargo.lock
    if not update_cargo_lock(args.dry_run):
        sys.exit(1)

    # Step 8: Commit changes
    if not commit_version_bump(new_version, args.dry_run):
        sys.exit(1)

    # Step 9: Create git tag
    if not create_git_tag(new_version, args.dry_run):
        sys.exit(1)

    # Success!
    print(f"\n{'='*60}")
    print("✓ Release preparation complete!")
    print(f"{'='*60}")

    if args.dry_run:
        print("\nThis was a dry run. No changes were made.")
        print(f"Run without --dry-run to perform the release.")
    else:
        print(f"\nVersion {new_version} has been committed and tagged.")
        print(f"\nNext steps:")
        print(f"  1. Review the changes: git show")
        print(f"  2. Push the commit: git push origin master")
        print(f"  3. Push the tag: git push origin v{new_version}")
        print(f"\nPushing the tag will trigger the GitHub Actions release workflow.")


if __name__ == "__main__":
    main()
