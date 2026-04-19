 <p align="center">
  <a href="https://monochrome.tf">
    <img src="https://github.com/monochrome-music/monochrome/blob/main/public/assets/512.png?raw=true" alt="Monochrome Logo" width="150px">
  </a>
</p>

<h1 align="center">Monochrome</h1>

<p align="center">
  <strong>An open-source, privacy-respecting, ad-free music app.</strong>
</p>

<p align="center">
  <a href="https://monochrome.tf">Website</a> -
  <a href="https://ko-fi.com/monochromemusic">Donate</a> -
  <a href="#features">Features</a> -
  <a href="#usage">Usage</a> -
  <a href="#self-hosting">Self-Hosting</a> -
  <a href="CONTRIBUTING.md">Contributing</a>
</p>

<p align="center">
  <a href="https://github.com/Abhay-Cerberus/monochrome-app/stargazers">
    <img src="https://img.shields.io/github/stars/Abhay-Cerberus/monochrome-app?style=for-the-badge&color=ffffff&labelColor=000000" alt="GitHub stars">
  </a>
  <a href="https://github.com/Abhay-Cerberus/monochrome-app/forks">
    <img src="https://img.shields.io/github/forks/Abhay-Cerberus/monochrome-app?style=for-the-badge&color=ffffff&labelColor=000000" alt="GitHub forks">
  </a>
  <a href="https://github.com/Abhay-Cerberus/monochrome-app/issues">
    <img src="https://img.shields.io/github/issues/Abhay-Cerberus/monochrome-app?style=for-the-badge&color=ffffff&labelColor=000000" alt="GitHub issues">
  </a>
</p>

---

## What is Monochrome?

**Monochrome** is an open-source, privacy-respecting, ad-free [TIDAL](https://tidal.com) web UI, built on top of Hi-Fi. It provides a beautiful, minimalist interface for streaming high-quality music without the clutter of traditional streaming platforms.

[![Monochrome UI: NASIR by Nas](https://i.samidy.xyz/NASIR.png)](https://monochrome.tf/album/90502209)

[![Monochrome UI: Jump Out by Osamason](https://i.samidy.xyz/jumpout.png)](https://monochrome.tf/album/413189044)

## Features

### Audio Quality

- High-quality High-Res/lossless audio streaming
- Support for local music files
- API caching for improved performance

### Interface

- Dark, minimalist interface optimized for focus
- Animated Album Covers For Supported Albums
- High-quality Music Videos
- Customizable themes & Community Theme Store
- Accurate and unique audio visualizer
- Offline-capable Progressive Web App (PWA)
- Media Session API integration for system controls
- **Native Desktop App** via Tauri (Windows, macOS, Linux) with System Tray support

### Library & Organization

- Recently Played tracking for easy history access
- Comprehensive Personal Library for favorites
- Queue management with shuffle and repeat modes
- Native Podcast support & organization
- Playlist import from other platforms
- Public playlists for social sharing
- Smart recommendations for new songs, albums & artists
- Infinite Recommendation Radio
- Explore Page (Hot & New) for discovering newly added music and whats trending overall or within each genre

### Lyrics & Metadata

- Lyrics support with karaoke mode
- Genius integration for lyrics
- Track downloads with automatic metadata embedding

### Integrations

- Account system for cross-device syncing
- Customizable & Public Profiles
- Real-time Listening Parties for synced playback with friends
- Last.fm and ListenBrainz integration for scrobbling
- OAuth support (Google, Discord, GitHub, Spotify)
- Unreleased music from [ArtistGrid](https://artistgrid.cx)
- Dynamic Discord Embeds
- Artist Biography + Social Links for learning more about your favorite artists
- Multiple API instance support with failover

### Power User Features

- Keyboard shortcuts & Command Palette (CTRL+K) for power users

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (Tauri requirement)
- [Bun](https://bun.sh/) (preferred) or [Node.js](https://nodejs.org/) v20+
- Platform-specific C++ build tools:
  - **Windows**: MSVC (included with Visual Studio)
  - **macOS**: Xcode Command Line Tools
  - **Linux**: `build-essential` and other dev packages

### Development

```bash
git clone https://github.com/Abhay-Cerberus/monochrome-app.git
cd monochrome
bun install  # Install dependencies

# Run the app with hot-reload
npx tauri dev
```

### Build for Production

```bash
npx tauri build
# Output: src-tauri/target/release/bundle/
```

Supports Windows (NSIS installer + portable), macOS (DMG), and Linux (AppImage, Deb).

### Web Development

For web-based development without Tauri:

```bash
bun run dev        # Dev server on localhost:5173
bun run build      # Build web version
bun run preview    # Preview built version
```

---

## Testing

```bash
bun run test          # Run tests
bun run test:watch    # Watch mode
bun run test:headless # Headless mode
```

## Code Quality

```bash
bun run lint       # Check all (ESLint, Stylelint, HTMLHint)
bun run format     # Auto-format code
bun run lint:js -- --fix   # Fix JS issues
bun run lint:css -- --fix  # Fix CSS issues
```

## Self-Hosting (Web Version)

Monochrome can be deployed as a web app via Docker. See [DOCKER.md](DOCKER.md) for configuration.

**⚠️ Note:** Accounts/authentication only work on official instances. Self-hosted instances require Appwrite setup.

## Online Instances

- **[monochrome.tf](https://monochrome.tf)** (Official)
- **[monochrome.samidy.com](https://monochrome.samidy.com)** (Official mirror)

See [INSTANCES.md](INSTANCES.md) for community instances.

#### Building for Production

```bash
bun run build
# or
npm run build
```

---

## Usage

### Basic Usage

1. Visit the [Website](https://monochrome.tf) or your local development server
2. Search for your favorite artists, albums, or tracks
3. Click play to start streaming
4. Use the media controls to manage playback, queue, and volume

### Keyboard Shortcuts

| Shortcut      | Action                       |
| ------------- | ---------------------------- |
| `Space`       | Play / Pause                 |
| `→`           | Seek forward 10s             |
| `←`           | Seek backward 10s            |
| `Shift` + `→` | Next track                   |
| `Shift` + `←` | Previous track               |
| `↑`           | Volume up                    |
| `↓`           | Volume down                  |
| `M`           | Mute / Unmute                |
| `S`           | Toggle shuffle               |
| `R`           | Toggle repeat                |
| `Q`           | Open queue                   |
| `L`           | Toggle lyrics                |
| `/`           | Focus search                 |
| `Esc`         | Close modals                 |
| `[`           | Previous visualizer preset   |
| `]`           | Next visualizer preset       |
| `\`           | Toggle visualizer auto-cycle |
| `Ctrl` + `K`  | Command Palette              |

### Account Features

To sync your library, history, and playlists across devices:

1. Click the "Accounts" Section
2. Sign in with Google or Email
3. Your data will automatically sync across all devices

---

## Contributing

We welcome contributions from the community! Please see our [Contributing Guide](CONTRIBUTING.md) for:

- Setting up your development environment
- Code style and linting
- Project structure
- Before You Contribute
- Commit message conventions
- Deployment information

---

<p align="center">
  <a href="https://fmhy.net/audio#streaming-sites">
    <img src="https://raw.githubusercontent.com/monochrome-music/monochrome/refs/heads/main/public/assets/asseenonfmhy880x310.png" alt="As seen on FMHY" height="50">
  </a>
</p>

<p align="center">
  <a href="https://notbyai.fyi">
    <img src="https://i.samidy.xyz/Developed-By-Humans-Not-By-AI-Badge-black%402x.png" alt="Developed by Humans" height="50">
  </a>
</p>

<p align="center">
  Made with ❤️ by the Monochrome team
</p>

## Star History

<a href="https://www.star-history.com/#monochrome-music/monochrome&type=date&logscale&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=monochrome-music/monochrome&type=date&theme=dark&logscale&legend=top-left" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=monochrome-music/monochrome&type=date&logscale&legend=top-left" />
   <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=monochrome-music/monochrome&type=date&logscale&legend=top-left" />
 </picture>
</a>
