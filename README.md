# cv-fundamentals

Computer vision in Rust. From pixels to features.

---

## What This Does

`cv-fundamentals` implements the fundamental algorithms taught in a first course on computer vision:

| Module | What it covers |
|---|---|
| `image` | Grayscale image type with pixel ops, histograms, Otsu thresholding, convolution |
| `filter` | Gaussian/box blur, sharpen, Sobel/Prewitt gradients, Canny edge detection, Laplacian |
| `morphology` | Erosion, dilation, opening, closing, gradient, top-hat, black-hat with configurable structuring elements |
| `feature` | Harris corner detection, Laplacian-of-Gaussian blob detection |
| `segmentation` | Thresholding (binary, range, adaptive), connected-component labelling, watershed |
| `transform` | Hough line detection, affine image transforms with bilinear interpolation |
| `stereo` | Stereo disparity via block matching (SSD), triangulation, depth maps, point clouds |
| `optical_flow` | Lucas-Kanade dense optical flow |

**67 tests · zero unsafe · serde-serializable**

---

## Install

```toml
[dependencies]
cv-fundamentals = "0.1"
```

```bash
cargo add cv-fundamentals
```

### Dependencies

| Crate | Purpose |
|---|---|
| `nalgebra` 0.33 | Vectors and matrices (stereo rig, 3D triangulation) |
| `serde` (derive) | Serialisable image data |

---

## Quick Start

```rust
use cv_fundamentals::{GrayImage, ImageError};
use cv_fundamentals::filter;
use cv_fundamentals::feature;
use cv_fundamentals::segmentation;

// Create an image
let mut img = GrayImage::new(100, 100);
for y in 0..100 {
    for x in 0..50 {
        img.set(x, y, 200.0).unwrap(); // left half bright
    }
}

// Gaussian blur
let blurred = filter::gaussian_blur(&img, 1.5);

// Sobel edge detection
let (magnitude, direction, gx, gy) = filter::sobel(&blurred);

// Canny edges
let edges = filter::canny(&img, 20.0, 60.0);

// Harris corners
let corners = feature::harris_corners(&img, 0.04, 1.0, 3);

// Otsu automatic threshold
let thresh = img.otsu_threshold();
let binary = segmentation::threshold(&img, thresh);

// Connected components
let components = segmentation::connected_components(&binary);
for comp in &components {
    println!("Component {}: area={}, centroid=({:.1}, {:.1})",
        comp.label, comp.area, comp.centroid.0, comp.centroid.1);
}
```

---

## API Reference

### `image` — Grayscale image type

```rust
// Construction
let img = GrayImage::new(width, height);
let img = GrayImage::from_vec(w, h, vec![...f64])?;
let img = GrayImage::from_u8(w, h, &bytes)?;

// Pixel access
img.get(x, y) -> Result<f64, ImageError>
img.get_padded(x: isize, y: isize) -> f64      // zero-padded boundary
img.get_clamped(x: isize, y: isize) -> f64     // clamp-to-edge boundary
img.set(x, y, val) -> Result<(), ImageError>

// Statistics
img.histogram() -> [usize; 256]
img.histogram_normalized() -> [f64; 256]
img.mean() -> f64
img.std_dev() -> f64
img.otsu_threshold() -> f64

// Transforms
img.map(|v| v * 2.0) -> GrayImage
img.zip_with(&other, |a, b| a + b) -> Result<GrayImage, ImageError>
img.convolve(&kernel) -> Result<GrayImage, ImageError>
img.convolve_separable(&h_kernel, &v_kernel) -> Result<GrayImage, ImageError>
```

### `filter` — Convolution filters

```rust
// Blur
filter::gaussian_blur(&img, sigma) -> GrayImage
filter::box_blur(&img, size) -> GrayImage

// Edge detection
filter::sobel(&img) -> (magnitude, direction, gx, gy)  // all GrayImages
filter::canny(&img, low_threshold, high_threshold) -> GrayImage
filter::laplacian(&img) -> GrayImage
filter::sharpen(&img) -> GrayImage

// Raw kernels
filter::gaussian_kernel(sigma, size) -> Vec<Vec<f64>>
filter::sobel_gx() / sobel_gy() -> Vec<Vec<f64>>
filter::prewitt_gx() / prewitt_gy() -> Vec<Vec<f64>>
filter::laplacian_kernel() -> Vec<Vec<f64>>
filter::sharpen_kernel() -> Vec<Vec<f64>>
```

### `morphology` — Shape operations

```rust
// Structuring elements
let se = StructuringElement::square(3);
let se = StructuringElement::cross(5);

// Operations
morphology::erode(&img, &se) -> GrayImage
morphology::dilate(&img, &se) -> GrayImage
morphology::open(&img, &se) -> GrayImage      // erode then dilate
morphology::close(&img, &se) -> GrayImage     // dilate then erode
morphology::gradient(&img, &se) -> GrayImage  // dilate - erode
morphology::top_hat(&img, &se) -> GrayImage   // image - opening
morphology::black_hat(&img, &se) -> GrayImage // closing - image
```

### `feature` — Corner and blob detection

```rust
// Harris corners
let response = feature::harris_response(&img, k);
let corners = feature::harris_corners(&img, k, threshold, nms_radius) -> Vec<FeaturePoint>;

// Blob detection (multi-scale LoG)
let blobs = feature::detect_blobs(&img, min_sigma, max_sigma, num_scales, threshold) -> Vec<FeaturePoint>;

// FeaturePoint fields: x, y, response (strength), scale
```

### `segmentation` — Region-based image partitioning

```rust
// Thresholding
segmentation::threshold(&img, t) -> GrayImage
segmentation::threshold_range(&img, low, high) -> GrayImage
segmentation::threshold_inverse(&img, t) -> GrayImage
segmentation::adaptive_threshold(&img, block_size, c) -> GrayImage

// Connected components (4-connected, union-find)
let components = segmentation::connected_components(&binary_img) -> Vec<Component>;
// Component fields: label, pixels, area, centroid (f64,f64), bounding_box

// Watershed
segmentation::watershed(&markers, &gradient) -> GrayImage
```

### `transform` — Geometric image transforms

```rust
// Hough line detection
let mut detector = HoughLineDetector::new(width, height, theta_bins, rho_bins);
detector.detect(&edge_image);
let lines = detector.peaks(threshold, nms_radius) -> Vec<HoughLine>;
// HoughLine fields: rho, theta, votes

// Affine transforms (2×3 matrix, inverse mapping, bilinear interpolation)
let result = transform::affine_transform(&img, &matrix);
transform::rotation_matrix(angle, cx, cy) -> [[f64; 3]; 2]
transform::translation_matrix(tx, ty) -> [[f64; 3]; 2]
transform::scale_matrix(sx, sy, cx, cy) -> [[f64; 3]; 2]
```

### `stereo` — Stereo depth perception

```rust
let rig = StereoRig::new(baseline, focal_length, cx, cy);

// Disparity via block matching (SSD)
let disparity = rig.compute_disparity(&left, &right, max_disparity, block_size);

// Triangulate 3D point
let point = rig.triangulate(x_left, y_left, disparity) -> Option<Vector3<f64>>;

// Depth map
let depth = rig.disparity_to_depth(&disparity);

// Point cloud
let points = reproject_to_3d(&rig, &disparity) -> Vec<Option<Vector3<f64>>>;

// Camera intrinsics
let k = rig.intrinsic_matrix() -> Matrix3<f64>;
```

### `optical_flow` — Motion estimation

```rust
// Lucas-Kanade dense flow (magnitude image)
let flow_mag = optical_flow::lucas_kanade(&prev, &curr, window_size) -> GrayImage;

// With separate u/v components
let (flow_u, flow_v) = optical_flow::lucas_kanade_uv(&prev, &curr, window_size);

// Flow vectors
let flow = optical_flow::dense_flow(&prev, &curr, window_size) -> Vec<Vec<Option<FlowVector>>>;
// FlowVector { u: f64, v: f64 }
```

---

## Testing

```bash
cargo test
```

---

## License

MIT OR Apache-2.0
