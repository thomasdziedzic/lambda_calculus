type
  TermKind = enum
    tkBVar,
    tkUVar,
    tkAbs,
    tkApp
  Term = ref object
    case kind: TermKind
    of tkBVar: index: int
    of tkUVar: name: string
    of tkAbs: body: Term
    of tkApp: left, right: Term

# test is the identity function applied to y, i.e. (λx.x) y
var test = Term(kind: tkApp, left: Term(kind: tkAbs, body: Term(kind: tkBVar, index: 0)), right: Term(kind: tkUVar, name: "y"))

proc eval(term: Term): int =
  case term.kind
  of tkBVar: 1
  of tkUVar: 2
  of tkAbs: 3
  of tkApp: 4

echo eval(test)
