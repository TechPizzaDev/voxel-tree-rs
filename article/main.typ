#import "ieee-template.typ": ieee
#import "todo.typ": todo, mark

#show: mark

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
    #todo[]
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
  index-terms: ("TODO:", "generative tree modeling"),
  bibliography: bibliography("refs.bib"),
  figure-supplement: [Fig.],
)

#include "sections/intro.typ"

#include "sections/index.typ"
// #pagebreak()

#include "sections/prior_work.typ"

#include "sections/methods.typ"

#include "sections/results.typ"
