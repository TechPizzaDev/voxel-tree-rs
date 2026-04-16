
#let TODO_GREEN = rgb(5%, 60%, 5%)

#let todo(body, critical: false) = {
  set text(TODO_GREEN)
  set text(red) if critical

  [*TODO:* #body]
}

#let mark(body) = [
  #show "TODO": it => {
    set text(TODO_GREEN)
    it
  }
  #show "FIXME": it => {
    set text(rgb(0, 200, 200))
    it
  }

  #body
]
