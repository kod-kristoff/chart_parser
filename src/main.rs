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

    println!("grammar rules: {}", grammar);
    
    for i in 0..10 {
        println!("example({}) = {:?}", i, parser::example(i));
    }
    let sent1: Vec<&'static str> = parser::example(3);

    let chart = parser::earley1(&grammar, &sent1);
    println!("chart = {:?}", chart);
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
        pub rhs: &'a Vec<String>,
        pub dot: usize,
    }
    #[derive(Debug)]
    pub struct Chart<'a> {
        pub chart: Vec<Vec<Edge<'a>>>,
        _empty_vec: Vec<String>,
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

    pub fn earley1<'a>(grammar: &'a Grammar, input: &'a Vec<&'static str>) -> Chart<'a> {
        let mut result = Chart {
            chart: Vec::new(),
            _empty_vec: Vec::new()
        };
        let mut chart: Vec<HashSet<Edge>> = Vec::new();

        for (k, word) in input.iter().enumerate() {
            println!("word {}: {}", k, word);
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
                rhs: &result._empty_vec,
                dot: 0,
            });
            while agenda.len() > 0 {
                println!("agenda = {:?}", agenda);
                let edge = match agenda.pop() {
                    Some(edge) => edge,
                    None => panic!("no edge")
                };
                
                if !edgeset.contains(&edge) {

                    if edge.is_passive() {
                        println!("found passive edge.");

                        // Predict
                        for rule in &grammar.rules {
                            if edge.lhs == rule.rhs[0] {
                                agenda.push(
                                    Edge {
                                        start: edge.start,
                                        end: k,
                                        lhs: &rule.lhs,
                                        rhs: &rule.rhs,
                                        dot: 1,
                                })
                            } // if
                        } // for

                        // Complete
                        for e in &chart[edge.start] {
                            println!("edge: {:?}", e);
                            if !e.is_passive() && edge.lhs == e.rhs[e.dot] {
                                println!("complete");
                            }
                        }
                    } // if edge.is_passive
                    edgeset.insert(edge);
                } // if !edgeset.contains
                
            }
            chart.push(edgeset);
            println!("chart: {:?}", chart);
        }
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

    impl fmt::Display for Rule {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} --> {:?}", self.lhs, self.rhs)
        }
    }

    impl fmt::Display for Grammar {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self.rules)
        }
    }

    impl Edge<'_> {
        pub fn is_passive(&self) -> bool {
            self.dot == self.rhs.len()
        }
    }
}
