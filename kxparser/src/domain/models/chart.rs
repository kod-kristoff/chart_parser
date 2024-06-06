use crate::domain::models::edge::Edge;

#[derive(Debug)]
pub struct Chart<'a> {
    pub chart: Vec<Vec<Edge<'a>>>,
}

impl Chart<'_> {
    pub fn new() -> Self {
        Chart { chart: Vec::new() }
    }

    pub fn chartsize(&self) -> usize {
        self.chart.iter().map(|v| v.len()).sum()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_chart() {
        let chart = Chart::new();

        // assert_eq!(chart.chartsize(), 0);
        assert_eq!(chart.chart.len(), 0);
    }

    #[test]
    fn empty_chart_has_size_zero() {
        let chart = Chart::new();

        assert_eq!(chart.chartsize(), 0);
    }
}
