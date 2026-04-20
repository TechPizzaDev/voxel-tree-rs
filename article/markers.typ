
#let TODO_GREEN = rgb(5%, 60%, 5%)

#let REFINE_COLOR = rgb(5%, 40%, 60%)

#let todo(body, critical: false) = {
  set text(TODO_GREEN)
  set text(red) if critical

  [*TODO:* #body]
}

#let refine(body) = {
  set text(REFINE_COLOR)

  [*REFINE:* #body]
}

#let show-rule(body) = [
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
