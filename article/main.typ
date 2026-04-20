#import "abbr.typ"
#show: abbr.show-rule

#import "markers.typ"
#show: markers.show-rule

#import "abbreviations.typ"
#abbreviations.define(abbr)

// TODO: move to style.typ?
#show link: it => {
  set text(blue)
  it
}

#show ref: it => {
  set text(blue)
  it
}

#import "keywords.typ"
#import "ieee-template.typ"
#show: ieee-template.ieee.with(
  title: [Procedural trees for real-time environments],
  abstract: include "sections/abstract.typ",
  authors: (
    (
      name: "Michal Piatkowski",
    ),
    (
      name: "Georgios Palamas",
      department: [Supervisor],
    ),
  ),
  organizations: (
    (
      name: "Malmö University",
      location: "Sweden",
    ),
  ),
  index-terms: keywords.items,
  bibliography: bibliography("refs.bib"),
  figure-supplement: [Fig.],
)

#import "sections/mod.typ": body
#body(abbr)
