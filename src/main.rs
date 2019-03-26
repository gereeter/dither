extern crate clap;
extern crate image;
extern crate rayon;

mod palettes;
mod color;
mod geom;

use color::{Srgb8, LinearRgb, Lab, PseudoLab};
use geom::{Vec3, determinant, subtract};

use rayon::iter::{IntoParallelIterator, ParallelIterator, ParallelBridge};

// Traditional Floyd-Steinberg dithering. Or it would be, except that everything is gamma-correct and using CIEDE2000, which seems to
// result in some pretty drastic changes, or at least occasional failures that look significanty worse than any other implementation I've
// seen. This may be the result of errors being done in floating point, which allows them to grow arbitrarily large and excessive, combined
// with CIEDE2000 continuing to choose a value that doesn't correct for the error. Therefore, there is also a manual clamping that can be
// enabled to force errors within a reasonable range. TODO: make color comparison configuable, and see if that fixes things
fn floyd_steinberg(img: &mut image::RgbImage, palette: &[Srgb8], linear_palette: &[LinearRgb], lab_palette: &[Lab], serpentine: bool, clamp: bool) {
    let select_color = |rgb: LinearRgb| -> usize {
        let lab = Lab::from(rgb);
        // Since error diffusion is mostly inherently serial, we parallelize the search for the nearest color
        if lab_palette.len() < 250 { // TODO: don't just do an ad-hoc switch
            (0..lab_palette.len()).map(|i| (i, Lab::ciede2000_distance2(lab, lab_palette[i])))
                                  .fold((!0, core::f64::INFINITY), |(i1, d1), (i2, d2)| {
                                      if d1 < d2 {
                                          (i1, d1)
                                      } else {
                                          (i2, d2)
                                      }
                                  }).0
        } else {
            (0..lab_palette.len()).into_par_iter()
                                  .map(|i| (i, Lab::ciede2000_distance2(lab, lab_palette[i])))
                                  .reduce(|| (!0, core::f64::INFINITY), |(i1, d1), (i2, d2)| {
                                      if d1 < d2 {
                                          (i1, d1)
                                      } else {
                                          (i2, d2)
                                      }
                                  }).0
        }
    };

    let do_clamp = |rgb: LinearRgb| if clamp { rgb.clamp() } else { rgb };

    // At any point in the loop, this contains the error for the next row in positions 0..=x+1 and the error
    // for the current row in the remaining positions
    let mut row_error: Vec<Vec3<LinearRgb>> = core::iter::repeat(Vec3::zero()).take(img.width() as usize).collect();
    for y in 0..img.height()-1 {
        if y % 10 == 0 {
            eprintln!("Processing line {}", y);
        }

        // If serpentine is set, then we use the same algorithm, but traverse the arrays in the opposite order
        // every other row.
        let widthm1 = img.width() - 1;
        let flip = |x| if serpentine && (y & 1) != 0 { widthm1 - x } else { x };

        // Because row_error[x + 1] holds the next row's error, we need to save the current row's
        // error so that it doesn't get overwritten.
        let mut next_error = row_error[flip(0) as usize];

        // Separate the left side: because we cannot send error off the image, send it downwards.
        let first_target = do_clamp(next_error + LinearRgb::from(*img.get_pixel(flip(0), y)));
        next_error = row_error[flip(1) as usize];
        let first_selection = select_color(first_target);
        let first_error = subtract(first_target, linear_palette[first_selection]);
        *img.get_pixel_mut(flip(0), y) = palette[first_selection];
        row_error[flip(0) as usize] = first_error * 0.5;
        row_error[flip(1) as usize] = first_error * 0.0625;
        next_error += first_error * 0.4375;

        for x in 1..img.width()-1 {
            let target = do_clamp(next_error + LinearRgb::from(*img.get_pixel(flip(x), y)));
            next_error = row_error[flip(x + 1) as usize];
            let selection = select_color(target);
            let error = subtract(target, linear_palette[selection]);
            *img.get_pixel_mut(flip(x), y) = palette[selection];
            row_error[flip(x - 1) as usize] += error * 0.1875;
            row_error[flip(x) as usize] += error * 0.3125;
            row_error[flip(x + 1) as usize] = error * 0.0625;
            next_error += error * 0.4325;
        }

        // Separate the righ side: to avoid sending error over the edge, distribute it proportionally downwards.
        let last_target = do_clamp(next_error + LinearRgb::from(*img.get_pixel(flip(img.width() - 1), y)));
        let last_selection = select_color(last_target);
        let last_error = subtract(last_target, linear_palette[last_selection]);
        *img.get_pixel_mut(flip(img.width() - 1), y) = palette[last_selection];
        row_error[flip(img.width() - 2) as usize] += last_error * 0.375;
        row_error[flip(img.width() - 1) as usize] += last_error * 0.625;
    }

    // Handle the last row by pushing error entirely rightwards.
    // TODO: flip for serpentine?
    let mut error = Vec3::zero();
    for x in 0..img.width() {
        let target = do_clamp(error + row_error[x as usize] + LinearRgb::from(*img.get_pixel(x, img.height() - 1)));
        let selection = select_color(target);
        error = subtract(target, linear_palette[selection]);
        *img.get_pixel_mut(x, img.height() - 1) = palette[selection];
    }
}

// Simple quantization: map each pixel to the nearest palette color.
fn nearest(pixel: Srgb8, palette: &[Srgb8], _linear_palette: &[LinearRgb], lab_palette: &[Lab], _bias: f64) -> Srgb8 {
    let lab_pixel = Lab::from(pixel);
    let mut best_dist = std::f64::INFINITY;
    let mut best = None;
    for (&opt, &lab_opt) in palette.iter().zip(lab_palette.iter()) {
        let dist = Lab::ciede2000_distance2(lab_opt, lab_pixel);
        if dist < best_dist {
            best_dist = dist;
            best = Some(opt);
        }
    }

    best.unwrap()
}

// A simple dithering scheme in the style of Yliluoma or simplex dithering: it chooses two nearby palette colors,
// then quasirandomly chooses between them. This is really just a toy and should probably be removed. It doesn't
// work very well.
fn nearest2_inv2_dist(pixel: Srgb8, palette: &[Srgb8], _linear_palette: &[LinearRgb], lab_palette: &[Lab], bias: f64) -> image::Rgb<u8> {
    let lab_pixel = Lab::from(pixel);
    let mut best_dist = std::f64::INFINITY;
    let mut best2_dist = std::f64::INFINITY;
    let mut best = None;
    let mut best2 = None;
    for (&opt, &lab_opt) in palette.iter().zip(lab_palette.iter()) {
        let dist = Lab::ciede2000_distance2(lab_opt, lab_pixel);
        if dist < best_dist {
            best2_dist = best_dist;
            best2 = best;
            best_dist = dist;
            best = Some(opt);
        } else if dist < best2_dist {
            best2_dist = dist;
            best2 = Some(opt);
        }
    }

    let inv2_best = 1.0 / best_dist;
    let inv2_best2 = 1.0 / best2_dist;
    let total = inv2_best + inv2_best2;
    if bias * total <= inv2_best {
        best.unwrap()
    } else {
        best2.unwrap()
    }
}

// The simplex ordered dithering algorithm. The majority of the actual code here is dealing with edge cases.
// The basic idea is to look for a simplex in linear space that contains the actual color value of the pixel
// we wish to represent. Then, we represent that pixel in barycentric coordinates within the simplex so that
// it can be seen as a weighted average of simplex values. Finally, we use the bias value to quasirandomly
// pick between the corners of the simplex, weighted by the barycentric coordinates: this has an expected
// value equal to the actual pixel value.
//
// When choosing a simplex, we wish to minimize the number of jarring transitions that are supposed to average
// out, so we want a simplex that is psychovisually small and close to the desired color. As such, we sort all
// palette points by their distance to the desired color and choose the simplex that fits inside the smallest
// ball around the desired color, i.e. we minimize that maximum deviation from our desired color. Ties are broken
// by considering the closeness of successively closer corners of the simplex.
//
// Once we've chosen a simplex, we need to "randomly" choose between the vertices. However, since our bias is
// quasirandom, it's important that we don't add more randomness, which would decrease the quality of the dither.
// To ensure consistency, we therefore first sort the vertices by luma. This means that low bias values result
// in a darker choice and high bias values give rise to a lighter choice, consistently across the image. Without
// this, the dither ends up very clumpy and not sharp.
//
//
// Note that if P is inside a simplex, then, for any other point C, P must be inside one of the 4 simplices
// obtained by replacing one of the corners with C. For the purposes of dithering, this implies that if our
// desired color is inside any simplex at all, then we can replace one of the corners with whatever palette color
// is closest to our desired color. Moreover, this substitution can only decrease the size of the simplex
// according to our metric. Therefore, we can safely assume that one of the corners of our simplex will be the
// nearest palette color, and we only need to search for 3 other corners, not 4. This reduces the worst-case
// runtime of our naive search from O(n^4) to O(n^3).
//
// There are definitely faster ways of performing this search - for example, an online convex hull algorithm
// on the surface of a sphere could be used to add palette points one by one until a simplex is found, then
// tear down point by point to determine the remaining corners. However, I haven't put much effort into optimizing
// yet, and this code is already quite complicated due to all the edge cases. Moreover, due to the complexity
// of CIEDE2000, much of the runtime seems to be consumed in color distance calculations. Avoiding doing the
// calculation between every pixel and every palette color, possibly with some sort of spatial data structure,
// would seem more prudent and effective. TODO: Optimize!
fn tight_simplex(pixel: Srgb8, palette: &[Srgb8], linear_palette: &[LinearRgb], lab_palette: &[Lab], bias: f64) -> Srgb8 {
    let linear_pixel = LinearRgb::from(pixel);
    let lab_pixel = Lab::from(linear_pixel);

    // The palette, sorted by distance from the pixel. We cache the distance, linear color, and luma and also store a flag
    // to indicate when we know for certain that a color will be useless.
    let mut trans_palette: Vec<_> = palette.iter().cloned().enumerate().map(|(i, rgb8)| {
        let linear = subtract(linear_palette[i], linear_pixel); // Shift to our pixel being at the origin, since this simplifies a good chunk of the math.
        let lab = lab_palette[i];
        let dist2 = Lab::ciede2000_distance2(lab_pixel, lab);
        (rgb8, linear, dist2, false, lab.l)
    }).collect();
    trans_palette.sort_unstable_by(|&(_, _, d1, _, _), &(_, _, d2, _, _)| d1.partial_cmp(&d2).unwrap());

    // Fast path that also avoids some of the most annoying edge cases: if we're on a palette color, just return that.
    if trans_palette[0].2 < 1e-20 {
        return trans_palette[0].0;
    }

    'outer_loop:
    for index3 in 3..trans_palette.len() {
        for index2 in 2..index3 {
            if trans_palette[index2].3 { continue; } // Skip known-useless colors.
            for index1 in 1..index2 {
                if trans_palette[index1].3 { continue; }
                let index0 = 0;

                // Calculate the signed volumes of the simplices with our pixel and 3 other points.
                // If all of these have the same sign, then we are inside the simplex.
                let points = [
                    trans_palette[index0].1,
                    trans_palette[index1].1,
                    trans_palette[index2].1,
                    trans_palette[index3].1,
                ];
                let d0 = determinant([points[1], points[3], points[2]]);
                let d1 = determinant([points[0], points[2], points[3]]);
                let d2 = determinant([points[0], points[3], points[1]]);
                let d3 = determinant([points[0], points[1], points[2]]);
                let d_all = d0 + d1 + d2 + d3;

                // Handle degenerate cases separately
                if d3.abs() < 1e-15 || d2.abs() < 1e-15 || d1.abs() < 1e-15 || d0.abs() < 1e-15 {//d_all.abs() < 1e-15 {
                    //eprintln!("  Degeneracy ({},{},{},{})! d_all={}, d0={}, d1={}, d2={}, d3={}", index0, index1, index2, index3, d_all, d0, d1, d2, d3);
                    for &face in &[[0,1,2],[0,1,3],[0,2,3],[1,2,3]] {
                        let face_points = [points[face[0]], points[face[1]], points[face[2]]];
                        let det = determinant(face_points);
                        if det.abs() < 1e-12 {
                            let vec_01 = face_points[1] - face_points[0];
                            let vec_02 = face_points[2] - face_points[0];
                            let normal = vec_01.cross(vec_02);
                            let size2 = normal.dot(normal);
                            //eprintln!("    Face ({},{},{}) has determinant {}, size2 {}", face[0], face[1], face[2], det, size2);
                            if size2 > 1e-18 {
                                let coeff_0 = normal.dot(face_points[1].cross(face_points[2]));
                                let coeff_1 = normal.dot(face_points[2].cross(face_points[0]));
                                let coeff_2 = normal.dot(face_points[0].cross(face_points[1]));
                                //eprintln!("    Valid triangle: {},{},{}", coeff_0, coeff_1, coeff_2);

                                if coeff_0 >= -1e-15 && coeff_1 >= -1e-15 && coeff_2 >= -1e-15 {
                                    //eprintln!("      Success!");
                                    let indexes = [index0, index1, index2, index3];
                                    let mut simplex = [(coeff_0, face[0]), (coeff_1, face[1]), (coeff_2, face[2])];
                                    simplex.sort_unstable_by(|&(_, f1), &(_, f2)| trans_palette[indexes[f1]].4.partial_cmp(&trans_palette[indexes[f2]].4).unwrap());
                                    if bias <= simplex[0].0 / size2 {
                                        return trans_palette[indexes[simplex[0].1]].0;
                                    } else if bias - simplex[0].0 / size2 <= simplex[1].0 / size2 {
                                        return trans_palette[indexes[simplex[1].1]].0;
                                    } else {
                                        return trans_palette[indexes[simplex[2].1]].0;
                                    }
                                }
                            }
                        }
                    }
                    for &edge in &[[0,1], [0,2], [0,3], [1,2], [1,3], [2,3]] {
                        let edge_points = [points[edge[0]], points[edge[1]]];
                        let vec_01 = edge_points[1] - edge_points[0];
                        let vec_0p = -edge_points[0];
                        let normal = vec_01.cross(vec_0p);
                        if normal.dot(normal) < 1e-15 {
                            let len2 = vec_01.dot(vec_01);
                            let partial = vec_0p.dot(vec_01);
                            if partial >= 0.0 && partial <= len2 {
                                let indexes = [index0, index1, index2, index3];
                                let mut simplex = [(partial / len2, edge[0]), (1.0 - partial / len2, edge[1])];
                                simplex.sort_unstable_by(|&(_, f1), &(_, f2)| trans_palette[indexes[f1]].4.partial_cmp(&trans_palette[indexes[f2]].4).unwrap());
                                if bias <= simplex[0].0 {
                                    return trans_palette[indexes[simplex[0].1]].0;
                                } else {
                                    return trans_palette[indexes[simplex[1].1]].0;
                                }
                            }
                        }
                    }
                } else if d_all.signum() == d0.signum() &&
                          d_all.signum() == d1.signum() &&
                          d_all.signum() == d2.signum() &&
                          d_all.signum() == d3.signum() {

                    // We're inside the simplex! 
                    let mut simplex = [(d0, index0), (d1, index1), (d2, index2), (d3, index3)];
                    simplex.sort_unstable_by(|&(_, i1), &(_, i2)| trans_palette[i1].4.partial_cmp(&trans_palette[i2].4).unwrap());

                    let mut bias_left = bias;
                    if bias_left <= simplex[0].0 / d_all {
                        return trans_palette[simplex[0].1].0;
                    } else {
                        bias_left -= simplex[0].0 / d_all;
                    }
                    if bias_left <= simplex[1].0 / d_all {
                        return trans_palette[simplex[1].1].0;
                    } else {
                        bias_left -= simplex[1].0 / d_all;
                    }
                    if bias_left <= simplex[2].0 / d_all {
                        return trans_palette[simplex[2].1].0;
                    } else {
                        return trans_palette[simplex[3].1].0;
                    }
                } else if d0.signum() == d1.signum() &&
                          d0.signum() == d2.signum() {
                    trans_palette[index3].3 = true;
                    continue 'outer_loop;
                }
            }
        }
    }

    // No simplex contains our point. Therefore, search for a triangle we can project onto or a line segment or the closest point (whichever allows
    // us to project to the closest point).

    // Start with the closest point
    let mut best_sample = trans_palette[0].0;
    let mut best_dist2 = trans_palette[0].2;

    // FIXME: The following code does orthogonal projection in linear space, which is wrong. We want the plane/line defined by linear space, but we want the
    // CIEDE-closest point. As an approximation, we try to make something that looks a bit more like LAB but is a linear transformation.
    let plab_pixel = PseudoLab::from(linear_pixel);

    // Try to beat it with a line segment
    for index2 in 1..trans_palette.len() {
        for index1 in 0..index2 {
            // TODO: just scale directly
            let point_1 = PseudoLab::from(trans_palette[index1].1 + linear_pixel);
            let point_2 = PseudoLab::from(trans_palette[index2].1 + linear_pixel);

            // Project our point onto the line segment
            let vec_12 = subtract(point_2, point_1);
            let vec_1p = subtract(plab_pixel, point_1);
            let mag2_12 = vec_12.dot(vec_12);
            let proj_1p = vec_12 * (vec_1p.dot(vec_12) / mag2_12);
            let projected = proj_1p + point_1;

            // Test whether we are actually in the segment, not elsewhere on the line
            let offset_mag = proj_1p.dot(vec_12);
            //eprintln!("Line: {}", offset_mag / mag2_12);
            if offset_mag >= 0.0 && offset_mag <= mag2_12 {
                // Only consider points that are better than the best seen
                let lab_projected = Lab::from(projected);
                let dist2 = Lab::ciede2000_distance2(lab_projected, lab_pixel);
                //eprintln!(" Projected: {:?}", LinearRgb::from(projected).data);
                //eprintln!(" Dist: {}, Point: {}", dist2, best_dist2);
                if dist2 < best_dist2 {
                    // Ensure our palette is sorted
                    let bias_shifted = if trans_palette[index1].4 < trans_palette[index2].4 {
                        1.0 - bias
                    } else {
                        bias
                    };

                    // Select between the two points
                    if bias_shifted * mag2_12 <= offset_mag {
                        best_sample = trans_palette[index2].0;
                    } else {
                        best_sample = trans_palette[index1].0;
                    }
                    best_dist2 = dist2;
                }
            }
        }
    }

    // Try to beat it with a triangle
    for index3 in 2..trans_palette.len() {
        for index2 in 1..index3 {
            for index1 in 0..index2 {
                let point_1 = PseudoLab::from(trans_palette[index1].1 + linear_pixel);
                let point_2 = PseudoLab::from(trans_palette[index2].1 + linear_pixel);
                let point_3 = PseudoLab::from(trans_palette[index3].1 + linear_pixel);

                // Project onto the plane
                let vec_12 = subtract(point_2, point_1);
                let vec_13 = subtract(point_3, point_1);
                let vec_1p = subtract(plab_pixel, point_1);
                let normal = vec_12.cross(vec_13);
                let mag2_normal = normal.dot(normal); // TODO: What happens if this is 0?
                let offset = normal * (vec_1p.dot(normal) / mag2_normal); // Do the projection by projecting onto the normal and subtracting
                let projected = (-offset) + plab_pixel;
                let lab_projected = Lab::from(projected);
                let dist2 = Lab::ciede2000_distance2(lab_projected, lab_pixel);
                if dist2 < best_dist2 {
                    let proj_p1 = offset - vec_1p;
                    let proj_p2 = proj_p1 + vec_12;
                    let proj_p3 = proj_p1 + vec_13;

                    let d_area_all = mag2_normal.sqrt();
                    let normal_23 = proj_p2.cross(proj_p3);
                    let d_area_23 = normal_23.dot(normal_23).sqrt();
                    let normal_13 = proj_p1.cross(proj_p3);
                    let d_area_13 = normal_13.dot(normal_13).sqrt();

                    let coord1 = d_area_23 / d_area_all;
                    let coord2 = d_area_13 / d_area_all;
                    let coord3 = 1.0 - coord1 - coord2;

                    if coord1 >= 0.0 &&
                       coord2 >= 0.0 &&
                       coord3 >= 0.0 {

                        let mut simplex = [(coord1, index1), (coord2, index2), (coord3, index3)];
                        simplex.sort_unstable_by(|&(_, i1), &(_, i2)| trans_palette[i1].4.partial_cmp(&trans_palette[i2].4).unwrap());

                        let mut bias_left = bias;
                        if bias_left <= simplex[0].0 {
                            best_sample = trans_palette[simplex[0].1].0;
                        } else {
                            bias_left -= simplex[0].0;
                            if bias_left <= simplex[1].0 {
                                best_sample = trans_palette[simplex[1].1].0;
                            } else {
                                best_sample = trans_palette[simplex[2].1].0;
                            }
                        }

                        best_dist2 = dist2;
                    }
                }
            }
        }
    }

    return best_sample;
}

fn into_rgb(img: image::DynamicImage) -> image::RgbImage {
    if let image::DynamicImage::ImageRgb8(rgb_img) = img {
        rgb_img
    } else {
        img.to_rgb()
    }
}

fn main() {
    let arg_matches =
        clap::App::new("dither")
            .version("0.1")
            .author("Jonathan S <gereeter+code@gmail.com>")
            .about("High-quality ordered dithering")
            .arg(clap::Arg::with_name("PALETTE").short("p").long("palette").takes_value(true).default_value("websafe").help("Chooses the palette to quantize to"))
            .arg(clap::Arg::with_name("ALGORITHM").short("a").long("algorithm").takes_value(true).default_value("simplex").help("Chooses the dithering algorithm to use"))
            .arg(clap::Arg::with_name("OUTPUT").short("o").long("output").takes_value(true).default_value("out.png").help("Sets where to write the dithered file to"))
            .arg(clap::Arg::with_name("IMAGE").required(true).help("Sets the image to dither"))
            .get_matches();

    let file_name = arg_matches.value_of_os("IMAGE").unwrap();
    let out_file_name = arg_matches.value_of_os("OUTPUT").unwrap();
    let mut img = into_rgb(image::open(&file_name).unwrap());

    let palette = match arg_matches.value_of("PALETTE").unwrap() {
        "bw" | "1bit" => vec![Srgb8 { data: [0,0,0] }, Srgb8 { data: [255,255,255] }],
        "websafe" | "r6g6b6" => palettes::grid(6, 6, 6),
        "3bit" | "r2g2b2" => palettes::grid(2, 2, 2),
        "rgbi" => palettes::RGBI.to_vec(),
        "microsoft16" => palettes::MICROSOFT16.to_vec(),
        "macintosh16" => palettes::MACINTOSH16.to_vec(),
        "r3g3b2" => palettes::grid(3, 3, 2),
        "8bit" | "r8g8b4" => palettes::grid(8, 8, 4),
        "12bit" | "r16g16b16" => palettes::grid(16, 16, 16),
        "15bit" | "r32g32b32" => palettes::grid(32, 32, 32),
        "yliluoma" => palettes::YLILUOMA_EXAMPLE.to_vec(),
        "yliluoma_alternate" => palettes::YLILUOMA_EXAMPLE_ALTERNATE.to_vec(),
        "octree16" => palettes::make_box_palette(16, img.pixels().cloned(), palettes::Split::Half, true),
        "octree256" => palettes::make_box_palette(256, img.pixels().cloned(), palettes::Split::Half, true),
        "octree16-notight" => palettes::make_box_palette(16, img.pixels().cloned(), palettes::Split::Half, false),
        "octree256-notight" => palettes::make_box_palette(256, img.pixels().cloned(), palettes::Split::Half, false),
        "mediancut-box16" => palettes::make_box_palette(16, img.pixels().cloned(), palettes::Split::Median, true),
        "mediancut-box256" => palettes::make_box_palette(256, img.pixels().cloned(), palettes::Split::Median, true),
        "meancut-box16" => palettes::make_box_palette(16, img.pixels().cloned(), palettes::Split::Mean, true),
        "meancut-box256" => palettes::make_box_palette(256, img.pixels().cloned(), palettes::Split::Mean, true),
        "simplex16" => palettes::make_simplex_palette(16, img.pixels().cloned()),
        "simplex256" => palettes::make_simplex_palette(256, img.pixels().cloned()),
        _ => panic!("Unrecognized palette!")
    };

    eprintln!("Generated palette. Dithering...");

    let linear_palette: Vec<_> = palette.iter().cloned().map(LinearRgb::from).collect();
    let lab_palette: Vec<_> = linear_palette.iter().cloned().map(Lab::from).collect();

    let algorithm = match arg_matches.value_of("ALGORITHM").unwrap() {
        "nearest" => nearest,
        "nearest2:d^-2" => nearest2_inv2_dist,
        "simplex" => tight_simplex,
        "floyd-steinberg" => {
            floyd_steinberg(&mut img, &palette, &linear_palette, &lab_palette, false, false);
            img.save(out_file_name).unwrap();
            return;
        },
        "floyd-steinberg+serpentine" => {
            floyd_steinberg(&mut img, &palette, &linear_palette, &lab_palette, true, false);
            img.save(out_file_name).unwrap();
            return;
        },
        "floyd-steinberg+clamp" => {
            floyd_steinberg(&mut img, &palette, &linear_palette, &lab_palette, false, true);
            img.save(out_file_name).unwrap();
            return;
        },
        "floyd-steinberg+clamp+serpentine" => {
            floyd_steinberg(&mut img, &palette, &linear_palette, &lab_palette, true, true);
            img.save(out_file_name).unwrap();
            return;
        },
        _ => panic!("Unrecognized algorithm!")
    };

    img.enumerate_pixels_mut().par_bridge().for_each(|(x, y, pixel)| {
        // As suggested in The Unreasonable Effectiveness of Quasirandom Sequences (Martin Roberts),
        // using a simple linear function based on the plastic number, composed with a triangle wave,
        // gives good results for bias.
        // TODO: make this configurable?
        let plastic = 1.32471795724474602596;
        let r_bias = (x as f64 / plastic + y as f64 / plastic.powi(2)).fract();
        let bias = if r_bias < 0.5 {
            2.0 * r_bias
        } else {
            2.0 - 2.0 * r_bias
        };

        if x == 0 && y % 10 == 0 {
            eprintln!("Processing line {}", y);
        }

        //eprintln!();
        //eprintln!("Pixel at ({}, {})", x, y);
        *pixel = algorithm(*pixel, &palette, &linear_palette, &lab_palette, bias);
    });

    img.save(out_file_name).unwrap();
}

