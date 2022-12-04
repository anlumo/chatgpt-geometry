use crate::polygon::LineSegment;

// Computes the intersection of a line segment and a scanline.
// Returns the intersection point as a tuple (x, y).
pub fn intersection(segment: LineSegment, y: f64) -> (f64, f64) {
    // Compute the coordinates of the two points of the line segment.
    let x1 = segment.p1.x;
    let y1 = segment.p1.y;
    let x2 = segment.p2.x;
    let y2 = segment.p2.y;

    // Check if the line segment is vertical (i.e., if the x-coordinates are the same).
    if x1 == x2 {
        // In this case, the line segment is vertical and the intersection point
        // is simply the x-coordinate of the line segment and the y-coordinate of the scanline.
        (x1, y)
    } else {
        // Compute the slope and the intercept of the line segment.
        let m = (y2 - y1) / (x2 - x1);
        let b = y1 - m * x1;

        // Compute the x-coordinate of the intersection point.
        let x = (y - b) / m;

        // Return the intersection point.
        (x, y)
    }
}
