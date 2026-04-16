#import "ieee-template.typ": ieee
#import "todo.typ": mark, todo

#show: mark

#show link: it => {
  set text(blue)
  it
}

#show ref: it => {
  set text(blue)
  it
}

#import "keywords.typ"
#show: ieee.with(
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
      name: "Malmö Universitet",
      location: "Sweden",
    ),
  ),
  index-terms: keywords.items,
  bibliography: bibliography("refs.bib"),
  figure-supplement: [Fig.],
)

#include "sections/mod.typ"
