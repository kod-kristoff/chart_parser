fn main() {
    println!("Hello, world!");


    let rules = vec!(
        parser::Rule {
            lhs: String::from("S"),
            rhs: vec!(
                String::from("NP"),
                String::from("VP")
            ),
        },
        parser::Rule {
            lhs: String::from("VP"),
            rhs: vec!(String::from("Verb"),)
        },
        parser::Rule {
            lhs: String::from("VP"),
            rhs: vec!(String::from("Verb"), String::from("NP")) },
        parser::Rule { lhs: String::from("VP"),   rhs: vec!(String::from("VP"), String::from("PP")) },
        parser::Rule { lhs: String::from("NP"),   rhs: vec!(String::from("Det"), String::from("Noun")) },
        parser::Rule { lhs: String::from("NP"),   rhs: vec!(String::from("NP"), String::from("PP")) },
        parser::Rule { lhs: String::from("PP"),   rhs: vec!(String::from("Prep"), String::from("NP")) },
        parser::Rule { lhs: String::from("Verb"), rhs: vec!(String::from("sees"),) },
        parser::Rule { lhs: String::from("Det"),  rhs: vec!(String::from("the"),) },
        parser::Rule { lhs: String::from("Det"),  rhs: vec!(String::from("a"),) },
        parser::Rule { lhs: String::from("Prep"), rhs: vec!(String::from("under"),) },
        parser::Rule { lhs: String::from("Prep"), rhs: vec!(String::from("with"),) },
        parser::Rule { lhs: String::from("Prep"), rhs: vec!(String::from("in"),) },
        parser::Rule { lhs: String::from("Noun"), rhs: vec!(String::from("zebra"),) },
        parser::Rule { lhs: String::from("Noun"), rhs: vec!(String::from("lion"),) },
        parser::Rule { lhs: String::from("Noun"), rhs: vec!(String::from("tree"),) },
        parser::Rule { lhs: String::from("Noun"), rhs: vec!(String::from("park"),) },
        parser::Rule { lhs: String::from("Noun"), rhs: vec!(String::from("telescope"),) },
    );
    let grammar = parser::Grammar { rules };

    println!("grammar rules:");
    for rule in &grammar.rules {
        println!("{}", rule);
    }
    // println!("start rule: {}", grammar.rules[0]);

    // for i in 0..10 {
    //     println!("example({}) = {:?}", i, parser::example(i));
    // }
    let sent1: Vec<&'static str> = parser::example(3);

    println!("Parsing {} words: {:?}", sent1.len(), sent1);

    let chart = parser::earley1(&grammar, &sent1);
    // println!("chart = {:?}, ", chart);
    parser::print_chart(&chart);
    println!("Parsing succesful: {}", parser::success(&chart, "S", 0));
}

mod parser {
    use std::{
        collections::HashSet,
        fmt,
    };

    const EXAMPLE_PREFIX: [&'static str; 5] = [
        "the",
        "lion",
        "sees",
        "a",
        "zebra",
    ];
    const EXAMPLE_SUFFIX: [&'static str; 9] =  [
        "under",
        "a",
        "tree",
        "with",
        "a",
        "telescope",
        "in",
        "the",
        "park",
    ];

    #[derive(Debug)]
    pub struct Rule {
        pub lhs: String,
        pub rhs: Vec<String>,
    }

    pub struct Grammar {
        pub rules: Vec<Rule>,
    }


    #[derive(Debug, Eq, Hash, PartialEq)]
    pub struct Edge<'a> {
        pub start: usize,
        pub end: usize,
        pub lhs: &'a str,
        pub rhs: Vec<&'a str>,
        pub dot: usize,
    }
    #[derive(Debug)]
    pub struct Chart<'a> {
        pub chart: Vec<Vec<Edge<'a>>>,
    }

    pub fn example(n: usize) -> Vec<&'static str> {
        // let mut suffix = EXAMPLE_SUFFIX.iter();
        // for i in 0..(n/3) {
        //    suffix.chain(EXAMPLE_SUFFIX.iter());
        // }
        EXAMPLE_PREFIX.iter()
            .chain(EXAMPLE_SUFFIX.iter().take(n*3))
            .map(|x| *x).collect()
    }

    pub fn success(chart: &Chart, cat: &str, start: usize) -> bool {
        // println!("chart.chart.last() = {:?}", *chart.chart.last().unwrap());
        chart.chart.last().unwrap().iter().any(|edge| edge.start == start && edge.lhs == cat && edge.is_passive())
        // false
    }

    pub fn chartsize(chart: &Chart) -> usize {
        chart.chart.iter().map(|v| v.len()).sum()
    }
    pub fn print_chart(chart: &Chart) {
        println!("Chart size: {} edges", chartsize(chart));
        for (k, edgeset) in chart.chart.iter().enumerate() {
            if edgeset.len() > 0 {
                println!("{} edges ending in position {}:", edgeset.len(), k);
                for edge in edgeset {
                    println!("    {}", edge);
                }
            }
        }
    }
    pub fn earley1<'a>(grammar: &'a Grammar, input: &'a Vec<&'static str>) -> Chart<'a> {
        let mut result = Chart {
            chart: Vec::new(),
        };
        let mut chart: Vec<HashSet<Edge>> = vec!(HashSet::new());

        for (k, word) in input.iter().enumerate() {
            let k = k + 1;
            // println!("word {}: {}", k, word);
            let mut edgeset = HashSet::new();
            if k == 0 {
                chart.push(edgeset);
                continue;
            }
            // Scan
            let mut agenda = vec!(Edge {
                start: k-1,
                end: k,
                lhs: word,
                rhs: Vec::new(),
                dot: 0,
            });
            while agenda.len() > 0 {
                // println!("agenda = {:?}", agenda);
                let edge = match agenda.pop() {
                    Some(edge) => edge,
                    None => panic!("no edge")
                };
                // println!("edge = {:?}", edge);
                if !edgeset.contains(&edge) {

                    if edge.is_passive() {
                        // println!("found passive edge.");

                        // Predict
                        for rule in &grammar.rules {
                            if edge.lhs == rule.rhs[0] {
                                // println!("predict");
                                agenda.push(
                                    Edge {
                                        start: edge.start,
                                        end: k,
                                        lhs: &rule.lhs,
                                        rhs: rule.rhs.iter().map(String::as_str).collect(),
                                        dot: 1,
                                });
                            } // if
                        } // for

                        // Complete
                        for e in &chart[edge.start] {
                            // println!("edge e = {:?}", e);
                            if !e.is_passive() && edge.lhs == e.rhs[e.dot] {
                                // println!("complete");
                                agenda.push(
                                    Edge {
                                        start: e.start,
                                        end: k,
                                        lhs: e.lhs,
                                        rhs: e.rhs.iter().map(|x| *x).collect(),
                                        dot: e.dot + 1,
                                    }
                                );
                            }
                        }
                    } // if edge.is_passive
                    edgeset.insert(edge);
                } // if !edgeset.contains

            } // while agenda.len() > 0
            chart.push(edgeset);
            // println!("chart: {:?}", chart);
        } // for k, word in input
        for edgeset in chart {
            let mut part = Vec::new();
            for edge in edgeset {
                if edge.is_passive() {
                    part.push(edge);
                }
            }
            result.chart.push(part);
        }
        result
    }
    pub fn format_vec(vec: &Vec<&str>) -> String {
        vec.join(" ")
    }
    impl fmt::Display for Rule {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} --> {}", self.lhs, self.rhs.join(" "))
        }
    }

    impl fmt::Display for Grammar {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self.rules)
        }
    }

    impl fmt::Display for Edge<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "[{}-{}: {} --> {} . {:?}]",
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
    impl Edge<'_> {
        pub fn is_passive(&self) -> bool {
            self.dot == self.rhs.len()
        }
    }
}
