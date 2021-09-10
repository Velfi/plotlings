# Plotlings

A collection of art generators that can export SVGs. I purchased an [AxiDraw V3] and it's been a lot of fun. Feel free to use these generators to make your own art.

## Running it yourself

Rust is required. To export to SVG, set the `SVG_EXPORT_DIRECTORY` environment variable. The programs support `.env` files.

## Generators

### Line Groups

Run it with this command: `cargo run release --bin line_groups`

![Line Groups](/previews/line_groups.png)

The first one I created for this project. It generates groups of lines that look neat.

### Maze

Run it with this command: `cargo run release --bin maze`

![Maze](/previews/maze.png)

Based on the classic maze BASIC program.

### Dune

Run it with this command: `cargo run release --bin dune`

![Dune](/previews/dune.png)

A naïve recreation of [Sohan Murthy's Continuity Correction][continuity-correction] that ended up going in a new direction.

### Line Noise

Run it with this command: `cargo run release --bin line_noise`

![Line Noise](/previews/line_noise.png)

[continuity-correction]: https://sohan.space/portfolio/continuity-correction/
[AxiDraw V3]: https://shop.evilmadscientist.com/productsmenu/846
