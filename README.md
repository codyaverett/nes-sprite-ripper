# nes-sprite-ripper

Extract sprite tiles from NES ROM files and export them as PNG spritesheets.

## Features

- Parses iNES ROM format and decodes CHR ROM tile data
- Exports spritesheets as PNG images
- Interactive TUI preview (via ratatui) before exporting
- Optional transparent background
- Filter out empty/blank tiles
- Select specific tile ranges

## Installation

```sh
cargo install --path .
```

## Usage

```sh
# Basic export
nes-sprite-ripper game.nes

# Specify output path
nes-sprite-ripper game.nes -o sprites.png

# Preview in terminal before exporting
nes-sprite-ripper game.nes --preview

# Transparent background and skip empty tiles
nes-sprite-ripper game.nes --transparent-bg --skip-empty

# Export only a range of tiles
nes-sprite-ripper game.nes --tiles 0-127
```

## Options

| Flag | Description |
|------|-------------|
| `-o, --output <PATH>` | Output PNG file path (defaults to `<rom>_sprites.png`) |
| `--preview` | Show interactive TUI preview before exporting |
| `--transparent-bg` | Make background color transparent |
| `--skip-empty` | Skip empty/blank tiles |
| `--tiles <START-END>` | Select tile range (e.g., `0-255`) |

## License

MIT
