# 🔍 Gemini Search CLI

> *"Why Google when you can just ask an AI directly?"*

A stupidly simple CLI tool that lets you search for anything and get straight-to-the-point answers. No more scrolling through 47 StackOverflow tabs just to install a package.

## 🎯 The Story

Started this as a random side quest because I was tired of:
- Opening browser → Google → Click 5 links → Still confused
- Asking ChatGPT → Getting a novel when I just want a command
- Using `ddgr` and `googler` (shoutout to the legends) but still having to parse through search results

So here we are. An AI-powered terminal search that actually gives you what you asked for. Nothing more, nothing less.

## ✨ What It Does

- **Direct Answers**: Ask "install docker on ubuntu" → Get the exact commands
- **No BS**: Skip the "Docker is a containerization platform that..." essays
- **Terminal Native**: Because leaving the terminal is for quitters
- **Powered by Gemini**: Google's AI, but from your command line

## 🚀 Quick Start

```bash
# Clone this bad boy
git clone https://github.com/yourusername/gemini-search
cd gemini-search

# Get your Gemini API key from Google AI Studio
export GEMINI_API_KEY=your_actual_api_key_here

# Build it
cargo build --release

# Use it
./target/release/gemini-search install alacritty on fedora

# or do
cargo install --path .
# then do
gemini-search install alacritty on fedora
```

**Output:**
```bash
🔍 Searching for: install alacritty on fedora
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  sudo dnf install alacritty
```

That's it. No fluff. No "first, let me explain what Alacritty is..." Just the command you need.

## 🛠 Installation

### Option 1: The Rust Way
```bash
cargo install --git https://github.com/yourusername/gemini-search
```

### Option 2: Build from Source
```bash
git clone https://github.com/yourusername/gemini-search
cd gemini-search
cargo build --release
# Binary will be at ./target/release/gemini-search
```

### Dependencies
If you get OpenSSL errors (because Linux), just install the dev packages:
```bash
# Fedora/RHEL
sudo dnf install openssl-devel pkg-config

# Ubuntu/Debian
sudo apt install libssl-dev pkg-config

# Arch (btw)
sudo pacman -S openssl pkg-config
```

## 🎮 Usage

```bash
# Basic search
gemini-search how to setup nginx

# Installation queries
gemini-search install redis on arch linux

# Configuration stuff
gemini-search setup ssh keys github

# Raw output (no fancy formatting)
gemini-search --raw configure firewall ubuntu
```

## 🎨 Examples

### Package Installation
```bash
$ gemini-search install docker on fedora
  sudo dnf install docker
  sudo systemctl enable docker
  sudo systemctl start docker
```

### Configuration Help
```bash
$ gemini-search setup git ssh key
  ssh-keygen -t ed25519 -C "your_email@example.com"
  eval "$(ssh-agent -s)"
  ssh-add ~/.ssh/id_ed25519
  cat ~/.ssh/id_ed25519.pub
```

### Quick Commands
```bash
$ gemini-search check disk space linux
  df -h
```

## 🔧 Configuration

The tool is intentionally minimal, but you can:

- **Set API Key**: `export GEMINI_API_KEY=your_key`
- **Raw Output**: Use `--raw` flag for unformatted responses
- **Verbose**: The AI is trained to be concise, but if you want more detail, ask specifically

## 🤝 Contributing

This started as a weekend hack, but if you want to make it better:

1. Fork it
2. Make it better
3. PR it
4. Profit? (No actual profit involved)

Ideas welcome:
- Better formatting
- More search providers
- Local caching
- Whatever makes your terminal life easier

## 📝 License

MIT - Do whatever you want with it. Just don't blame me if it formats your hard drive (it won't, but you know, lawyers and stuff).

## 🙏 Credits

- Inspired by `ddgr` and `googler` - the OG terminal search tools
- Google's Gemini API for actually being useful
- Coffee, lots of coffee
- That one StackOverflow answer that saved my life in 2019

## 🐛 Issues

If something breaks:
1. Check if your API key is set
2. Check if your internet works
3. Open an issue with details
4. Or fix it yourself and send a PR (preferred)

---

*Built with ❤️ and mild frustration at having to leave the terminal for simple searches*
