type
  Comparable = concept x, y
    (x < y) is bool
  Test = object
    x: int

proc `<`(a, b: Test): bool =
  a.x < b.x

var test1 = Test(x: 1)
var test2 = Test(x: 2)

proc tryConcept(x, y: Comparable) =
  echo(x < y)

tryConcept(test1, test2)
