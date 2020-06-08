use crate::color::{self, Srgb8, LinearRgb, Lab};
use crate::geom::{determinant, subtract, midpoint};

pub fn grid(r_levels: usize, g_levels: usize, b_levels: usize) -> Vec<Srgb8> {
    let mut palette = Vec::with_capacity(r_levels * g_levels * b_levels);
    for r_idx in 0..r_levels {
        let r = (r_idx * 255 + (r_levels - 1) / 2) / (r_levels - 1);
        for g_idx in 0..g_levels {
            let g = (g_idx * 255 + (g_levels - 1) / 2) / (g_levels - 1);
            for b_idx in 0..b_levels {
                let b = (b_idx * 255 + (b_levels - 1) / 2) / (b_levels - 1);
                palette.push(Srgb8 { data: [r as u8, g as u8, b as u8] });
            }
        }
    }
    palette
}

pub const RGBI: [Srgb8; 16] = [
    image::Rgb { data: [0x00,0x00,0x00] },
    image::Rgb { data: [0xff,0x00,0x00] },
    image::Rgb { data: [0x00,0xff,0x00] },
    image::Rgb { data: [0xff,0xff,0x00] },
    image::Rgb { data: [0x00,0x00,0xff] },
    image::Rgb { data: [0xff,0x00,0xff] },
    image::Rgb { data: [0x00,0xff,0xff] },
    image::Rgb { data: [0xff,0xff,0xff] },
    image::Rgb { data: [0x55,0x55,0x55] },
    image::Rgb { data: [0xaa,0x55,0x55] },
    image::Rgb { data: [0x55,0xaa,0x55] },
    image::Rgb { data: [0xaa,0xaa,0x55] },
    image::Rgb { data: [0x55,0x55,0xaa] },
    image::Rgb { data: [0xaa,0x55,0xaa] },
    image::Rgb { data: [0x55,0xaa,0xaa] },
    image::Rgb { data: [0xaa,0xaa,0xaa] },
];

pub const MICROSOFT16: [Srgb8; 16] = [
    image::Rgb { data: [0x00,0x00,0x00] },
    image::Rgb { data: [0x80,0x00,0x00] },
    image::Rgb { data: [0x00,0x80,0x00] },
    image::Rgb { data: [0x80,0x80,0x00] },
    image::Rgb { data: [0x00,0x00,0x80] },
    image::Rgb { data: [0x80,0x00,0x80] },
    image::Rgb { data: [0x00,0x80,0x80] },
    image::Rgb { data: [0xc0,0xc0,0xc0] },
    image::Rgb { data: [0x80,0x80,0x80] },
    image::Rgb { data: [0xff,0x00,0x00] },
    image::Rgb { data: [0x00,0xff,0x00] },
    image::Rgb { data: [0xff,0xff,0x00] },
    image::Rgb { data: [0x00,0x00,0xff] },
    image::Rgb { data: [0xff,0x00,0xff] },
    image::Rgb { data: [0x00,0xff,0xff] },
    image::Rgb { data: [0xff,0xff,0xff] },
];

pub const MACINTOSH16: [Srgb8; 16] = [
    image::Rgb { data: [0xff,0xff,0xff] },
    image::Rgb { data: [0xfb,0xf3,0x05] },
    image::Rgb { data: [0xff,0x64,0x03] },
    image::Rgb { data: [0xdd,0x09,0x07] },
    image::Rgb { data: [0xf2,0x08,0x84] },
    image::Rgb { data: [0x47,0x00,0xa5] },
    image::Rgb { data: [0x00,0x00,0xd3] },
    image::Rgb { data: [0x02,0xab,0xea] },
    image::Rgb { data: [0x1f,0xb7,0x14] },
    image::Rgb { data: [0x00,0x64,0x12] },
    image::Rgb { data: [0x56,0x2c,0x05] },
    image::Rgb { data: [0x90,0x71,0x3a] },
    image::Rgb { data: [0xc0,0xc0,0xc0] },
    image::Rgb { data: [0x80,0x80,0x80] },
    image::Rgb { data: [0x40,0x40,0x40] },
    image::Rgb { data: [0x00,0x00,0x00] },
];

// Taken from https://bisqwit.iki.fi/story/howto/dither/jy/
// Used to demonstrate the Yliluoma dithering algorithms
pub const YLILUOMA_EXAMPLE: [Srgb8; 16] = [
    image::Rgb { data: [0x08,0x00,0x00] },
    image::Rgb { data: [0x23,0x43,0x09] },
    image::Rgb { data: [0x2b,0x34,0x7c] },
    image::Rgb { data: [0x6a,0x94,0xab] },
    image::Rgb { data: [0x20,0x1a,0x0b] },
    image::Rgb { data: [0x5d,0x4f,0x1e] },
    image::Rgb { data: [0x2b,0x74,0x09] },
    image::Rgb { data: [0xd5,0xc4,0xb3] },
    image::Rgb { data: [0x43,0x28,0x17] },
    image::Rgb { data: [0x9c,0x6b,0x20] },
    image::Rgb { data: [0xd0,0xca,0x40] },
    image::Rgb { data: [0xfc,0xe7,0x6e] },
    image::Rgb { data: [0x49,0x29,0x10] },
    image::Rgb { data: [0xa9,0x22,0x0f] },
    image::Rgb { data: [0xe8,0xa0,0x77] },
    image::Rgb { data: [0xfc,0xfa,0xe2] },
];

// A palette selected to work well for the example used in the
// description of Yliluoma's dithering algorithms, making sure to bound
// all the colors.
pub const YLILUOMA_EXAMPLE_ALTERNATE: [Srgb8; 16] = [
    image::Rgb { data: [0x00,0x00,0x00] },
    image::Rgb { data: [0xff,0x00,0x00] },
    image::Rgb { data: [0x00,0xff,0x00] },
    image::Rgb { data: [0xff,0xff,0x00] },
    image::Rgb { data: [0x00,0x00,0xff] },
    image::Rgb { data: [0xff,0x00,0xff] },
    image::Rgb { data: [0x00,0xff,0xff] },
    image::Rgb { data: [0xff,0xff,0xff] },
    image::Rgb { data: [0x22,0x1d,0x12] },
    image::Rgb { data: [0xd4,0xe7,0xdc] },
    image::Rgb { data: [0x53,0x63,0x57] },
    image::Rgb { data: [0x6d,0x40,0x1a] },
    image::Rgb { data: [0x96,0x76,0x32] },
    image::Rgb { data: [0xaf,0xb8,0x6c] },
    image::Rgb { data: [0x74,0xa5,0xbc] },
    image::Rgb { data: [0x93,0x88,0x5f] },
];


pub fn make_simplex_palette(palette_size: usize, pixels: impl Iterator<Item=image::Rgb<u8>>, distance2: fn(Lab, Lab) -> f64) -> Vec<image::Rgb<u8>> {
    struct SimplexCut {
        vertices_rgb: [Srgb8; 4],
        vertices_lin: [LinearRgb; 4],
        vertices_lab: [Lab; 4],
        diameter2: f64,
        diameter_edge: [usize; 2],
        points: Vec<[f64; 4]>
    }

    impl SimplexCut {
        fn weight(&self) -> f64 {
            self.diameter2 * self.points.len() as f64
        }

        fn optimize(&mut self, referenced_points: &mut std::collections::HashMap<image::Rgb<u8>, usize>, distance2: fn(Lab, Lab) -> f64) {
            let mut changed = false;
            for opt_vertex in 0..4 {
                for other_vertex in (0..4).filter(|&v| v != opt_vertex) {
                    let mut max_ratio = 0.0;
                    for coords in &self.points {
                        let ratio = coords[opt_vertex] / coords[other_vertex];
                        if ratio > max_ratio {
                            max_ratio = ratio;
                        }
                    }
                    if max_ratio < 1e15 {
                        let factor = 1.0 - 1.0 / (max_ratio + 1.0);
                        //eprintln!("  Optimization factor {}", factor);
                        let new_lin = subtract(self.vertices_lin[opt_vertex], self.vertices_lin[other_vertex]) * factor + self.vertices_lin[other_vertex];
                        let new_rgb = Srgb8::from(new_lin);
                        if new_rgb != self.vertices_rgb[opt_vertex] &&
                           (referenced_points.get(&self.vertices_rgb[opt_vertex]) == Some(&1) || referenced_points.contains_key(&new_rgb)) {
                            changed = true;
                            let new_lin = LinearRgb::from(new_rgb);
                            let new_lab = Lab::from(new_lin);
                            match referenced_points.entry(self.vertices_rgb[opt_vertex]) {
                                std::collections::hash_map::Entry::Occupied(mut occ_entry) => {
                                    *occ_entry.get_mut() -= 1;
                                    if *occ_entry.get() == 0 {
                                        occ_entry.remove();
                                    }
                                },
                                _ => unreachable!()
                            }
                            self.vertices_rgb[opt_vertex] = new_rgb;
                            self.vertices_lin[opt_vertex] = new_lin;
                            self.vertices_lab[opt_vertex] = new_lab;
                            *referenced_points.entry(new_rgb).or_insert(0) += 1;
                            for coords in &mut self.points {
                                if factor < 1e-15 {
                                    coords[opt_vertex] += coords[other_vertex];
                                    coords[other_vertex] = 0.0;
                                } else {
                                    coords[opt_vertex] /= factor;
                                    coords[other_vertex] -= coords[opt_vertex] * (1.0 - factor);
                                }
                            }
                        }
                    }
                }
            }
            if changed {
                let mut max_distance_seen = 0.0;
                let mut max_distance_edge = [0, 1];
                for &edge in &[[0, 1], [0, 2], [0, 3], [1, 2], [1, 3], [2, 3]] {
                    let dist2 = distance2(self.vertices_lab[edge[0]], self.vertices_lab[edge[1]]);
                    if dist2 > max_distance_seen {
                        max_distance_seen = dist2;
                        max_distance_edge = edge;
                    }
                }
                self.diameter2 = max_distance_seen;
                self.diameter_edge = max_distance_edge;
            }
        }
    }

    impl std::cmp::PartialEq for SimplexCut {
        fn eq(&self, other: &SimplexCut) -> bool {
            self.weight() == other.weight()
        }
    }
    impl std::cmp::Eq for SimplexCut { }
    impl std::cmp::PartialOrd for SimplexCut {
        fn partial_cmp(&self, other: &SimplexCut) -> Option<std::cmp::Ordering> {
            self.weight().partial_cmp(&other.weight())
        }
    }
    impl std::cmp::Ord for SimplexCut {
        fn cmp(&self, other: &SimplexCut) -> std::cmp::Ordering {
            self.weight().partial_cmp(&other.weight()).unwrap()
        }
    }

    let mut palette = Vec::with_capacity(palette_size);

    let mut hue_split_points = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];

    for pixel in pixels {
        if pixel.data[0] < pixel.data[1] {
            if pixel.data[1] < pixel.data[2] {
                // Cyan-Blue
                hue_split_points[3].push(pixel);
            } else if pixel.data[2] < pixel.data[0] {
                // Yellow-Green
                hue_split_points[1].push(pixel);
            } else {
                // Cyan-Green
                hue_split_points[2].push(pixel);
            }
        } else {
            if pixel.data[0] < pixel.data[2] {
                // Magenta-Blue
                hue_split_points[4].push(pixel);
            } else if pixel.data[2] < pixel.data[1] {
                // Yellow-Red
                hue_split_points[0].push(pixel);
            } else {
                // Magenta-Red
                hue_split_points[5].push(pixel);
            }
        }
    }

    let mut nodes = std::collections::BinaryHeap::new();
    let mut referenced_points = std::collections::HashMap::new();

    let black_rgb = image::Rgb { data: [0, 0, 0] };
    let white_rgb = image::Rgb { data: [255, 255, 255] };
    let black_lin = LinearRgb::from(black_rgb);
    let white_lin = LinearRgb::from(white_rgb);
    let black_lab = Lab::from(black_lin);
    let white_lab = Lab::from(white_lin);

    for hue_idx in 0..6 {
        let cube_corners = [
            image::Rgb { data: [255, 0, 0] },
            image::Rgb { data: [255, 255, 0] },
            image::Rgb { data: [0, 255, 0] },
            image::Rgb { data: [0, 255, 255] },
            image::Rgb { data: [0, 0, 255] },
            image::Rgb { data: [255, 0, 255] },
            image::Rgb { data: [255, 0, 0] },
        ];

        let prev_rgb = cube_corners[hue_idx];
        let next_rgb = cube_corners[hue_idx + 1];
        let prev_lin = LinearRgb::from(prev_rgb);
        let next_lin = LinearRgb::from(next_rgb);
        let prev_lab = Lab::from(prev_lin);
        let next_lab = Lab::from(next_lin);

        let node = SimplexCut {
            vertices_rgb: [black_rgb, white_rgb, prev_rgb, next_rgb],
            vertices_lin: [black_lin, white_lin, prev_lin, next_lin],
            vertices_lab: [black_lab, white_lab, prev_lab, next_lab],
            diameter2: 10000.0,
            diameter_edge: [0, 1],
            points: hue_split_points[hue_idx].iter().map(|&rgb| {
                let lin = LinearRgb::from(rgb);
                let shifted_points = [
                    subtract(black_lin, lin),
                    subtract(white_lin, lin),
                    subtract(prev_lin, lin),
                    subtract(next_lin, lin),
                ];
                // Tag with barycentric coords
                let d0 = determinant([shifted_points[1], shifted_points[3], shifted_points[2]]);
                let d1 = determinant([shifted_points[0], shifted_points[2], shifted_points[3]]);
                let d2 = determinant([shifted_points[0], shifted_points[3], shifted_points[1]]);
                let d3 = determinant([shifted_points[0], shifted_points[1], shifted_points[2]]);
                let d_all = d0 + d1 + d2 + d3;

                [d0 / d_all, d1 / d_all, d2 / d_all, d3 / d_all]
            }).collect()
        };
        if !node.points.is_empty() {
            for &vertex in &node.vertices_rgb {
                *referenced_points.entry(vertex).or_insert(0) += 1;
            }
            nodes.push(node);
        }
    }

    while !nodes.is_empty() && referenced_points.len() < palette_size {
        let mut split_node = nodes.pop().unwrap();
        eprintln!("{}, {}, {}, {}", nodes.len(), referenced_points.len(), split_node.points.len(), split_node.diameter2);
        //eprintln!("  #{:02x}{:02x}{:02x}, #{:02x}{:02x}{:02x}, #{:02x}{:02x}{:02x}, #{:02x}{:02x}{:02x}",
        //    split_node.vertices_rgb[0].data[0], split_node.vertices_rgb[0].data[1], split_node.vertices_rgb[0].data[2],
        //    split_node.vertices_rgb[1].data[0], split_node.vertices_rgb[1].data[1], split_node.vertices_rgb[1].data[2],
        //    split_node.vertices_rgb[2].data[0], split_node.vertices_rgb[2].data[1], split_node.vertices_rgb[2].data[2],
        //    split_node.vertices_rgb[3].data[0], split_node.vertices_rgb[3].data[1], split_node.vertices_rgb[3].data[2],
        //);
        split_node.optimize(&mut referenced_points, distance2);
        //eprintln!("  #{:02x}{:02x}{:02x}, #{:02x}{:02x}{:02x}, #{:02x}{:02x}{:02x}, #{:02x}{:02x}{:02x}",
        //    split_node.vertices_rgb[0].data[0], split_node.vertices_rgb[0].data[1], split_node.vertices_rgb[0].data[2],
        //    split_node.vertices_rgb[1].data[0], split_node.vertices_rgb[1].data[1], split_node.vertices_rgb[1].data[2],
        //    split_node.vertices_rgb[2].data[0], split_node.vertices_rgb[2].data[1], split_node.vertices_rgb[2].data[2],
        //    split_node.vertices_rgb[3].data[0], split_node.vertices_rgb[3].data[1], split_node.vertices_rgb[3].data[2],
        //);
        //eprintln!("  Diameter edge: {}, {}", split_node.diameter_edge[0], split_node.diameter_edge[1]);
       

        if split_node.points.len() == 1 {
            continue;
        }
        let end0 = split_node.vertices_rgb[split_node.diameter_edge[0]];
        let end1 = split_node.vertices_rgb[split_node.diameter_edge[1]];
        let split_vertex_lin = midpoint(split_node.vertices_lin[split_node.diameter_edge[0]], split_node.vertices_lin[split_node.diameter_edge[1]]);
        let split_vertex_rgb = Srgb8::from(split_vertex_lin);
        //eprintln!("  Split at #{:02x}{:02x}{:02x}", split_vertex_rgb.data[0], split_vertex_rgb.data[1], split_vertex_rgb.data[2]);
        if split_vertex_rgb == end0 || split_vertex_rgb == end1 {
            continue;
        }
        let split_vertex_lin = LinearRgb::from(split_vertex_rgb);
        let split_vertex_lab = Lab::from(split_vertex_lin);
        let other0 = (0..4).find(|&x| x != split_node.diameter_edge[0] && x != split_node.diameter_edge[1]).unwrap();
        let other1 = (0..4).rfind(|&x| x != split_node.diameter_edge[1] && x != split_node.diameter_edge[0]).unwrap();

        let mut split_points = [Vec::new(), Vec::new()];
        for coords in split_node.points {
            if coords[split_node.diameter_edge[0]] > coords[split_node.diameter_edge[1]] {
                split_points[0].push([
                    coords[split_node.diameter_edge[0]] - coords[split_node.diameter_edge[1]],
                    coords[split_node.diameter_edge[1]] * 2.0,
                    coords[other0],
                    coords[other1]
                ]);
            } else {
                split_points[1].push([
                    coords[split_node.diameter_edge[1]] - coords[split_node.diameter_edge[0]],
                    coords[split_node.diameter_edge[0]] * 2.0,
                    coords[other0],
                    coords[other1]
                ]);
            }
        }

        let new_vertices_rgb = [
            [end0, split_vertex_rgb, split_node.vertices_rgb[other0], split_node.vertices_rgb[other1]],
            [end1, split_vertex_rgb, split_node.vertices_rgb[other0], split_node.vertices_rgb[other1]],
        ];
        let new_vertices_lin = [
            [split_node.vertices_lin[split_node.diameter_edge[0]], split_vertex_lin, split_node.vertices_lin[other0], split_node.vertices_lin[other1]],
            [split_node.vertices_lin[split_node.diameter_edge[1]], split_vertex_lin, split_node.vertices_lin[other0], split_node.vertices_lin[other1]],
        ];
        let new_vertices_lab = [
            [split_node.vertices_lab[split_node.diameter_edge[0]], split_vertex_lab, split_node.vertices_lab[other0], split_node.vertices_lab[other1]],
            [split_node.vertices_lab[split_node.diameter_edge[1]], split_vertex_lab, split_node.vertices_lab[other0], split_node.vertices_lab[other1]],
        ];

        for i in 0..2 {
            let mut max_distance_seen = 0.0;
            let mut max_distance_edge = [0, 1];
            for &edge in &[[0, 1], [0, 2], [0, 3], [1, 2], [1, 3], [2, 3]] {
                let dist2 = distance2(new_vertices_lab[i][edge[0]], new_vertices_lab[i][edge[1]]);
                if dist2 > max_distance_seen {
                    max_distance_seen = dist2;
                    max_distance_edge = edge;
                }
            }

            let node = SimplexCut {
                vertices_rgb: new_vertices_rgb[i],
                vertices_lin: new_vertices_lin[i],
                vertices_lab: new_vertices_lab[i],
                diameter2: max_distance_seen,
                diameter_edge: max_distance_edge,
                points: split_points[i].clone()
            };
            if !node.points.is_empty() {
                for &vertex in &node.vertices_rgb {
                    *referenced_points.entry(vertex).or_insert(0) += 1;
                }
                nodes.push(node);
            }
        }

        for &vertex in &split_node.vertices_rgb {
            match referenced_points.entry(vertex) {
                std::collections::hash_map::Entry::Occupied(mut occ_entry) => {
                    *occ_entry.get_mut() -= 1;
                    if *occ_entry.get() == 0 {
                        occ_entry.remove();
                    }
                },
                _ => unreachable!()
            }
        }
    }

    while let Some(mut node) = nodes.pop() {
        node.optimize(&mut referenced_points, distance2);
    }

    for &color in referenced_points.keys() {
        palette.push(color);
    }

    palette
}

#[derive(Copy, Clone)]
pub enum Split {
    Half,
    Median,
    Mean
}

pub fn make_box_palette(palette_size: usize, pixels: impl Iterator<Item=image::Rgb<u8>>, split: Split, optim: bool) -> Vec<image::Rgb<u8>> {
    let first_node = OctreeNode {
        bounding_box: [[0, 255], [0, 255], [0, 255]],
        pixels: pixels.collect()
    };

    let mut refs = std::collections::HashMap::new();
    for corner in first_node.corners() {
        refs.insert(corner, 1);
    }

    struct OctreeNode {
        bounding_box: [[u8; 2]; 3],
        pixels: Vec<image::Rgb<u8>>
    }

    impl std::cmp::PartialEq for OctreeNode {
        fn eq(&self, other: &OctreeNode) -> bool {
            self.pixels.len() == other.pixels.len()
        }
    }
    impl std::cmp::Eq for OctreeNode { }
    impl std::cmp::PartialOrd for OctreeNode {
        fn partial_cmp(&self, other: &OctreeNode) -> Option<std::cmp::Ordering> {
            self.pixels.len().partial_cmp(&other.pixels.len())
        }
    }
    impl std::cmp::Ord for OctreeNode {
        fn cmp(&self, other: &OctreeNode) -> std::cmp::Ordering {
            self.pixels.len().cmp(&other.pixels.len())
        }
    }

    impl OctreeNode {
        fn corners<'a>(&'a self) -> impl Iterator<Item=image::Rgb<u8>> + 'a {
            self.bounding_box[0].iter().flat_map(move |&r|
                self.bounding_box[1].iter().flat_map(move |&g|
                    self.bounding_box[2].iter().map(move |&b|
                        image::Rgb { data: [r, g, b] }
                    )
                )
            )
        }

        fn optimize(&mut self, corn_refs: &mut std::collections::HashMap<image::Rgb<u8>, usize>) {
            for axis in 0..3 {
                let other_axes = [[1,2],[0,2],[0,1]][axis];
                'face_loop:
                for &side in &[0,1] {
                    let fixed_value = self.bounding_box[axis][side];

                    for &other_side0 in &[0,1] {
                        for &other_side1 in &[0,1] {
                            let mut data = [0,0,0];
                            data[axis] = fixed_value;
                            data[other_axes[0]] = self.bounding_box[other_axes[0]][other_side0];
                            data[other_axes[1]] = self.bounding_box[other_axes[1]][other_side1];
                            if corn_refs.get(&image::Rgb { data: data }).cloned() != Some(1) {
                                continue 'face_loop;
                            }
                        }
                    }

                    let furthest = if side == 0 {
                        self.pixels.iter().map(|px| px.data[axis]).min().unwrap_or(self.bounding_box[axis][side])
                    } else {
                        self.pixels.iter().map(|px| px.data[axis]).max().unwrap_or(self.bounding_box[axis][side])
                    };

                    if furthest != fixed_value {
                        self.bounding_box[axis][side] = furthest;
                        for &other_side0 in &[0,1] {
                            for &other_side1 in &[0,1] {
                                let mut data = [0,0,0];
                                data[axis] = fixed_value;
                                data[other_axes[0]] = self.bounding_box[other_axes[0]][other_side0];
                                data[other_axes[1]] = self.bounding_box[other_axes[1]][other_side1];
                                corn_refs.remove(&image::Rgb { data: data });
                                data[axis] = furthest;
                                *corn_refs.entry(image::Rgb { data: data }).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    let mut nodes = std::collections::BinaryHeap::new();
    nodes.push(first_node);

    while refs.len() + 4 <= palette_size {
        let mut split_node = if let Some(node) = nodes.pop() {
            node
        } else {
            break;
        };
        if optim {
            split_node.optimize(&mut refs);
        }

        let mut axis = 0;
        let mut range = split_node.bounding_box[axis][1] - split_node.bounding_box[axis][0];
        if split_node.bounding_box[1][1] - split_node.bounding_box[1][0] > range {
            axis = 1;
            range = split_node.bounding_box[axis][1] - split_node.bounding_box[axis][0];
        }
        if split_node.bounding_box[2][1] - split_node.bounding_box[2][0] > range {
            axis = 2;
            range = split_node.bounding_box[axis][1] - split_node.bounding_box[axis][0];
        }
        if range <= 1 {
            continue;
        };
        let mid = match split {
            Split::Half => split_node.bounding_box[axis][0] + range / 2,
            Split::Median => {
                split_node.pixels.sort_by(|&p1, &p2| p1.data[axis].cmp(&p2.data[axis]));
                let small_count = split_node.pixels.iter().take_while(|&p| p.data[axis] == split_node.bounding_box[axis][0]).count();
                let large_count = split_node.pixels.iter().rev().take_while(|&p| p.data[axis] == split_node.bounding_box[axis][1]).count();
                if small_count + large_count == split_node.pixels.len() {
                    split_node.bounding_box[axis][0] + range / 2
                } else {
                    split_node.pixels[small_count + (split_node.pixels.len() - small_count - large_count) / 2].data[axis]
                }
            },
            Split::Mean => {
                split_node.pixels.sort_by(|&p1, &p2| p1.data[axis].cmp(&p2.data[axis]));
                let total = split_node.pixels.iter().map(|&p| color::srgb_decode_channel(p.data[axis])).sum::<f64>();
                let mean = color::srgb_encode_channel(total / split_node.pixels.len() as f64);
                if mean <= split_node.bounding_box[axis][0] || mean >= split_node.bounding_box[axis][1] {
                    split_node.bounding_box[axis][0] + range / 2
                } else {
                    mean
                }
            }
        };

        let mut low_bounding_box = split_node.bounding_box;
        let mut high_bounding_box = split_node.bounding_box;
        low_bounding_box[axis][1] = mid;
        high_bounding_box[axis][0] = mid;

        for &bounding_box in &[low_bounding_box, high_bounding_box] {
            let node = OctreeNode {
                bounding_box: bounding_box,
                pixels: split_node.pixels.iter().filter(|&&p| (0..3).all(|channel| {
                    bounding_box[channel][0] <= p.data[channel] && p.data[channel] <= bounding_box[channel][1]
                })).cloned().collect()
            };

            if node.pixels.iter().any(|&p| p.data[axis] != mid) {
                for corner in node.corners() {
                    *refs.entry(corner).or_insert(0) += 1;
                }
                nodes.push(node);
            }
        }

        for corner in split_node.corners() {
            match refs.entry(corner) {
                std::collections::hash_map::Entry::Occupied(mut occ_entry) => {
                    *occ_entry.get_mut() -= 1;
                    if *occ_entry.get() == 0 {
                        occ_entry.remove();
                    }
                },
                _ => unreachable!()
            }
        }
    }

    if optim {
        let mut vec_nodes = nodes.into_vec();
        for node in &mut vec_nodes {
            node.optimize(&mut refs);
        }
        nodes = vec_nodes.into_iter().collect();
    }

    while refs.len() < palette_size {
        if let Some(node) = nodes.pop() {
            refs.insert(image::Rgb { data: [
                node.bounding_box[0][0] + (node.bounding_box[0][1] - node.bounding_box[0][0]) / 2,
                node.bounding_box[1][0] + (node.bounding_box[1][1] - node.bounding_box[1][0]) / 2,
                node.bounding_box[2][0] + (node.bounding_box[2][1] - node.bounding_box[2][0]) / 2,
            ] }, 1);
        } else {
            break;
        }
    }

    refs.keys().cloned().collect()
}
