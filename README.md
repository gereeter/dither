A program for reducing images to small palettes.

The primary innovation is the introduction of a new ordered dithering algorithm, called simplex dithering, that works with arbitrary palettes. Unlike many other such algorithms, it is provably "correct": as area increases, the average color of the resulting dithered image will converge to the average color of the original image.

This accuracy is the primary goal of this implementation. As a result, quality sometimes suffers compared to other palletization software. To remain accurate, the colors used don't just need to resemble the target colors, but they must be in some sense more extreme than the target colors so that they can average out to the correct middle value. Including the corners of the RGB cube (white, red, green, blue, yellow, cyan, magenta, and black: this is also the `3bit` palette selectable in the program) is always sufficient to approximate anything, but isn't always pretty. Traditional palette selection, using K-Means or related methods, aims to produce a set of colors that are in the middle of the targets, minimizing distance and improving the quality of simple quantization, but it fails to represent the full range of colors. This trades high frequency noise (which is more noticeable to the human eye) for low frequency error.

# Installation

If you already have [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed, you can use it directly to install `dither`:

```
cargo install --git https://github.com/gereeter/dither
```

# Usage

```
USAGE:
    dither [OPTIONS] <IMAGE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --algorithm <ALGORITHM>    Chooses the dithering algorithm to use [default: simplex]
    -b, --bias <BIAS>              Chooses the bias pattern for ordered dithering algorithms [default: plastic+triangle]
    -d, --distance <DISTANCE>      Chooses how to calculate how far apart colors are [default: CIEDE2000]
    -o, --output <OUTPUT>          Sets where to write the dithered file to [default: out.png]
    -p, --palette <PALETTE>        Chooses the palette to quantize to [default: simplex]
    -c, --colors <PALETTE_SIZE>    How many colors to use in a procedural palette [default: 16]

ARGS:
    <IMAGE>    Sets the image to dither
```

For example, if you want to dither an image to only use websafe colors, you might run

```
dither --palette websafe --output image_dithered.png image.png
```

Available algorithms include
- `nearest`, which implements basic image quantization mapping every pixel to its nearest palette color, often producing large blocks of solid color,
- `simplex`, the new innovation of this program, which mixes up to 4 palette colors to perfectly accurately approximate an image with an ordered / stable dither pattern, and
- `floyd-steinberg`, implementing [Floyd-Steinberg dithering](https://en.wikipedia.org/wiki/Floyd%E2%80%93Steinberg_dithering), an error diffusion dithering algorithm, which can produce excellent results but can vary wildly with small changes to the input, and can sometimes result in "worm" artifacts where a specific error is pushed all into a line.

Bias patterns control the look and feel of the resulting dithered image, and only apply to ordered dither algorithms. (Most algorithms implemented are ordered dithers, but, e.g. Floyd-Steinberg dithering is unaffected.)
- The default, `plastic+triangle`, produces a very even fabric-like pattern, based on the suggestion in [The Unreasonable Effectiveness of Quasirandom Sequences](http://extremelearning.com.au/unreasonable-effectiveness-of-quasirandom-sequences/).
- For an old-school look, use `bayer256`, which implements the classic Bayer matrix ordered dithering on a large scale. Smaller Bayer matrices are available, but they can reduce quality for little gain.
- For something approximating halftoning, use `dot8`, which deliberately groups colors together into larger artifacts. This was mostly intended for debugging, and so its quality isn't ideal for producing that look.