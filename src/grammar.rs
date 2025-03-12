use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Grammar {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rule {
    lhs: String,
    rhs: Vec<String>,
}

impl Grammar {
    pub fn new() -> Self {
        Grammar { rules: Vec::new() }
    }

    pub fn from_rules(rules: Vec<Rule>) -> Self {
        Grammar { rules }
    }
}

impl Rule {
    pub fn new(lhs: &str, rhs: Vec<String>) -> Self {
        Rule {
            lhs: lhs.to_string(),
            rhs: rhs,
        }
    }
}

impl fmt::Display for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = write!(f, "grammar rules:");
        match result {
            Err(e) => return Err(e),
            _ => {}
        }
        for rule in &self.rules {
            result = write!(f, "\n  {}", rule);
        }
        //write!(f, "{:?}", self.rules)
        result
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} --> {}", self.lhs, self.rhs.join(" "))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_rule() {
        let rule = Rule::new("test", Vec::new());

        assert_eq!(rule.lhs, "test");
        assert_eq!(rule.rhs, Vec::<String>::new());
        assert_eq!(format!("{}", rule), "test --> ");
    }

    #[test]
    fn new_creates_empty_grammar() {
        let grammar = Grammar::new();

        assert_eq!(grammar.rules.len(), 0);
        assert_eq!(format!("{}", grammar), "grammar rules:");
    }

    #[test]
    fn from_rules_creates_grammar() {
        let rules = vec!(Rule::new("S", Vec::new()));
        let grammar = Grammar::from_rules(rules);

        assert_eq!(grammar.rules.len(), 1);
        assert_eq!(
            format!("{}", grammar),
            "grammar rules:\n  S --> "
        );
    }
} // mod tests
