Source: The standard Barbara test image, downloaded from <http://demo.ipol.im/demo/103/input/barbara.jpg>, which seemed to have a larger and more complete image than most. The image was then (losslessly) converted into PNG format.

![The Barbara test image. She sits on the ground wearing black-and-white striped pants and head covering, beside a table with a checkerboard tablecloth and toys. In the background are books and a wicker chair.](original.png)

This test is useful for seeing behavior on textures that are already high-frequency. Remarkably, these textures seem to be well preserved.

Additionally, when run on the RGBI palette, this image demonstrates a particularly dramatic example of a failure mode for Floyd-Steinberg on small palettes. Clamping intermediate colors to fall within a sensible range fixes the worst of these issues, but throws away error, leading the image to look overall different. Note that this failure is highly palette- and image-dependent, as the result on the Macintosh 16-color palette is completely reasonable. The simplex dither avoids these problems but gives a slightly more grainy result.

![The Barbara test image dithering to the RGBI palette using Floyd-Steinberg. Many regions are incorrectly colored, and pink lines run across the photo.](floyd-steinberg@rgbi.png)

![The Barbara test image dithering to the RGBI palette using Floyd-Steinberg, with intermediate colors clamped. Everywhere went back to near the correct color, but the overall image still looks faded.](floyd-steinberg+clamp@rgbi.png)

![The Barbara test image dithering to the Macintosh 16-color palette using Floyd-Steinberg. In contrast to on RGBI, everything looks roughly correct.](floyd-steinberg@macintosh16.png)
