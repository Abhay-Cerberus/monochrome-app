# Contributing to Monochrome

Thank you for contributing! This guide will help you get started.

---

## Development Setup

### Prerequisites

- [Rust](https://rustup.rs/)
- [Bun](https://bun.sh/) (preferred) or [Node.js](https://nodejs.org/) v22+
- Platform C++ build tools:
  - **Windows**: MSVC
  - **macOS**: Xcode Command Line Tools
  - **Linux**: `build-essential`

### Quick Start

```bash
git clone https://github.com/YOUR_USERNAME/monochrome.git
cd monochrome
bun install
```

### Development

```bash
# Run with hot-reload
npx tauri dev

# Run tests
bun run test
bun run test:watch

# Check code quality
bun run lint
bun run format
```

### Build

```bash
# Desktop app
npx tauri build

# Web version (outputs to dist/)
bun run build
```

---

## Code Quality

All code must pass linting before merging.

```bash
# Check everything
bun run lint

# Auto-format
bun run format

# Fix specific issues
bun run lint:js -- --fix   # JavaScript
bun run lint:css -- --fix  # CSS
```

**Note:** GitHub Actions automatically runs linting on all PRs.

---

## Project Structure

```
monochrome/
├── js/              # Application source code
├── src-tauri/       # Tauri desktop app config
├── public/          # Static assets
├── index.html       # Entry point
└── package.json     # Dependencies & scripts
```

---

## Before You Contribute

To ensure a smooth contribution process and avoid wasted effort, please adhere to the following guidelines before starting any major work.

### Consult on Major Features

If you're looking into contributing a big feature, please speak with us before starting work. You might be implementing something we are already working on, or a feature that could create more issues long-term. You can reach us via a [GitHub Issue](https://github.com/monochrome-music/monochrome/issues) or on our **[Discord](https://monochrome.tf/discord)**.

### Open Draft PRs Early

Whether you've spoken with us or not, we highly recommend opening **Draft Pull Requests** early. This allows us to catch potential issues before you spend too much time on them. Large PRs that appear suddenly are often difficult to review, and we may close them if they conflict with internal work we haven't pushed yet.

### AI as a Tool

**AS A TOOL**, AI is a great way to help you navigate our (admittedly messy) codebase or refactor logic. We actually encourage using it to speed up your workflow, but we have a zero-tolerance policy for Vibecoding.

#### Permissible (and encouraged):

- Using AI as a tutor to help you understand a specific module or issue.
- Using AI to help clean up your code or write clearer PR descriptions.
- Making sure you understand **every line** of code you submit.
- Mentioning in your PR if you used AI to help with a specific section.

#### Prohibited (AI Slop):

- **Vibecoding** the entire PR (letting AI write the code without human oversight).
- Submitting code you don't actually understand or haven't tested.
- Ignoring edge cases because the AI didn't suggest them.

> :warning:: If we can verify that a Pull Request is just unvetted AI/Vibecoded Work, **it will be automatically closed without review.** If you can't explain your code, it doesn't belong in Monochrome.

### No Hard Feelings

If we end up closing your Pull Request, please don't feel bad about it! We **really appreciate** you taking the time to help out with Monochrome.

There are a lot of reasons why we might close a PR, and most of them have nothing to do with you. It might be because:

- We’re already working on the same thing behind the scenes.
- The feature doesn't quite fit where the project is headed right now.
- We’re still undecided on how a certain part of the app should work.
- It doesn't quite follow the guidelines we've set here.

In short: we don't hate you, and we aren't trying to be mean. We know how much work goes into a PR, and we're grateful you chose to spend your time on our project. Even if a PR gets closed, we'd still love to have you around the community!

---

## Before Contributing

### Discuss Large Changes

Open a [GitHub Issue](https://github.com/monochrome-music/monochrome/issues) or ask on [Discord](https://monochrome.tf/discord) before starting major work to avoid duplicate effort.

### Open Draft PRs Early

Submit draft PRs early to catch issues before investing too much time.

### AI Code Policy

AI is allowed as a tool (tutoring, cleanup, understanding code). Not allowed: Vibecoding entire PRs, submitting code you don't understand, or ignoring edge cases.

If we detect unvetted AI work, the PR will be automatically closed. Explain your code clearly.

---

## Contribution Workflow

1. Create a branch: `git checkout -b feature/name` or `fix/description`
2. Make changes, follow code style
3. Run `bun run lint` and `bun run test`
4. Commit with conventional commit message
5. Push and open a Pull Request

---

## Commit Message Guidelines

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>
```

**Types:** `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`

**Scopes:** `player`, `ui`, `api`, `library`, `playlists`, `lyrics`, `auth`, `settings`, `theme`, etc.

**Examples:**
```
feat(playlists): add shuffle button
fix(player): resolve audio cutoff
docs(README): add build instructions
refactor(api): simplify response handling
```

Use present tense, lowercase, no period. Keep first line under 72 characters.

---

## Deployment

### Desktop App

```bash
npx tauri build
# Output: src-tauri/target/release/bundle/
```

### Web Version

```bash
bun run build
# Output: dist/
```

Deployment automation runs on `main` branch pushes (Cloudflare Pages for web).

---

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on constructive feedback
- Respect different viewpoints and experiences

Thank you for contributing to Monochrome!
