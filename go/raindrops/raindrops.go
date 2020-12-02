package raindrops

import "strconv"

func Convert(n int) (ret string) {
  if n % 3 == 0 {
    ret += "Pling"
  }
  if n % 5 == 0 {
    ret += "Plang"
  }
  if n % 7 == 0 {
    ret += "Plong"
  }
  if len(ret) == 0 {
    ret = strconv.Itoa(n)
  }
  return
}
