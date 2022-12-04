use std::collections::BTreeSet;
use std::f64;

use crate::{intersection::intersection, point::Point};

// A struct to represent a line segment in 2D space.
#[derive(Clone, Copy, Debug)]
pub struct LineSegment {
    pub p1: Point,
    pub p2: Point,
}

// A struct to represent a polygon as a list of points.
#[derive(Clone, Debug)]
pub struct Polygon {
    pub points: Vec<Point>,
}

impl Polygon {
    // Computes the edges of the polygon as a list of line segments.
    fn edges(&self) -> Vec<LineSegment> {
        let mut edges = vec![];
        for i in 0..self.points.len() {
            let j = (i + 1) % self.points.len();
            edges.push(LineSegment {
                p1: self.points[i],
                p2: self.points[j],
            });
        }
        edges
    }

    // Computes the bounding box of the polygon.
    pub fn bounding_box(&self) -> (f64, f64, f64, f64) {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for point in &self.points {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }
        (min_x, min_y, max_x, max_y)
    }

    // Computes the union of this polygon with another polygon using a
    // scanline algorithm.
    pub fn union(&self, other: &Polygon) -> Polygon {
        // Create a set of points to store the result of the union.
        let mut result = BTreeSet::new();

        // Add the points of both polygons to the set of points.
        for point in &self.points {
            result.insert(*point);
        }
        for point in &other.points {
            result.insert(*point);
        }

        // Create a vector of all the points in the set, sorted by y-coordinate.
        let mut points: Vec<Point> = result.iter().copied().collect();
        points.sort_by(|point1, point2| point1.y.partial_cmp(&point2.y).unwrap());

        let mut y = f64::NEG_INFINITY;
        // Iterate over the scanlines in the bounding box.
        for point in points {
            // Check if the y-coordinate of the point is different from the previous point.
            if point.y != y {
                // If it is, we have reached a new scanline.
                y = point.y;

                // Create a set of points to store the intersections with the scanline.
                let mut intersections = BTreeSet::new();

                // Compute the intersections of the scanline with the edges of the first polygon.
                for edge in self.edges() {
                    // Check if the scanline intersects the edge.
                    let (x, _) = intersection(edge, y);
                    if x.is_finite() {
                        // Add the intersection point to the set of intersections.
                        intersections.insert(Point { x, y });
                    }
                }

                // Compute the intersections of the scanline with the edges of the second polygon.
                for edge in other.edges() {
                    // Check if the scanline intersects the edge.
                    let (x, _) = intersection(edge, y);
                    if x.is_finite() {
                        // Add the intersection point to the set of intersections.
                        intersections.insert(Point { x, y });
                    }
                }

                // Add the intersections to the result.
                for intersection in intersections {
                    result.insert(intersection);
                }
            }
        }

        // Return the result as a polygon.
        Polygon {
            points: result.into_iter().collect(),
        }
    }

    // Computes the difference between two polygons.
    fn difference(&self, other: &Polygon) -> Polygon {
        // Create a set of points to store the result of the difference.
        let mut result = BTreeSet::new();

        // Add the points of the first polygon to the set of points.
        for point in &self.points {
            result.insert(point);
        }

        // Create a vector of all the points in the set, sorted by y-coordinate.
        let mut points: Vec<&Point> = result.iter().copied().collect();
        points.sort_by(|point1, point2| point1.y.partial_cmp(&point2.y).unwrap());

        // Iterate over the points in the vector.
        let mut y = f64::NEG_INFINITY;
        let mut inside = false;
        for point in points {
            // Check if the y-coordinate of the point is different from the previous point.
            if point.y != y {
                // If it is, we have reached a new scanline.
                y = point.y;

                // Compute the intersections of the scanline with the edges of the second polygon.
                let mut intersections = BTreeSet::new();
                for edge in other.edges() {
                    // Check if the scanline intersects the edge.
                    let (x, _) = intersection(edge, y);
                    if x.is_finite() {
                        // Add the intersection point to the set of intersections.
                        intersections.insert(Point { x, y });
                    }
                }

                // Update the inside/outside state based on the intersections.
                if intersections.len() % 2 == 0 {
                    inside = !inside;
                }
            }

            // Check if the point is inside the second polygon.
            if inside {
                // If it is, remove it from the result.
                result.remove(point);
            }
        }

        // Convert the result set to a vector of points.
        let mut points: Vec<Point> = result.into_iter().copied().collect();

        // Sort the points in counter-clockwise order.
        let centroid = self.centroid();
        points.sort_by(|point1, point2| {
            (*point2 - centroid)
                .cross(*point1 - centroid)
                .partial_cmp(&0.0)
                .unwrap()
        });

        // Return the result as a polygon.
        Polygon { points }
    }

    // Computes the difference between two polygons.
    fn fixed_difference(&self, other: &Polygon) -> Vec<Polygon> {
        // Create a set of points to store the result of the difference.
        let mut result = BTreeSet::new();

        // Add the points of both polygons to the set of points.
        for point in &self.points {
            result.insert(point);
        }
        for point in &other.points {
            result.insert(point);
        }

        // Create a vector of all the points in the set, sorted by y-coordinate.
        let mut points: Vec<&Point> = result.iter().copied().collect();
        points.sort_by(|point1, point2| point1.y.partial_cmp(&point2.y).unwrap());

        // Create a vector to store the result polygons.
        let mut polygons = Vec::new();

        // Iterate over the points in the vector.
        let mut y = f64::NEG_INFINITY;
        let mut inside = false;
        for point in points {
            // Check if the y-coordinate of the point is different from the previous point.
            if point.y != y {
                // If it is, we have reached a new scanline.
                y = point.y;

                // Compute the intersections of the scanline with the edges of the second polygon.
                let mut intersections = BTreeSet::new();
                for edge in other.edges() {
                    // Check if the scanline intersects the edge.
                    let (x, _) = intersection(edge, y);
                    if x.is_finite() {
                        // Add the intersection point to the set of intersections.
                        intersections.insert(Point { x, y });
                    }
                }

                // Update the inside/outside state based on the intersections.
                if intersections.len() % 2 == 0 {
                    inside = !inside;
                }

                // Check if the point is inside the second polygon.
                if inside {
                    // If it is, remove it from the result.
                    result.remove(point);
                } else if !intersections.is_empty() {
                    // If it is not, and there are intersections at this scanline,
                    // create a new polygon from the points in the result.
                    let mut points: Vec<Point> = result.iter().map(|point| **point).collect();

                    // Sort the points in counter-clockwise order.
                    let centroid = self.centroid();
                    points.sort_by(|point1, point2| {
                        (*point2 - centroid)
                            .cross(*point1 - centroid)
                            .partial_cmp(&0.0)
                            .unwrap()
                    });

                    // Add the new polygon to the result vector.
                    polygons.push(Polygon { points });

                    // Clear the result set for the next polygon.
                    result.clear();
                }
            }

            // If there are remaining points in the result set,
            // create one more polygon from the points.
            if !result.is_empty() {
                let mut points: Vec<Point> = result.iter().map(|point| **point).collect();

                // Sort the points in counter-clockwise order.
                let centroid = self.centroid();
                points.sort_by(|point1, point2| {
                    (*point2 - centroid)
                        .cross(*point1 - centroid)
                        .partial_cmp(&0.0)
                        .unwrap()
                });

                // Add the new polygon to the result vector.
                polygons.push(Polygon { points });
            }
        }
        // Return the result vector of polygons.
        polygons
    }

    pub fn centroid(&self) -> Point {
        // Compute the sum of the x- and y-coordinates of the points.
        let mut x_sum = 0.0;
        let mut y_sum = 0.0;
        for point in &self.points {
            x_sum += point.x;
            y_sum += point.y;
        }

        // Compute the centroid as the average of the coordinates.
        let n = self.points.len() as f64;
        Point {
            x: x_sum / n,
            y: y_sum / n,
        }
    }

    pub fn convex_hull(&self) -> Polygon {
        // Create a vector of all the points in the polygon.
        let mut points: Vec<Point> = self.points.to_vec();

        // Sort the points by x-coordinate.
        points.sort_by(|point1, point2| point1.y.partial_cmp(&point2.y).unwrap());

        // Create a vector to store the result points.
        let mut result: Vec<Point> = Vec::new();

        // Compute the lower hull.
        for point in &points {
            while result.len() >= 2
                && (result[result.len() - 2] - result[result.len() - 1])
                    .cross(*point - result[result.len() - 1])
                    <= 0.0
            {
                result.pop();
            }
            result.push(*point);
        }

        // Compute the upper hull.
        let n = result.len() + 1;
        for point in points.iter().rev() {
            while result.len() >= n
                && (result[result.len() - 2] - result[result.len() - 1])
                    .cross(*point - result[result.len() - 1])
                    <= 0.0
            {
                result.pop();
            }
            result.push(*point);
        }

        // Return the result as a polygon.
        Polygon { points: result }
    }
}
