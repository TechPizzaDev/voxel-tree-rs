#import "abbr.typ"
#show: abbr.show-rule

#import "markers.typ"
#show: markers.show-rule

#import "abbreviations.typ"
#abbreviations.define(abbr)

// TODO: move to style.typ?

#abbr.cfg.update(cfg => (
  cfg
    + (
      style-short: it => text(rgb("#003060"), it),
      style-long: it => text(rgb("#003060"), it),
    )
))

#show link: it => {
  set text(blue)
  it
}

#show ref: it => {
  set text(blue)
  it
}

#import "sections/indices.typ"
#import "keywords.typ"
#import "ieee-template.typ"
#show: ieee-template.ieee.with(
  title: [Procedural trees for real-time environments],
  abstract: include "sections/abstract.typ",
  authors: (
    (
      name: "Michal Piatkowski",
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
  prefix: {
    show outline.entry.where(level: 1): it => {
      v(0.6em)
      strong(it)
    }
    indices.headings

    linebreak()

    // colbreak() // Nudge intro header to next column
  },
)

#import "sections/mod.typ": body
#body(abbr)
