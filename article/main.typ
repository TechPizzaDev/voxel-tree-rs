#import "ieee-template.typ": ieee

#show link: it => {
  set text(blue)
  it
}

#show ref: it => {
  set text(blue)
  it
}

#show: ieee.with(
  title: [Procedural trees for real-time environments],
  abstract: [
    TODO
  ],
  authors: (
    (
      name: "Michal Piatkowski",
      department: [Student],
      organization: [Malmö Universitet],
    ),
    (
      name: "Georgios Palamas",
      department: [Supervisor],
      organization: [Malmö Universitet],
    ),
  ),
  index-terms: ("TODO", "generative tree modeling"),
  bibliography: bibliography("refs.bib"),
  figure-supplement: [Fig.],
)

#include "sections/intro.typ"

#include "sections/methods.typ"
