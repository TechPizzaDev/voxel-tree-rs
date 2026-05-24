#import "indices.typ"

#let body(abbr) = [
  #indices.headings

  #colbreak() // Nudge intro header to next column

  #include "intro.typ"

  #include "concepts/mod.typ"

  #include "algos.typ"

  #include "artifacts.typ"

  #include "results.typ"

  #include "discussion.typ"

  #include "future_work.typ"

  #colbreak() // Nudge end-of-document headers to next column

  #abbr.list(title: "Glossary", columns: 1)

  #indices.figures

  #indices.tables
]
