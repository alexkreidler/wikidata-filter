# wikidata-filter

A tool to get the human-readable labels, descriptions, and wikidata IDs from a Wikidata dump like `latest-truthy.nt.gz`, `latest-all.nt.gz`, etc.

Takes a big dump of WikiData like this:

<details>
<summary>N-Triples Data</summary>

```nt
<http://wikiba.se/ontology#Dump> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/Dataset> .
<http://wikiba.se/ontology#Dump> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Ontology> .
<http://wikiba.se/ontology#Dump> <http://creativecommons.org/ns#license> <http://creativecommons.org/publicdomain/zero/1.0/> .
<http://wikiba.se/ontology#Dump> <http://schema.org/softwareVersion> "1.0.0" .
<http://wikiba.se/ontology#Dump> <http://schema.org/dateModified> "2019-09-09T23:00:01Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<http://wikiba.se/ontology#Dump> <http://www.w3.org/2002/07/owl#imports> <http://wikiba.se/ontology-1.0.owl> .
<https://www.wikidata.org/wiki/Special:EntityData/Q31> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/Dataset> .
<https://www.wikidata.org/wiki/Special:EntityData/Q31> <http://schema.org/about> <http://www.wikidata.org/entity/Q31> .
<https://www.wikidata.org/wiki/Special:EntityData/Q31> <http://schema.org/version> "1007238018"^^<http://www.w3.org/2001/XMLSchema#integer> .
<https://www.wikidata.org/wiki/Special:EntityData/Q31> <http://schema.org/dateModified> "2019-09-02T21:49:21Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<https://www.wikidata.org/wiki/Special:EntityData/Q31> <http://wikiba.se/ontology#sitelinks> "320"^^<http://www.w3.org/2001/XMLSchema#integer> .
<https://www.wikidata.org/wiki/Special:EntityData/Q31> <http://wikiba.se/ontology#statements> "766"^^<http://www.w3.org/2001/XMLSchema#integer> .
<https://www.wikidata.org/wiki/Special:EntityData/Q31> <http://wikiba.se/ontology#identifiers> "75"^^<http://www.w3.org/2001/XMLSchema#integer> .
<http://www.wikidata.org/entity/Q31> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://wikiba.se/ontology#Item> .
<https://it.wikivoyage.org/wiki/Belgio> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/Article> .
<https://it.wikivoyage.org/wiki/Belgio> <http://schema.org/about> <http://www.wikidata.org/entity/Q31> .
<https://it.wikivoyage.org/wiki/Belgio> <http://schema.org/inLanguage> "it" .
<https://it.wikivoyage.org/wiki/Belgio> <http://schema.org/isPartOf> <https://it.wikivoyage.org/> .
<https://it.wikivoyage.org/wiki/Belgio> <http://schema.org/name> "Belgio"@it .
<https://it.wikivoyage.org/> <http://wikiba.se/ontology#wikiGroup> "wikivoyage" .
<https://an.wikipedia.org/wiki/Belchica> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/Article> .
<https://an.wikipedia.org/wiki/Belchica> <http://schema.org/about> <http://www.wikidata.org/entity/Q31> .
<https://an.wikipedia.org/wiki/Belchica> <http://schema.org/inLanguage> "an" .
<https://an.wikipedia.org/wiki/Belchica> <http://schema.org/isPartOf> <https://an.wikipedia.org/> .
<https://an.wikipedia.org/wiki/Belchica> <http://schema.org/name> "Belchica"@an .
```

</details>

And generates a JSON document like this:

```json
[
  {
    "id": "Q23",
    "label": "George Washington",
    "description": "First President of the United States"
  },
  {
    "id": "Q24",
    "label": "Jack Bauer",
    "description": "character from the television series 24"
  },
  {
    "id": "Q42",
    "label": "Douglas Adams",
    "description": "British author and humorist"
  },
  {
    "id": "Q31",
    "label": "Belgium",
    "description": "federal constitutional monarchy in Western Europe"
  },
  {
    "id": "Q8",
    "label": "happiness",
    "description": "mental or emotional state of well-being characterized by pleasant emotions"
  }
]
```
