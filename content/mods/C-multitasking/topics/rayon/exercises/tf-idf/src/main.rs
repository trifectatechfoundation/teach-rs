use rayon::prelude::*;
use std::{cmp::Ordering, collections::HashMap};

// in this exercise we implement a basic version of tf-idf using rayon.

/// Some random books from Project Gutenberg
const DOCUMENTS: &[(&str, &str)] = &[
    ("Middlemarch", include_str!("../documents/pg145.txt")),
    (
        "THE TRAGEDY OF ROMEO AND JULIET",
        include_str!("../documents/pg1513.txt"),
    ),
    (
        "A Room With A View",
        include_str!("../documents/pg2641.txt"),
    ),
    (
        "The Enchanted April",
        include_str!("../documents/pg16389.txt"),
    ),
    ("Little Women", include_str!("../documents/pg37106.txt")),
];

/// For each word in the document, how often does it occur in the document
fn term_frequency(document: &str) -> HashMap<&str, usize> {
    // HINT: use the https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html#method.entry method, and
    // its API https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html, particular the `or_insert` function.
    document
        // good enough definition of "word" for this exercise
        .split_whitespace()
        // using fold to get some extra practice with monoids. Using a for loop is also totally fine.
        .fold(HashMap::default(), |mut hash_map, word| todo!())
}

fn combine_occurences<'a>(
    a: HashMap<&'a str, usize>,
    b: HashMap<&'a str, usize>,
) -> HashMap<&'a str, usize> {
    // combine the counts from maps a and b. If a word is in both maps, add up their counts,
    // otherwise just use the count from one of the maps.
    //
    // NOTE: we're already using all of our cores to process whole documents. Using a parallel iterator
    // here would likely make performance worse!
    b.into_iter().fold(a, |mut hash_map, (word, count)| todo!())
}

/// Map each word in the document to the value 1
fn term_occurence(document: &str) -> HashMap<&str, usize> {
    todo!()
}

/// For each word, in how many of the documents it occurs
fn document_frequency<'a>(
    documents: impl rayon::iter::ParallelIterator<Item = &'a str>,
) -> HashMap<&'a str, usize> {
    // map each document to a hashmap that maps words to whether they occur (use `term_occurence`),
    // then reduce, combining the counts.
    todo!()
}

fn score_document(
    query: &str,
    term_frequencies: &HashMap<&str, usize>,
    document_frequencies: &HashMap<&str, usize>,
) -> f64 {
    let n = document_frequencies.len() as f64;

    query
        .split_whitespace()
        .map(|word| {
            let tf = *term_frequencies.get(word).unwrap_or(&0) as f64;
            let idf = (n / (1.0 + *document_frequencies.get(word).unwrap_or(&0) as f64)).log10();

            tf * idf
        })
        .sum::<f64>()
}

#[derive(Debug)]
struct SearchResultQueue<'a> {
    results: Vec<(f64, &'a str)>,
    n_results: usize,
}

impl<'a> SearchResultQueue<'a> {
    fn new(n_results: usize) -> Self {
        Self {
            results: Vec::with_capacity(n_results),
            n_results,
        }
    }

    fn sort_and_truncate(&mut self) {
        // sort big to small
        self.results
            .sort_by(|(s1, _), (s2, _)| f64::total_cmp(s2, s1));
        self.results.truncate(self.n_results);
    }

    fn push(&mut self, score: f64, name: &'a str) {
        let empty_space = self.results.len() < self.n_results;
        let higher_score = matches!(self.results.get(0), Some((s2, _)) if f64::total_cmp(&score, s2) == Ordering::Greater);

        if empty_space || higher_score {
            self.results.push((score, name));

            self.sort_and_truncate();
        }
    }

    fn append(mut self, mut other: Self) -> Self {
        self.results.append(&mut other.results);
        self.sort_and_truncate();
        self
    }
}
fn search<'a>(
    query: &str,
    documents: &'a [(&'a str, &'a str)],
    n_results: usize,
) -> SearchResultQueue<'a> {
    let document_frequencies = document_frequency(documents.par_iter().map(|t| t.1));

    documents
        .par_iter()
        .fold(
            || SearchResultQueue::new(n_results),
            |mut state, (name, doc)| {
                let term_frequencies = term_frequency(doc);
                let score = score_document(query, &term_frequencies, &document_frequencies);

                state.push(score, name);

                state
            },
        )
        .reduce(
            || SearchResultQueue::new(n_results),
            SearchResultQueue::append,
        )
}

fn main() {
    // expected output (modulo floating point rounding)
    //
    // THE TRAGEDY OF ROMEO AND JULIET (209.9525770093088)
    // Little Women (4.284746469577731)
    for (score, name) in search("Romeo", DOCUMENTS, 2).results {
        println!("{} ({})", name, score);
    }
}
