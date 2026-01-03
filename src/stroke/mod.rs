//! Stroke engine module
//!
//! Stroke collection, gesture recognition, and stroke management

/// A single point in a stroke
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub pressure: u16,
    pub timestamp_ms: u64,
}

/// A collection of points forming a stroke
#[derive(Debug, Clone)]
pub struct Stroke {
    pub points: Vec<Point>,
}

impl Stroke {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    /// Get the bounding box of this stroke
    pub fn bounding_box(&self) -> Option<(i32, i32, i32, i32)> {
        if self.points.is_empty() {
            return None;
        }

        let x_min = self.points.iter().map(|p| p.x).min().unwrap();
        let y_min = self.points.iter().map(|p| p.y).min().unwrap();
        let x_max = self.points.iter().map(|p| p.x).max().unwrap();
        let y_max = self.points.iter().map(|p| p.y).max().unwrap();

        Some((x_min, y_min, x_max, y_max))
    }

    /// Get the total length of this stroke (sum of distances between consecutive points)
    pub fn length(&self) -> f32 {
        if self.points.len() < 2 {
            return 0.0;
        }

        self.points
            .windows(2)
            .map(|w| {
                let dx = (w[1].x - w[0].x) as f32;
                let dy = (w[1].y - w[0].y) as f32;
                (dx * dx + dy * dy).sqrt()
            })
            .sum()
    }

    /// Get duration of stroke in milliseconds
    pub fn duration_ms(&self) -> u64 {
        if self.points.len() < 2 {
            return 0;
        }

        let first = self.points.first().unwrap();
        let last = self.points.last().unwrap();
        last.timestamp_ms - first.timestamp_ms
    }
}

impl Default for Stroke {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Add collector.rs for stroke aggregation
// TODO: Add gesture recognition
