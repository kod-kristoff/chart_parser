fn main() {
    use std::time::Instant;
    use std::fs::File;
    use std::io::{BufWriter, Write};
    use itertools::Itertools;

    let grammar = match parser::read_grammar_from_ron_file("grammar.ron") {
        Ok(grammar) => grammar,
        Err(e) => {
            println!("error reading 'grammar.ron': {}", e);
            println!("loading default ...");
            vec!(
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
            ) 
        }
    };
    // let grammar = parser::Grammar { rules };

//    let f = File::create("grammar.ron").expect("fail 1");
//    let mut f = BufWriter::new(f);
//    match ron::ser::to_writer(f, &grammar) {
//        Ok(_) => println!("serialize"),
//        Err(e) => println!("Error {}", e)
//    }

    println!("grammar rules:");
    for rule in &grammar {
        println!("{}", rule);
    }
    // println!("start rule: {}", grammar.rules[0]);

    for (lc, rules) in parser::leftcorners_dict(&grammar) {
        println!("{:10}: {}", lc, rules.iter().format("      "));
    }

    for i in 0..10 {
        println!("example({}) = {}", i, parser::example(i).join(" "));
    }
    let sent1: Vec<&'static str> = parser::example(3);

    // println!("Parsing {} words: {:?}", sent1.len(), sent1);

    // let chart = parser::earley1(&grammar, &sent1);
    // println!("chart = {:?}, ", chart);
    // parser::print_chart(&chart);
    // println!("Parsing succesful: {}", parser::success(&chart, "S", 0));
    parser::test(
        parser::earley1,
        &grammar,
        "S",
        &sent1,
        &[1,2,-2,-1],
    );
    parser::test(
        parser::earley1,
        &grammar,
        "S",
        &sent1[..6],
        &[1,2,3,4,5,6],
    );

    let now = Instant::now();
    parser::test(
        parser::earley1,
        &grammar,
        "S",
        &parser::example(3),
        &[-1],
    );
    println!("earley1, elapsed time: {:.6?}", now.elapsed());

    let now = Instant::now();
    parser::test(
        parser::earley2,
        &grammar,
        "S",
        &parser::example(3),
        &[-1],
    );
    println!("earley2, elapsed time: {:.6?}", now.elapsed());

    let now = Instant::now();
    parser::test(
        parser::earley2,
        &grammar,
        "S",
        &parser::example(3),
        &[-1],
    );
    println!("earley2, elapsed time: {:.6?}", now.elapsed());


}

mod parser {
    use std::{
        collections::{HashMap, HashSet},
        cmp,
        fmt,
        fs::File,
    };
    use ron::Result;
    use serde::{Deserialize, Serialize};

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

    pub struct Rule {
        pub lhs: String,
        pub rhs: Vec<String>,
    }


    pub fn read_grammar_from_ron_file(file_name: &str) -> ron::Result<Vec<Rule>> {
        let f = File::open(file_name)?;
        let grammar: ron::Result<Vec<Rule>> = ron::de::from_reader(f);
        grammar
    }

    pub fn example(n: usize) -> Vec<&'static str> {
        // let mut suffix = EXAMPLE_SUFFIX.iter();
        // for i in 0..(n/3) {
        //    suffix.chain(EXAMPLE_SUFFIX.iter());
        // }
        EXAMPLE_PREFIX.iter()
            .chain(EXAMPLE_SUFFIX.iter().cycle().take(n*3))
            .map(|x| *x).collect()
    }

    pub fn leftcorners_dict<'a>(grammar: &'a [Rule]) -> HashMap<&'a str, Vec<&Rule>> {
        let mut leftcorners = HashMap::new();
        for rule in grammar {
            let entry = leftcorners.entry(rule.rhs[0].as_str()).or_insert(Vec::new());
            entry.push(rule);
        }
        leftcorners
    }

    pub fn success(chart: &Chart, cat: &str, start: usize) -> bool {
        // println!("chart.chart.last() = {:?}", *chart.chart.last().unwrap());
        chart.chart.last().unwrap().iter().any(|edge| edge.start == start && edge.lhs == cat && edge.is_passive())
        // false
    }

    pub fn test<'a>(
        parser: impl Fn(&'a[Rule], &[&'a str]) -> Chart<'a>,
        grammar: &'a [Rule],
        cat: &str,
        sentence: &'a [&str],
        positions: &[i32],
        ) {
        let nwords = sentence.len();
        if nwords <= 15 {
        println!("Parsing {} words: {}", sentence.len(), sentence.join(" "));
        } else {
            println!(
                "Parsing {} words: {} ... {}",
                sentence.len(),
                sentence[..3].join(" "),
                sentence[(nwords-9)..].join(" "),
            );
        }
        let chart = parser(grammar, sentence);
        if success(&chart, cat, 0) {
            println!("Yay, success!!");
        } else {
            println!("Meh, failure :(");
        }
        print_chart(&chart, positions, None);
    }

    pub fn print_chart(chart: &Chart, positions: &[i32], cutoff: Option<usize>) {
        let cutoff: usize = cutoff.unwrap_or(8);
        println!("Chart size: {} edges", chartsize(chart));
        for (k, edgeset) in chart.chart.iter().enumerate() {
            if edgeset.len() > 0 && (positions.contains(&(k as i32)) || positions.contains(&(k as i32 - chart.chart.len() as i32))) {
                println!("{} edges ending in position {}:", edgeset.len(), k);
                let mut sorted_edgeset = edgeset.to_vec();
                sorted_edgeset.sort();
                for (n, edge) in sorted_edgeset.iter().enumerate() {
                    if cutoff > 0 && n >= cutoff {
                        println!("    ...");
                        break;
                    }
                    println!("    {}", edge);
                }
            }
        }
    }
    pub fn earley1<'a>(grammar: &'a [Rule], input: &[&'a str]) -> Chart<'a> {
        let mut result = Chart {
            chart: Vec::new(),
        };
        let mut chart: Vec<HashSet<Edge>> = vec!(HashSet::new());

        for (k, word) in input.iter().enumerate() {
            let k = k + 1;
            // println!("word {}: {}", k, word);
            let mut edgeset = HashSet::new();
            // if k == 0 {
            //     chart.push(edgeset);
            //     continue;
            // }
            // Scan
            let mut agenda = vec!(Edge::new(k-1, k, word, None, 0));
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
                        for rule in grammar {
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

    pub fn earley2<'a>(grammar: &'a [Rule], input: &[&'a str]) -> Chart<'a> {
        let leftcorners = leftcorners_dict(grammar);

        let mut chart: Vec<HashMap<Option<&str>, HashSet<Edge>>> = Vec::new();
        {
            let mut entry_0 = HashMap::new();
            entry_0.insert(None, HashSet::new());
            chart.push(entry_0);
        }

        for (k, sym) in input.iter().enumerate() {
            let k = k + 1;

            let mut lc_edgesets = HashMap::new();

            // Scan
            let mut agenda = vec!(Edge {
                start: k-1,
                end: k,
                lhs: sym,
                rhs: Vec::new(),
                dot: 0,
            });

            while agenda.len() > 0 {
                // println!("agenda = {:?}", agenda);

                let edge = match agenda.pop() {
                    Some(edge) => edge,
                    None => panic!("no edge")
                };

                let leftc = match edge.is_passive() {
                    true => None,
                    false => Some(edge.rhs[edge.dot])
                };
                let edgeset = lc_edgesets.entry(leftc).or_insert(HashSet::<Edge>::new());

                if !edgeset.contains(&edge) {
                    if edge.is_passive() {
                        // Predict
                        if leftcorners.contains_key(edge.lhs) {
                            let rules = &leftcorners[edge.lhs];
                            for rule in rules {
                                agenda.push(
                                    Edge {
                                        start: edge.start,
                                        end: k,
                                        lhs: &rule.lhs,
                                        rhs: rule.rhs.iter().map(String::as_str).collect(),
                                        dot: 1,
                                    }
                                );
                            }
                        }

                        // Complete
                        if chart[edge.start].contains_key(&Some(edge.lhs)) {
                            for e in &chart[edge.start][&Some(edge.lhs)] {
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
                    } // if edge is passive
                    edgeset.insert(edge);
                } // if edge not in edgeset
            } // while agenda
            chart.push(lc_edgesets);
        } // for input

        let mut result = Chart::new();
        for lc_edgeset in chart {
            let mut part = Vec::new();
            for edge in lc_edgeset.get(&None).unwrap() {
                part.push(edge.clone())
            }
            result.chart.push(part);
        }
        result
    }
    
    #[derive(Clone, Debug)]
    pub struct Tree {
        root: String,
        children: Vec<Tree>,
    }

    impl Tree {
        pub fn new(root: &str, children: Vec<Tree>) -> Self {
            Tree { 
                root: root.to_string(),  
                children: children 
            }
        }

        pub fn leaf(root: &str) -> Self {
            Tree {
                root: root.to_string(),
                children: Vec::new()
            }
        }
    }

    pub fn earley3<'a>(grammar: &'a [Rule], input: &[&'a str]) -> Chart<'a> {
        let leftcorners = leftcorners_dict(grammar);

        let mut chart: Vec<HashMap<Option<&str>, HashSet<Edge>>> = Vec::new();
        {
            let mut entry_0 = HashMap::new();
            entry_0.insert(None, HashSet::new());
            chart.push(entry_0);
        }

        for (k, sym) in input.iter().enumerate() {
            let k = k + 1;

            let mut lc_edgesets = HashMap::new();

            // Scan
            let mut agenda = vec!(Edge::with_result(
                k-1,
                k,
                sym,
                Vec::new(),
                0,
                )
            );

            while agenda.len() > 0 {
                // println!("agenda = {:?}", agenda);

                let edge = match agenda.pop() {
                    Some(edge) => edge,
                    None => panic!("no edge")
                };

                let leftc = match edge.is_passive() {
                    true => None,
                    false => Some(edge.rhs[edge.dot])
                };
                let edgeset = lc_edgesets.entry(leftc).or_insert(HashSet::<Edge>::new());

                if !edgeset.contains(&edge) {
                    if edge.is_passive() {
                        // Predict
                        if leftcorners.contains_key(edge.lhs) {
                            let rules = &leftcorners[edge.lhs];
                            for rule in rules {
                                agenda.push(
                                    Edge {
                                        start: edge.start,
                                        end: k,
                                        lhs: &rule.lhs,
                                        rhs: rule.rhs.iter().map(String::as_str).collect(),
                                        dot: 1,
                                    }
                                );
                            }
                        }

                        // Complete
                        if chart[edge.start].contains_key(&Some(edge.lhs)) {
                            for e in &chart[edge.start][&Some(edge.lhs)] {
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
                    } // if edge is passive
                    edgeset.insert(edge);
                } // if edge not in edgeset
            } // while agenda
            chart.push(lc_edgesets);
        } // for input

        let mut result = Chart::new();
        for lc_edgeset in chart {
            let mut part = Vec::new();
            for edge in lc_edgeset.get(&None).unwrap() {
                part.push(edge.clone())
            }
            result.chart.push(part);
        }
        result
    }
    // pub fn format_vec(vec: &Vec<&str>) -> String {
    //     vec.join(" ")
    // }

}
