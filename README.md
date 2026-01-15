# Mouse Jiggler

A lightweight Windows GUI application that prevents screen lock and sleep by periodically moving the mouse cursor.

## Features

- Minimal, always-on-top window
- Stealth jiggle - moves cursor 1 pixel and back (barely visible)
- Configurable interval, delay, and distance
- Settings persist between sessions
- Visual countdown timer with progress bar

## Screenshot

```
┌─────────────────────────┐
│     Mouse Jiggler       │
│                         │
│    [Emulate Mouse]      │
│                         │
│      ( -.- )            │
│    Next: 3s [###-----]  │
│                         │
│      [Settings]         │
└─────────────────────────┘
```

## Build

Requires [Rust](https://rustup.rs/).

```bash
cargo build --release
```

Executable: `target/release/project9mouse.exe`

## Usage

1. Run `project9mouse.exe`
2. Click **Emulate Mouse** to start
3. Adjust settings as needed:
   - **Interval** - seconds between jiggles (1-120)
   - **Delay** - pause during jiggle in ms (10-1000)
   - **Distance** - pixels to move (1-100)

## How It Works

Every interval, the cursor moves slightly right then back left:

```
●►◄  (relative movement, returns to original position)
```

This registers as activity, preventing:
- Screen lock
- Screensaver activation
- Sleep mode

## Dependencies

- [eframe](https://github.com/emilk/egui) - GUI framework
- [enigo](https://github.com/enigo-rs/enigo) - Mouse control
- [serde](https://serde.rs/) - Settings serialization

## License

MIT
