use clap::Clap;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::{fs, str};

#[derive(Debug)]
struct Node {
    freq: i32,
    ch: Option<char>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn make_counts(text: &str) -> HashMap<char, i32> {
    let mut out: HashMap<char, i32> = HashMap::new();

    for c in text.chars() {
        if out.contains_key(&c) {
            *out.get_mut(&c).unwrap() += 1;
        } else {
            out.insert(c, 1);
        }
    }

    out
}

fn make_map_recur(node: &Box<Node>, map: &mut HashMap<char, String>, root: String) {
    if let Some(c) = node.ch {
        // println!("inserting {} as {}", c, root);
        map.insert(c, root);
    } else {
        if let Some(ref l) = node.left {
            // println!("going left, {:?}", root);
            make_map_recur(l, map, root.clone() + "0");
        }
        if let Some(ref r) = node.right {
            // println!("going right, {:?}", root);
            make_map_recur(r, map, root.clone() + "1");
        }
    }
}

fn tree_to_map(tree: Box<Node>) -> HashMap<char, String> {
    let mut out = HashMap::new();

    make_map_recur(&tree, &mut out, String::from(""));

    out
}

fn encode(msg: &str, map: &HashMap<char, String>) -> String {
    let mut out = String::from("");
    let maplen = map.len() as u32;

    out.push_str(format!("{:032b}", maplen).as_str());

    // add the char map
    for (c, b) in map {
        let len = b.len() as u32;
        out.push_str(format!("{:08b}", len).as_str());
        out.push_str(format!("{:08b}", *c as u8).as_str());
    }

    // add the compressed message
    for c in msg.chars() {
        out.push_str(map.get(&c).unwrap());
    }

    out
}

#[derive(Clap)]
#[clap(name = "cmp-rs")]
struct CLI {
    // Name of the file to compress
    #[clap(short, long)]
    infile: String,

    // Name of the file to write compressed data to
    #[clap(short, long)]
    outfile: String,
}

fn main() {
    let cliargs = CLI::parse();
    let msg =
        fs::read_to_string(cliargs.infile).expect("Something went wrong reading the input file");

    let counts = make_counts(msg.as_str());
    let mut nodes: Vec<Box<Node>> = counts
        .iter()
        .map(|x| {
            Box::new(Node {
                freq: *x.1,
                ch: Some(*x.0),
                left: None,
                right: None,
            })
        })
        .collect();

    // construct hoffman tree
    while nodes.len() > 1 {
        nodes.sort_by(|x, y| (&(y.freq)).cmp(&(x.freq)));
        let a = nodes.pop().unwrap();
        let b = nodes.pop().unwrap();
        let c = Box::new(Node {
            freq: a.freq + b.freq,
            ch: None,
            left: Some(a),
            right: Some(b),
        });
        nodes.push(c);
    }

    let char_map = tree_to_map(nodes.pop().unwrap());

    let compressed = encode(msg.as_str(), &char_map);

    let x: Vec<&str> = compressed
        .as_str()
        .as_bytes()
        .chunks(4)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    let nums: Vec<u8> = x
        .iter()
        .map(|x| u8::from_str_radix(x, 2).unwrap())
        .collect();

    let mut ofile = File::create(cliargs.outfile).unwrap();
    let _ = ofile
        .write_all(nums.as_slice())
        .expect("Something went wrong writing to the output file");
}
