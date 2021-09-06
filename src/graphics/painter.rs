/// The current number type used to represent colors.
pub type Color = u8;

/// Allows to draw shapes on the screen.
///
/// Many functions accept color as an option argument,
/// passing none will use the current active color.
pub trait GraphicsPainter {
    /// Gets the active painting color.
    fn get_color(&self) -> Color;
    /// Sets the active painting color.
    fn set_color(&mut self, color: Color);
    /// Clears the screen and fills it with a specific color.
    fn clear(&mut self, color: Option<Color>);
    /// Draws a point into the screen.
    fn point(&mut self, x: f64, y: f64, color: Option<Color>);
    /// Draws a line into the screen.
    fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, color: Option<Color>);
    /// Draws a triangle into the screen.
    fn triangle(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, filled: bool, color: Option<Color>);
    /// Draws a rectangle into the screen.
    fn rectangle(&mut self, x: f64, y: f64, width: f64, height: f64, fill: bool, color: Option<Color>);
    /// Draws a filled polygon into the screen.
    fn polygon(&mut self, vertices: &[(f64, f64)], color: Option<Color>);
    /// Draws a circle on the screen.
    fn circle(&mut self, cx: f32, cy: f64, radius: f64, filled: bool, color: Option<Color>);
    /// Draws an ellipse on the screen.
    fn ellipse(&mut self, x: f32, y: f32, radius_x: f32, radius_y: f32, filled: bool, color: Option<Color>);
}