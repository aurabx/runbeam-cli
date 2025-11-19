# Setting Up the Homebrew Tap

This document describes how to set up and maintain the `aurabx/homebrew-tap` repository for distributing `runbeam-cli` via Homebrew.

## Initial Setup

### 1. Create the Homebrew Tap Repository

Create a new repository: `https://github.com/aurabx/homebrew-tap`

Repository structure:
```
homebrew-tap/
├── Formula/
│   └── runbeam.rb          # The Homebrew formula
├── README.md               # Documentation
└── .github/workflows/
    └── tests.yml           # Optional: test the formula
```

### 2. Add the Formula

Copy the template from `runbeam.rb.template` to the tap repository at `Formula/runbeam.rb` and update placeholders with actual SHA256 checksums and version.

### 3. Create README.md

```markdown
# Aurabx Homebrew Tap

Homebrew tap for Aurabx tools.

## Installation

```bash
brew tap aurabx/tap
brew install runbeam
```

## Updating

```bash
brew upgrade runbeam
```

## Uninstalling

```bash
brew uninstall runbeam
brew untap aurabx/tap
```
```

## Release Workflow

### Manual Release Process

When releasing a new version of `runbeam-cli`:

1. **Tag the release** in runbeam-cli repository:
   ```bash
   git tag v0.8.0
   git push origin v0.8.0
   ```

2. **Wait for GitHub Actions** to build binaries and create the release

3. **Get the SHA256 checksums** from the GitHub release page:
   - Download all `.sha256` files
   - Extract the hashes

4. **Update the Homebrew formula** in the tap repository:
   - Edit `Formula/runbeam.rb`
   - Update version number
   - Update SHA256 checksums for each platform
   - Commit and push

   Example SHA256 extraction from release checksums:
   ```bash
   # If checksums.txt contains:
   # abc123def456...  runbeam-x86_64-apple-darwin.tar.gz
   # def789ghi012...  runbeam-aarch64-apple-darwin.tar.gz
   # etc.
   ```

### Automated Release Process (Optional)

Consider setting up a GitHub Actions workflow in the tap repository to automatically update the formula on release. This requires:

1. Adding a workflow that triggers on runbeam-cli releases
2. Using the GitHub API to fetch release info
3. Extracting checksums automatically
4. Creating a PR with updates (or auto-committing with appropriate permissions)

Example workflow trigger:
```yaml
on:
  repository_dispatch:
    types: [runbeam-release]
```

Then from runbeam-cli CI, dispatch an event:
```bash
curl -X POST https://api.github.com/repos/aurabx/homebrew-tap/dispatches \
  -H "Authorization: token $TOKEN" \
  -H "Accept: application/vnd.github.v3+raw" \
  -d '{"event_type":"runbeam-release","client_payload":{"version":"0.8.0","checksums":{...}}}'
```

## Testing the Formula

To test the formula locally before releasing:

```bash
# Create a test formula
brew tap-new aurabx/test-tap
cp Formula/runbeam.rb /usr/local/Homebrew/Library/Taps/aurabx/test-tap/Formula/

# Test installation
brew install aurabx/test-tap/runbeam --verbose

# Verify
runbeam --version

# Cleanup
brew untap aurabx/test-tap
```

Or use Homebrew's built-in testing:

```bash
brew test-bot --root-url=https://github.com/aurabx/runbeam-cli/releases/download/v0.8.0 aurabx/tap/runbeam
```

## Formula Maintenance

### When to Update

- New releases of runbeam-cli
- Dependency updates (rare for binary distribution)
- Bug fixes in the formula itself

### Version Management

The formula version should match the runbeam-cli release tag. For example:
- runbeam-cli v0.8.0 → Homebrew formula version 0.8.0
- runbeam-cli v0.8.1 → Homebrew formula version 0.8.1

### Platform Support

The current formula supports:
- macOS Intel (x86_64-apple-darwin)
- macOS ARM (aarch64-apple-darwin) — Apple Silicon
- Linux Intel (x86_64-unknown-linux-musl)
- Linux ARM (aarch64-unknown-linux-musl)

Update the formula if adding/removing platform support.

## User Documentation

Point users to installation docs:

```markdown
## Installation

### Homebrew (Recommended for macOS/Linux)

```bash
brew tap aurabx/tap
brew install runbeam
```

### Cargo (All Platforms)

```bash
cargo install runbeam-cli
```

### Direct Download

Download from [GitHub Releases](https://github.com/aurabx/runbeam-cli/releases)
```

## Troubleshooting

### "tap not found"

Ensure the tap URL is correct:
```bash
brew tap aurabx/tap https://github.com/aurabx/homebrew-tap
```

### "runbeam: incompatible architecture"

The formula detected an unsupported platform. Check that SHA256s are defined for that platform in the formula.

### "SHA256 mismatch"

The downloaded binary's checksum doesn't match. This usually indicates:
1. Network corruption (try again)
2. Wrong binary version (check the formula is updated)
3. The binary was modified after release (security concern)
