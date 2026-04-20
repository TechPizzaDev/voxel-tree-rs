#import "indices.typ"

#let body(abbr) = [
  #indices.headings

  #include "intro.typ"

  #include "prior_work.typ"

  #include "methods.typ"

  #include "results.typ"

  #include "discussion.typ"

  #include "future_work.typ"

  // TODO: remove pagebreak ?
  #pagebreak()

  #abbr.list(title: "Glossary", columns: 1)

  #indices.figures

  #indices.tables
]
