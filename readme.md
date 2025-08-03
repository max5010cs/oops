# oops

  ___                  
 / _ \  ___  _ __  ___ 
| | | |/ _ \| '_ \/ __|
| |_| | (_) | |_) \__ \
 \___/ \___/| .__/|___/
            |_|        


> A real-time code security companion for your terminal.  
> [max5010cs/oops](https://github.com/max5010cs/oops)

## 🤔 What is oops?

`oops` is a terminal-based code security tool built to help you **catch issues before they become f*ckups**.

It can do:

- ✅ Pre-deployment scans (`oops run .`)
- 👀 Live real-time monitoring (`oops watch .`)
- 😅 Injects funny reminders when you're about to mess up

What it looks for:

- 🔐 Hardcoded secrets & credentials
- ☣️ Dangerous patterns (e.g. `eval`, unsafe functions)
- ⚙️ Misconfigurations (e.g. debug mode on)
- 🔓 Over-permissive access/permissions
- 🤡 Common bad practices
- 🧠 Known vulnerable code patterns

It's lightweight, fast, terminal-native, and works on any project.  
You write — it watches. You commit — it warns. That's it.

---

## ⚙️ Installation

Clone and install globally via Cargo:

```bash
git clone https://github.com/max5010cs/oops.git
cd oops
cargo install --path . --force
oops
The rest? You’ll find out 😉




🚧 Coming soon...
💬 Language-specific rules (Rust, Python, PHP, JS, etc.)

🛠 Custom config with .oops.toml

🧹 Safe auto-fix suggestions (where possible)
