//! Gesture detection for stylus input
//!
//! Detects common gestures like circles, underlines, and lasso selections.

use crate::stroke::Stroke;
use log::debug;

/// Detected gestures
#[derive(Debug, Clone, PartialEq)]
pub enum Gesture {
    /// Circle gesture - used to trigger AI response
    Circle {
        center_x: i32,
        center_y: i32,
        radius: i32,
    },
    /// Underline gesture - emphasize text or select region
    Underline {
        start_x: i32,
        end_x: i32,
        y: i32,
    },
    /// Lasso gesture - free-form selection
    Lasso {
        bounds: (i32, i32, i32, i32), // (x_min, y_min, x_max, y_max)
    },
    /// Not a recognized gesture
    None,
}

/// Detects gestures from completed strokes
pub struct GestureDetector {
    /// Minimum points required for gesture detection
    min_points: usize,
    /// Tolerance for circle detection (deviation from perfect circle)
    circle_tolerance: f32,
}

impl GestureDetector {
    /// Create a new gesture detector
    pub fn new() -> Self {
        Self {
            min_points: 10,
            circle_tolerance: 0.3,
        }
    }

    /// Detect gesture from a stroke
    pub fn detect(&self, stroke: &Stroke) -> Gesture {
        if stroke.points.len() < self.min_points {
            return Gesture::None;
        }

        // Try to detect a circle first (most common gesture)
        if let Some(circle) = self.detect_circle(stroke) {
            return circle;
        }

        // Try to detect an underline
        if let Some(underline) = self.detect_underline(stroke) {
            return underline;
        }

        // Try to detect a lasso (closed shape)
        if let Some(lasso) = self.detect_lasso(stroke) {
            return lasso;
        }

        Gesture::None
    }

    /// Detect if stroke is a circle
    fn detect_circle(&self, stroke: &Stroke) -> Option<Gesture> {
        // TODO: Implement circle detection algorithm
        // 1. Find centroid of points
        // 2. Calculate average distance from centroid (radius)
        // 3. Check if all points are within tolerance of that radius
        // 4. Check if stroke closes back near the start

        let points = &stroke.points;
        if points.is_empty() {
            return None;
        }

        // Calculate centroid
        let sum_x: i32 = points.iter().map(|p| p.x).sum();
        let sum_y: i32 = points.iter().map(|p| p.y).sum();
        let center_x = sum_x / points.len() as i32;
        let center_y = sum_y / points.len() as i32;

        // Calculate average radius
        let sum_dist: f32 = points
            .iter()
            .map(|p| {
                let dx = (p.x - center_x) as f32;
                let dy = (p.y - center_y) as f32;
                (dx * dx + dy * dy).sqrt()
            })
            .sum();
        let avg_radius = sum_dist / points.len() as f32;

        // Check if stroke closes (start and end are close)
        let first = &points[0];
        let last = &points[points.len() - 1];
        let dx = (last.x - first.x) as f32;
        let dy = (last.y - first.y) as f32;
        let closure_dist = (dx * dx + dy * dy).sqrt();

        // Must close within 20% of radius
        if closure_dist > avg_radius * 0.2 {
            return None;
        }

        // Check deviation from perfect circle
        let deviations: Vec<f32> = points
            .iter()
            .map(|p| {
                let dx = (p.x - center_x) as f32;
                let dy = (p.y - center_y) as f32;
                let dist = (dx * dx + dy * dy).sqrt();
                ((dist - avg_radius) / avg_radius).abs()
            })
            .collect();

        let avg_deviation: f32 = deviations.iter().sum::<f32>() / deviations.len() as f32;

        if avg_deviation < self.circle_tolerance {
            debug!(
                "Circle detected at ({}, {}) radius={}",
                center_x, center_y, avg_radius as i32
            );
            Some(Gesture::Circle {
                center_x,
                center_y,
                radius: avg_radius as i32,
            })
        } else {
            None
        }
    }

    /// Detect if stroke is an underline
    fn detect_underline(&self, stroke: &Stroke) -> Option<Gesture> {
        // TODO: Implement underline detection
        // 1. Check if stroke is mostly horizontal
        // 2. Check if Y variance is low
        // 3. Return start/end X and average Y

        let points = &stroke.points;
        if points.is_empty() {
            return None;
        }

        // Calculate Y variance
        let avg_y: i32 = points.iter().map(|p| p.y).sum::<i32>() / points.len() as i32;
        let y_variance: f32 = points
            .iter()
            .map(|p| {
                let diff = (p.y - avg_y) as f32;
                diff * diff
            })
            .sum::<f32>()
            / points.len() as f32;

        // Calculate X range
        let min_x = points.iter().map(|p| p.x).min().unwrap();
        let max_x = points.iter().map(|p| p.x).max().unwrap();
        let x_range = (max_x - min_x) as f32;

        // Must be mostly horizontal (low Y variance relative to X range)
        if y_variance.sqrt() < x_range * 0.1 && x_range > 100.0 {
            debug!("Underline detected from {} to {} at y={}", min_x, max_x, avg_y);
            Some(Gesture::Underline {
                start_x: min_x,
                end_x: max_x,
                y: avg_y,
            })
        } else {
            None
        }
    }

    /// Detect if stroke is a lasso (closed shape)
    fn detect_lasso(&self, stroke: &Stroke) -> Option<Gesture> {
        // TODO: Implement lasso detection
        // 1. Check if stroke forms a closed shape
        // 2. Calculate bounding box
        // 3. Return bounds

        let points = &stroke.points;
        if points.is_empty() {
            return None;
        }

        // Check if stroke closes
        let first = &points[0];
        let last = &points[points.len() - 1];
        let dx = (last.x - first.x) as f32;
        let dy = (last.y - first.y) as f32;
        let closure_dist = (dx * dx + dy * dy).sqrt();

        // Must close within a reasonable distance
        if closure_dist > 50.0 {
            return None;
        }

        // Calculate bounds
        let x_min = points.iter().map(|p| p.x).min().unwrap();
        let y_min = points.iter().map(|p| p.y).min().unwrap();
        let x_max = points.iter().map(|p| p.x).max().unwrap();
        let y_max = points.iter().map(|p| p.y).max().unwrap();

        debug!(
            "Lasso detected: bounds=({}, {}, {}, {})",
            x_min, y_min, x_max, y_max
        );
        Some(Gesture::Lasso {
            bounds: (x_min, y_min, x_max, y_max),
        })
    }
}

impl Default for GestureDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stroke::Point;

    fn create_circle_stroke(center_x: i32, center_y: i32, radius: i32, num_points: usize) -> Stroke {
        let mut stroke = Stroke::new();
        for i in 0..num_points {
            let angle = 2.0 * std::f32::consts::PI * (i as f32) / (num_points as f32);
            let x = center_x + (radius as f32 * angle.cos()) as i32;
            let y = center_y + (radius as f32 * angle.sin()) as i32;
            stroke.add_point(Point {
                x,
                y,
                pressure: 1000,
                timestamp_ms: i as u64,
            });
        }
        stroke
    }

    #[test]
    fn test_circle_detection() {
        let detector = GestureDetector::new();
        let stroke = create_circle_stroke(500, 500, 100, 50);

        match detector.detect(&stroke) {
            Gesture::Circle { center_x, center_y, radius } => {
                assert!((center_x - 500).abs() < 10);
                assert!((center_y - 500).abs() < 10);
                assert!((radius - 100).abs() < 10);
            }
            _ => panic!("Expected circle gesture"),
        }
    }

    #[test]
    fn test_underline_detection() {
        let detector = GestureDetector::new();
        let mut stroke = Stroke::new();

        // Draw horizontal line
        for i in 0..30 {
            stroke.add_point(Point {
                x: 100 + i * 10,
                y: 500,
                pressure: 1000,
                timestamp_ms: i as u64,
            });
        }

        match detector.detect(&stroke) {
            Gesture::Underline { start_x, end_x, y } => {
                assert_eq!(start_x, 100);
                assert_eq!(end_x, 390);
                assert!((y - 500).abs() < 5);
            }
            _ => panic!("Expected underline gesture"),
        }
    }
}
