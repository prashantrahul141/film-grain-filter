<h1 align="center">Film Grain Filter</h1>
<p align="center">
Film grain or granularity is the random optical texture of processed photographic film due to the presence of small particles of a metallic silver, or dye clouds, developed from silver halide that have received enough photons. <a href="https://en.wikipedia.org/wiki/Film_grain">learn more. </a>
</p>

<h2>Usage</h2>

1. Clone the repository / or download the source code.

```sh
git clone https://github.com/prashantrahul141/film-grain-filter
```

2. Build using cargo.

```sh
cargo build
```

3. Using

put your `"your_image.jpg"` in the build folder, same place the `main` executable is.

```sh
main.exe --filename "your_image.jpg" i --intensity 1.0 --lumen 0.5
```

Intensity is the custom value to change the colors overall, with testing i found keeping its value close to 1.0 works best.

Lumen is how much luminosity of the image gets affected by the random noise. with testing i found keeping its value close to 0.5 works best.
