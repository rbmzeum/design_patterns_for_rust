use crate::patterns::structural::bridge::abstract_chart::AbstractChart;

pub mod abstract_chart;
pub mod abstract_chart_imp;

pub mod chart;
pub mod chart_imp;

// Example
pub fn structural_bridge() {
    use chart::gtk4_drawing_area_chart::Gtk4DrawingAreaChart;
    let mut chart = Gtk4DrawingAreaChart::new();

    chart.init();
    chart.draw();
}