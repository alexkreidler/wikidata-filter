use std::collections::HashMap;
use std::fs;

use anyhow::{Context, Result};
use rio_api::model::{Literal, NamedNode, Subject, Term};
use rio_api::parser::TriplesParser;
use rio_turtle::{NTriplesParser, TurtleError};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct EntityInfo {
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
            Literal::Simple { value } => None,
            Literal::Typed { value, datatype } => None,
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

fn main() -> Result<()> {
    //     let file = b"<http://example.com/foo> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/Person> .
    // <http://example.com/foo> <http://schema.org/name> \"Foo\" .
    // <http://example.com/bar> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/Person> .
    // <http://example.com/bar> <http://schema.org/name> \"Bar\" .";
    let file = fs::read_to_string("./data/all-20k.nt").expect("Unable to read file");

    let rdf_type = NamedNode {
        iri: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
    };
    let schema_person = NamedNode {
        iri: "http://schema.org/Person",
    };
    let rdfs_label = NamedNode {
        iri: "http://www.w3.org/2000/01/rdf-schema#label",
    };
    let rdfs_description = NamedNode {
        iri: "http://www.w3.org/2000/01/rdf-schema#description",
    };
    let mut count = 0;

    /** A map from wikidata ID to label and description */
    let mut entity_map = HashMap::new();

    entity_map.insert(
        "Hey".to_string(),
        EntityInfo {
            label: None,
            description: None,
        },
    );

    NTriplesParser::new(file.as_ref()).parse_all(&mut |t| {
        // if (is_named_node(t.subject) && t.subject) {

        // }
        // let x: NamedNode = NamedNode::try_from(t.subject)?;
        // let subj: Literal = Literal::try_from(t.subject)?;
        let subject = to_named_node(&t.subject);
        let object = to_lang_literal(t.object);
        if (subject.is_some() && object.is_some()) {
            let sub = subject.unwrap();
            let obj = object.unwrap();
            if sub.iri.contains("http://www.wikidata.org/entity/") && obj.language == "en" {
                let wikiID = sub.iri.replace("http://www.wikidata.org/entity/", "");
                if t.predicate == rdfs_label.into() {
                    let label = obj.value;
                    let modval = entity_map.entry(wikiID.clone());
                    modval
                        .and_modify(|ent| ent.label = Some(label.to_string()))
                        .or_insert(EntityInfo {
                            label: Some(label.to_string()),
                            description: None,
                        });
                }

                if t.predicate == rdfs_description.into() {
                    // TODO: refactor to be DRY
                    let description = obj.value;
                    // .clone().to_string();
                    let modval = entity_map.entry(wikiID.clone());
                    modval
                        .and_modify(|ent| ent.description = Some(description.to_string()))
                        .or_insert(EntityInfo {
                            label: None,
                            description: Some(description.to_string()),
                        });
                }
            }
        }

        // if x.iri.contains("http://www.wikidata.org/entity/") && t.predicate == rdfs_label.into() && value. {

        // }

        Ok(()) as Result<(), TurtleError>
    })?;

    Ok(())
}
