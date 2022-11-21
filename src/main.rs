use rio_api::model::{NamedNode, Subject, Term, Literal};
use rio_api::parser::TriplesParser;
use rio_turtle::{NTriplesParser, TurtleError};
use anyhow::{Context, Result};

struct EntityInfo {
    /** Wikidata ID */
    id: String,
    /** Human-readable name (english) */
    label: String,
    /** Also in english */
    description: String
}

// enum Knowable {
//     Subject(Subject),
//     Term(Term)
// }

// fn is_named_node(input: Knowable) {
//     match input {
//         Knowable::Subject(k) => match k {
//             Subject::NamedNode(_) => true,
//             Subject::BlankNode(_) => false,
//             Subject::Triple(_) => false,
//         },
//         Knowable::Term(k) => match k {
//             Term::NamedNode(_) => true,
//             Term::BlankNode(_) => false,
//             Term::Literal(_) => false,
//             Term::Triple(_) => false,
//         },
//     }
// }


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
            Literal::LanguageTaggedString { value, language } => Some(LanguageTaggedString { value, language } ),
            Literal::Simple { value } => None,
            Literal::Typed { value, datatype } => None,
        },
        Term::Triple(_) => None,
    }
}


fn to_named_node(subject: Subject) -> Option<NamedNode> {
    match subject {
        Subject::NamedNode(n) => Some(n),
        Subject::BlankNode(_) => None,
        Subject::Triple(_) => None,
    }
}


fn main() -> Result<()> {
    let file = b"<http://example.com/foo> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/Person> .
<http://example.com/foo> <http://schema.org/name> \"Foo\" .
<http://example.com/bar> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/Person> .
<http://example.com/bar> <http://schema.org/name> \"Bar\" .";

    let rdf_type = NamedNode {
        iri: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
    };
    let schema_person = NamedNode {
        iri: "http://schema.org/Person",
    };
    let rdfs_label = NamedNode {
        iri: "http://www.w3.org/2000/01/rdf-schema#label"
    };
    let rdfs_description = NamedNode {
        iri: "http://www.w3.org/2000/01/rdf-schema#description"
    };
    let mut count = 0;
    NTriplesParser::new(file.as_ref()).parse_all(&mut |t| {
        // if (is_named_node(t.subject) && t.subject) {

        // }
        // let x: NamedNode = NamedNode::try_from(t.subject)?;
        // let subj: Literal = Literal::try_from(t.subject)?;
        let subject = to_named_node(t.subject)
        let object = to_lang_literal(t.object);
        if (subject.is_some() && object.is_some()) {
            let sub = subject.unwrap();
            let obj = object.unwrap();
            if sub.iri.contains("http://www.wikidata.org/entity/") && obj.language == "en" {
                if t.predicate == rdfs_label.into()  {

                }

                if t.predicate == rdfs_description.into() {

                }
            }
        }
        
        // if x.iri.contains("http://www.wikidata.org/entity/") && t.predicate == rdfs_label.into() && value. {

        // }
        
        Ok(()) as Result<(), TurtleError>
    })?;
    assert_eq!(2, count);
    Ok(())
}
