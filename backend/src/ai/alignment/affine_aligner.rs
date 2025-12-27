//! Affine Face Aligner
//!
//! Align faces using affine transformation based on eye positions.
//! License: MIT (built-in)

use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

use crate::ai::error::{AiError, AiResult};
use crate::ai::traits::FaceAligner;
use crate::models::{FaceDetection, FaceLandmarks};

/// Standard output size for aligned faces (112x112 is common for face recognition).
pub const DEFAULT_OUTPUT_SIZE: (u32, u32) = (112, 112);

/// Reference landmarks for a 112x112 aligned face.
/// These are the target positions for the 5-point landmarks.
const REFERENCE_LANDMARKS: [(f32, f32); 5] = [
    (38.2946, 51.6963),  // Left eye
    (73.5318, 51.5014),  // Right eye
    (56.0252, 71.7366),  // Nose
    (41.5493, 92.3655),  // Left mouth
    (70.7299, 92.2041),  // Right mouth
];

/// Affine face aligner using similarity transform.
pub struct AffineAligner {
    /// Output image width.
    output_width: u32,
    /// Output image height.
    output_height: u32,
}

impl AffineAligner {
    /// Create a new affine aligner with default output size.
    pub fn new() -> Self {
        Self {
            output_width: DEFAULT_OUTPUT_SIZE.0,
            output_height: DEFAULT_OUTPUT_SIZE.1,
        }
    }

    /// Create with custom output size.
    pub fn with_output_size(width: u32, height: u32) -> Self {
        Self {
            output_width: width,
            output_height: height,
        }
    }

    /// Compute similarity transform matrix from source to target landmarks.
    ///
    /// Returns the transformation parameters (scale, rotation, translation).
    fn compute_similarity_transform(
        src_landmarks: &[(f32, f32); 5],
        dst_landmarks: &[(f32, f32); 5],
    ) -> SimilarityTransform {
        // Use eye positions for primary alignment
        let (src_left_eye, src_right_eye) = (src_landmarks[0], src_landmarks[1]);
        let (dst_left_eye, dst_right_eye) = (dst_landmarks[0], dst_landmarks[1]);

        // Compute source and destination eye vectors
        let src_dx = src_right_eye.0 - src_left_eye.0;
        let src_dy = src_right_eye.1 - src_left_eye.1;
        let dst_dx = dst_right_eye.0 - dst_left_eye.0;
        let dst_dy = dst_right_eye.1 - dst_left_eye.1;

        // Compute scale
        let src_dist = (src_dx * src_dx + src_dy * src_dy).sqrt();
        let dst_dist = (dst_dx * dst_dx + dst_dy * dst_dy).sqrt();
        let scale = if src_dist > 0.0 {
            dst_dist / src_dist
        } else {
            1.0
        };

        // Compute rotation
        let src_angle = src_dy.atan2(src_dx);
        let dst_angle = dst_dy.atan2(dst_dx);
        let rotation = dst_angle - src_angle;

        // Compute source center (between eyes)
        let src_center_x = (src_left_eye.0 + src_right_eye.0) / 2.0;
        let src_center_y = (src_left_eye.1 + src_right_eye.1) / 2.0;

        // Compute destination center
        let dst_center_x = (dst_left_eye.0 + dst_right_eye.0) / 2.0;
        let dst_center_y = (dst_left_eye.1 + dst_right_eye.1) / 2.0;

        // Compute translation
        let cos_r = rotation.cos();
        let sin_r = rotation.sin();
        let tx = dst_center_x - scale * (cos_r * src_center_x - sin_r * src_center_y);
        let ty = dst_center_y - scale * (sin_r * src_center_x + cos_r * src_center_y);

        SimilarityTransform {
            scale,
            rotation,
            tx,
            ty,
        }
    }

    /// Apply the transformation to get source coordinates for a destination point.
    fn inverse_transform(transform: &SimilarityTransform, dst_x: f32, dst_y: f32) -> (f32, f32) {
        let cos_r = transform.rotation.cos();
        let sin_r = transform.rotation.sin();

        // Invert the transformation
        let x_shifted = dst_x - transform.tx;
        let y_shifted = dst_y - transform.ty;

        let src_x = (cos_r * x_shifted + sin_r * y_shifted) / transform.scale;
        let src_y = (-sin_r * x_shifted + cos_r * y_shifted) / transform.scale;

        (src_x, src_y)
    }

    /// Bilinear interpolation to sample from source image.
    fn sample_bilinear(image: &DynamicImage, x: f32, y: f32) -> Rgba<u8> {
        let (width, height) = image.dimensions();

        // Clamp coordinates
        let x = x.max(0.0).min((width - 1) as f32);
        let y = y.max(0.0).min((height - 1) as f32);

        let x0 = x.floor() as u32;
        let y0 = y.floor() as u32;
        let x1 = (x0 + 1).min(width - 1);
        let y1 = (y0 + 1).min(height - 1);

        let fx = x - x.floor();
        let fy = y - y.floor();

        let p00 = image.get_pixel(x0, y0);
        let p01 = image.get_pixel(x0, y1);
        let p10 = image.get_pixel(x1, y0);
        let p11 = image.get_pixel(x1, y1);

        let mut result = [0u8; 4];
        for i in 0..4 {
            let v00 = p00[i] as f32;
            let v01 = p01[i] as f32;
            let v10 = p10[i] as f32;
            let v11 = p11[i] as f32;

            let v = v00 * (1.0 - fx) * (1.0 - fy)
                + v10 * fx * (1.0 - fy)
                + v01 * (1.0 - fx) * fy
                + v11 * fx * fy;

            result[i] = v.round().clamp(0.0, 255.0) as u8;
        }

        Rgba(result)
    }

    /// Scale reference landmarks to current output size.
    fn get_scaled_reference(&self) -> [(f32, f32); 5] {
        let scale_x = self.output_width as f32 / 112.0;
        let scale_y = self.output_height as f32 / 112.0;

        let mut scaled = REFERENCE_LANDMARKS;
        for point in &mut scaled {
            point.0 *= scale_x;
            point.1 *= scale_y;
        }
        scaled
    }
}

impl Default for AffineAligner {
    fn default() -> Self {
        Self::new()
    }
}

/// Similarity transformation parameters.
struct SimilarityTransform {
    scale: f32,
    rotation: f32,
    tx: f32,
    ty: f32,
}

impl FaceAligner for AffineAligner {
    fn id(&self) -> &str {
        "affine"
    }

    fn output_size(&self) -> (u32, u32) {
        (self.output_width, self.output_height)
    }

    fn align(&self, image: &DynamicImage, landmarks: &FaceLandmarks) -> AiResult<DynamicImage> {
        // Get 5-point landmarks
        let src_landmarks = landmarks.to_5_points().ok_or_else(|| {
            AiError::AlignmentFailed("Missing required 5-point landmarks".to_string())
        })?;

        // Get reference landmarks scaled to output size
        let dst_landmarks = self.get_scaled_reference();

        // Compute transformation
        let transform = Self::compute_similarity_transform(&src_landmarks, &dst_landmarks);

        // Create output image
        let mut output: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::new(self.output_width, self.output_height);

        // Apply inverse transformation for each output pixel
        for y in 0..self.output_height {
            for x in 0..self.output_width {
                let (src_x, src_y) = Self::inverse_transform(&transform, x as f32, y as f32);
                let pixel = Self::sample_bilinear(image, src_x, src_y);
                output.put_pixel(x, y, pixel);
            }
        }

        Ok(DynamicImage::ImageRgba8(output))
    }

    fn align_from_bbox(&self, image: &DynamicImage, detection: &FaceDetection) -> AiResult<DynamicImage> {
        // If we have landmarks, use them
        if let Some(ref landmarks) = detection.landmarks {
            return self.align(image, landmarks);
        }

        // Otherwise, just crop and resize based on bounding box
        let bbox = &detection.bounding_box;

        // Add some padding around the face
        let padding = 0.3;
        let pad_x = bbox.width * padding;
        let pad_y = bbox.height * padding;

        let x = (bbox.x - pad_x).max(0.0) as u32;
        let y = (bbox.y - pad_y).max(0.0) as u32;
        let w = (bbox.width + 2.0 * pad_x) as u32;
        let h = (bbox.height + 2.0 * pad_y) as u32;

        // Ensure we don't go out of bounds
        let (img_w, img_h) = image.dimensions();
        let x = x.min(img_w.saturating_sub(1));
        let y = y.min(img_h.saturating_sub(1));
        let w = w.min(img_w - x);
        let h = h.min(img_h - y);

        // Crop the face region
        let cropped = image.crop_imm(x, y, w, h);

        // Resize to output size
        let resized = cropped.resize_exact(
            self.output_width,
            self.output_height,
            image::imageops::FilterType::Lanczos3,
        );

        Ok(resized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbImage;

    fn create_test_image() -> DynamicImage {
        let img = RgbImage::from_fn(200, 200, |x, y| {
            image::Rgb([
                ((x + y) % 256) as u8,
                ((x * 2) % 256) as u8,
                ((y * 2) % 256) as u8,
            ])
        });
        DynamicImage::ImageRgb8(img)
    }

    #[test]
    fn test_output_size() {
        let aligner = AffineAligner::new();
        assert_eq!(aligner.output_size(), (112, 112));

        let aligner = AffineAligner::with_output_size(224, 224);
        assert_eq!(aligner.output_size(), (224, 224));
    }

    #[test]
    fn test_align_with_landmarks() {
        let aligner = AffineAligner::new();
        let image = create_test_image();

        let landmarks = FaceLandmarks::from_5_points([
            (50.0, 60.0),   // Left eye
            (150.0, 60.0),  // Right eye
            (100.0, 100.0), // Nose
            (60.0, 150.0),  // Left mouth
            (140.0, 150.0), // Right mouth
        ]);

        let aligned = aligner.align(&image, &landmarks).unwrap();
        let (w, h) = aligned.dimensions();

        assert_eq!(w, 112);
        assert_eq!(h, 112);
    }

    #[test]
    fn test_align_from_bbox() {
        let aligner = AffineAligner::new();
        let image = create_test_image();

        use crate::models::BoundingBox;
        let detection = FaceDetection::new(
            BoundingBox::new(30.0, 30.0, 140.0, 140.0),
            0.95,
            "test",
        );

        let aligned = aligner.align_from_bbox(&image, &detection).unwrap();
        let (w, h) = aligned.dimensions();

        assert_eq!(w, 112);
        assert_eq!(h, 112);
    }
}
