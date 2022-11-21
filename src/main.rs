
use rio_turtle::{NTriplesParser, TurtleError};
use rio_api::parser::TriplesParser;
use rio_api::model::NamedNode;

let file = b"<http://example.com/foo> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/Person> .
<http://example.com/foo> <http://schema.org/name> \"Foo\" .
<http://example.com/bar> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/Person> .
<http://example.com/bar> <http://schema.org/name> \"Bar\" .";

let rdf_type = NamedNode { iri: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type" };
let schema_person = NamedNode { iri: "http://schema.org/Person" };
let mut count = 0;
NTriplesParser::new(file.as_ref()).parse_all(&mut |t| {
    if t.predicate == rdf_type && t.object == schema_person.into() {
        count += 1;
    }
    Ok(()) as Result<(), TurtleError>
})?;
assert_eq!(2, count);