use std::collections::HashMap;
use std::{fs, os};
use std::path::PathBuf;
use std::process::exit;

use anyhow::{Result};
use rio_api::model::{Literal, NamedNode, Subject, Term};
use rio_api::parser::TriplesParser;
use rio_turtle::{NTriplesParser, TurtleError};
use serde::{Deserialize, Serialize};
// use serde_json::Result;

use clap::Parser;

/// Tool to filter Wikidata NTriples file to get english labels and descriptions
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file to read in n-triples (.nt) file format
    input: std::path::PathBuf,

    /// Output file to write with json format
   #[arg(short, long, default_value = "./output.json")]
    output: Option<std::path::PathBuf>,
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
    if get_extension_or_default(&args.input) != "nt" || get_extension_or_default(&args.output.clone().unwrap()) != "json" {
        println!("Invalid path args");
        exit(1);
    }
    
    let file = fs::read_to_string(args.input).expect("Unable to read file");
    
    let rdfs_label = NamedNode {
        iri: "http://www.w3.org/2000/01/rdf-schema#label",
    };
    let schemaorg_description = NamedNode {
        iri: "http://schema.org/description",
    };

    // A map from wikidata ID to label and description
    let mut entity_map: HashMap<String, EntityInfo> = HashMap::new();

    NTriplesParser::new(file.as_ref()).parse_all(&mut |t| {
        let subject = to_named_node(&t.subject);
        let object = to_lang_literal(t.object);
        if subject.is_some() && object.is_some() {
            let sub = subject.unwrap();
            let obj = object.unwrap();
            if sub.iri.contains("http://www.wikidata.org/entity/") && obj.language == "en" {
                let wiki_id = sub.iri.replace("http://www.wikidata.org/entity/", "");
                if t.predicate == rdfs_label.into() {
                    let label = obj.value;
                    let modval = entity_map.entry(wiki_id.clone());
                    modval
                        .and_modify(|ent| ent.label = Some(label.to_string()))
                        .or_insert(EntityInfo {
                            label: Some(label.to_string()),
                            description: None,
                        });
                }

                if t.predicate == schemaorg_description.into() {
                    // TODO: refactor to be DRY
                    let description = obj.value;
                    let modval = entity_map.entry(wiki_id.clone());
                    modval
                        .and_modify(|ent| ent.description = Some(description.to_string()))
                        .or_insert(EntityInfo {
                            label: None,
                            description: Some(description.to_string()),
                        });
                }
            }
        }

        Ok(()) as Result<(), TurtleError>
    })?;

    let mut output = Vec::new();
    for (k, v) in entity_map {
        output.push(EntityInfoWithID {id: k, label: v.label, description: v.description });
    }

    let j = serde_json::to_string(&output)?;
    let out = args.output.unwrap();
    
    fs::create_dir_all(out.parent().unwrap())?;
    fs::write(out, j)?;

    Ok(())
}
