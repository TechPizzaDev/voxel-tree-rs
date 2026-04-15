
#let todo(body, critical: false) = {
  set text(green)
  set text(red) if critical

  [*TODO:* #body]
}

#let mark(body) = [
  #show "TODO": it => {
    set text(green)
    it
  }
  #show "FIXME": it => {
    set text(rgb(0, 200, 200))
    it
  }

  #body
]
