use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::{fs};
use std::path::PathBuf;
use std::process::exit;

use anyhow::{Result};
use rio_api::model::{Literal, NamedNode, Subject, Term};
use rio_api::parser::TriplesParser;
use rio_turtle::{NTriplesParser, TurtleError};
use serde::{Deserialize, Serialize};
// use serde_jsonlines::{json_lines, write_json_lines};
// use serde_json::Result;

use clap::Parser;
use serde_jsonlines::JsonLinesWriter;

/// Tool to filter Wikidata NTriples file to get english labels and descriptions
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file to read in n-triples (.nt) file format
    input: std::path::PathBuf,

    /// Output file to write with json format
   #[arg(short, long, default_value = "./output.json")]
    output: Option<std::path::PathBuf>,

    /// Print progress information
    #[arg(short, long)]
    verbose: bool
}



// #[derive(Serialize, Deserialize)]
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct EntityInfo {
    /** Human-readable name (english) */
    label: Option<String>,
    /** Also in english */
    description: Option<String>,
}

#[derive(Eq, PartialEq, Debug, Clone, Hash, Serialize, Deserialize)]
struct EntityInfoWithID {
    id: String,
    /** Human-readable name (english) */
    label: Option<String>,
    /** Also in english */
    description: Option<String>,
}

// /** Wikidata ID */
// id: String,

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
struct LanguageTaggedString<'a> {
    /// The [lexical form](https://www.w3.org/TR/rdf11-concepts/#dfn-lexical-form).
    value: &'a str,
    /// The [language tag](https://www.w3.org/TR/rdf11-concepts/#dfn-language-tag).
    language: &'a str,
}

fn to_lang_literal(object: Term) -> Option<LanguageTaggedString> {
    match object {
        Term::NamedNode(_) => None,
        Term::BlankNode(_) => None,
        Term::Literal(l) => match l {
            Literal::LanguageTaggedString { value, language } => {
                Some(LanguageTaggedString { value, language })
            }
            Literal::Simple { value: _ } => None,
            Literal::Typed { value: _, datatype: _ } => None,
        },
        Term::Triple(_) => None,
    }
}

fn to_named_node<'a>(subject: &'a Subject) -> Option<&'a NamedNode<'a>> {
    match subject {
        Subject::NamedNode(n) => Some(n),
        Subject::BlankNode(_) => None,
        Subject::Triple(_) => None,
    }
}

fn get_extension_or_default(buf: &PathBuf) -> &str {
    buf.extension().unwrap_or_default().to_str().unwrap_or_default()
}

fn main() -> Result<()> {
    let args = Args::parse();
    let output = args.output.unwrap();
    if get_extension_or_default(&args.input) != "nt" || get_extension_or_default(&output) != "json" {
        println!("Invalid path args");
        exit(1);
    }
    
    let f = File::open(args.input)?;
    let reader = BufReader::new(f);

    fs::create_dir_all(&output.parent().unwrap())?;
    let output_file = File::create(&output)?;
    let mut writer = JsonLinesWriter::new(output_file);

    let rdfs_label = NamedNode {
        iri: "http://www.w3.org/2000/01/rdf-schema#label",
    };
    let schemaorg_description = NamedNode {
        iri: "http://schema.org/description",
    };

    // We actually know the file is likely to be >5billion, so it would integer overflow
    let mut count_triples: i64 = 0;

    // A map from wikidata ID to label and description
    let mut entity_map: HashMap<String, [Option<String>; 3]> = HashMap::new();

    let mut print_counter: i64 = 0;

    NTriplesParser::new(reader).parse_all(&mut |t| {
        let subject = to_named_node(&t.subject);
        let object = to_lang_literal(t.object);
        if subject.is_some() && object.is_some() {
            let sub = subject.unwrap();
            let obj = object.unwrap();
            if sub.iri.contains("http://www.wikidata.org/entity/") && obj.language == "en" {
                let wiki_id = sub.iri.replace("http://www.wikidata.org/entity/", "");
                
                if t.predicate == rdfs_label.into() {
                    let modval = entity_map.entry(wiki_id.clone());
                    let label = obj.value;
                    modval
                        .and_modify(|ent| ent[0] = Some(label.to_string()))
                        .or_insert([Some(label.to_string()), None, None]);
                }

                if t.predicate == schemaorg_description.into() {
                    let modval = entity_map.entry(wiki_id.clone());
                    // TODO: refactor to be DRY
                    let description = obj.value;
                    modval
                        .and_modify(|ent| ent[1] = Some(description.to_string()))
                        .or_insert([None, Some(description.to_string()), None]);
                }
                let modval = entity_map.entry(wiki_id.clone());
                let vals = modval.or_default();
                // All the real values, are defined, the complete value is not.
                if vals[0..2].iter().all(|m| m.is_some()) && !vals[2].is_some() {
                    writer.write(&EntityInfoWithID{id: wiki_id, label: vals[0].clone(), description: vals[1].clone()})?;
                    vals[2] = Some("DONE".to_string());
                }
            }
        }

        // The mod isn't much different
        if print_counter == 10_000 {
            writer.flush()?;
            if args.verbose {
                println!("Parsed {} triples", count_triples);
                // println!("{:#?}", entity_map);
                print_counter = 0;
            }
        }
        
        count_triples += 1;
        print_counter += 1;

        Ok(()) as Result<(), TurtleError>
    })?;

    // let mut output = Vec::new();
    // for (k, v) in entity_map {
    //     output.push(EntityInfoWithID {id: k, label: v[0], description: v[1] });
    // }

    // let j = serde_json::to_string(&output)?;
    // let out = args.output.unwrap();
    
    // fs::write(out, j)?;

    Ok(())
}
