use std::{
    cmp,
    fmt,
};

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Edge<'a> {
    start: usize,
    end: usize,
    lhs: &'a str,
    rhs: Vec<&'a str>,
    dot: usize,
}

impl<'a> Edge<'a> {
    pub fn new(start: usize, end: usize, lhs: &'a str, rhs: Option<&[&'a str]>, dot: usize) -> Self {
        Edge::<'a> {
            start: start,
            end: end,
            lhs: lhs,
            rhs: match rhs {
                None => Vec::new(),
                Some(vec) => vec.iter().map(|x| *x).collect()
            },
            dot: dot,
        }
    }
    pub fn is_passive(&self) -> bool {
        self.dot == self.rhs.len()
    }
}

impl fmt::Display for Edge<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}-{}: {} --> {} . {}]",
            self.start,
            self.end,
            // "lhs",
            self.lhs,
            // self.rhs,
            self.rhs[..self.dot].join(" "),
            self.rhs[self.dot..].join(" "),
        )
    }
}

impl Ord for Edge<'_> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        (self.start, self.end).cmp(&(other.start, other.end))
    }
}

impl PartialOrd for Edge<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_with_rhs_creates_edge() {
        let lhs = "S";
        let rhs = vec!("NP", "VP");
        let edge = Edge::new(0, 2, lhs, Some(&rhs), 1);

        assert_eq!(edge.start, 0);
        assert_eq!(edge.end, 2);
        assert_eq!(edge.lhs, lhs);
        assert_eq!(edge.rhs, rhs);
        assert_eq!(edge.dot, 1);
    }

    #[test]
    fn new_without_rhs_creates_edge() {
        let edge = Edge::new(0, 2, "S", None, 1);

        assert_eq!(edge.start, 0);
        assert_eq!(edge.end, 2);
        assert_eq!(edge.lhs, "S");
        assert_eq!(edge.rhs, Vec::<&str>::new());
        assert_eq!(edge.dot, 1);
    }

    #[test]
    fn format_edge_with_rhs_and_dot_less_than_rhs_len() {
        let edge = Edge::new(0, 2, "S", Some(&vec!("NP", "VP")), 1);

        assert_eq!(format!("{}", edge), "[0-2: S --> NP . VP]");
    }

    #[test]
    fn format_edge_with_rhs_and_dot_equal_to_rhs_len() {
        let edge = Edge::new(0, 2, "S", Some(&vec!("NP", "VP")), 2);

        assert_eq!(format!("{}", edge), "[0-2: S --> NP VP . ]");
    }

    #[test]
    fn format_edge_without_rhs() {
        let lhs = "S";
        let edge = Edge::new(0, 2, "S", None, 0);

        assert_eq!(format!("{}", edge), "[0-2: S -->  . ]");
    }

    #[test]
    fn edge_with_dot_less_than_rhs_len_is_not_passive() {
        let edge = Edge::new(0, 2, "S", Some(&vec!("NP", "VP")), 1);

        assert!(!edge.is_passive());
    }

    #[test]
    fn edge_with_dot_equal_to_rhs_len_is_passive() {
        let edge = Edge::new(0, 2, "S", Some(&vec!("NP", "VP")), 2);

        assert!(edge.is_passive());
    }

    #[test]
    fn edge_without_rhs_is_passive() {
        let edge = Edge::new(0, 2, "S", None, 0);

        assert!(edge.is_passive());    
    }

    #[test]
    fn test_cmp() {
        let e1 = Edge::new(0, 2, "D", None, 0);
        let e2 = Edge::new(1, 2, "D", None, 0);
        let e3 = Edge::new(1, 3, "D", None, 0);
        let e4 = Edge::new(0, 1, "D", None, 0);
        let e5 = Edge::new(0, 3, "D", None, 0);

        assert!(e1 < e2);
        assert!(e1 == e1);
        assert!(e1 < e3);
        assert!(e4 < e1);
        assert!(e1 < e5);
    }
}
