#!/usr/bin/env bash
set -euo pipefail

# Release automation script for CS2 Counter-Strafe Trainer
# Usage: ./release.sh <version>
# Example: ./release.sh 2.0.0

VERSION="${1:-}"

if [[ -z "$VERSION" ]]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 2.0.0"
    exit 1
fi

# Validate version format (semantic versioning)
if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Error: Version must be in format X.Y.Z (e.g., 2.0.0)"
    exit 1
fi

TAG="v${VERSION}"

echo "🚀 Preparing release ${TAG}"
echo ""

# Check if we're on a clean working tree
if [[ -n $(git status --porcelain) ]]; then
    echo "❌ Error: Working tree is not clean. Commit or stash your changes first."
    git status --short
    exit 1
fi

# Check if tag already exists
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo "❌ Error: Tag $TAG already exists"
    exit 1
fi

# Update version in Cargo.toml
echo "📝 Updating version in Cargo.toml..."
sed -i "s/^version = .*/version = \"$VERSION\"/" Cargo.toml

# Run tests
echo "🧪 Running tests..."
cargo test

# Build locally to verify
echo "🔨 Building locally to verify..."
cargo build --release

# Create git commit
echo "💾 Creating version bump commit..."
git add Cargo.toml Cargo.lock
git commit -m "Bump version to ${VERSION}"

# Create and push tag
echo "🏷️  Creating tag ${TAG}..."
git tag -a "$TAG" -m "Release ${TAG}"

echo ""
echo "✅ Release prepared successfully!"
echo ""
echo "Next steps:"
echo "  1. Review the commit: git show HEAD"
echo "  2. Push to GitHub: git push && git push origin ${TAG}"
echo "  3. GitHub Actions will automatically build and create the release"
echo ""
echo "To undo (if needed):"
echo "  git reset --hard HEAD~1"
echo "  git tag -d ${TAG}"
