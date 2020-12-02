package raindrops

import "strconv"

var translation = [3][2]interface{}{
  { 3, "Pling" },
  { 5, "Plang" },
  { 7, "Plong" },
}

func Convert(n int) (ret string) {
  for _, v := range translation {
    if n % v[0].(int) == 0 {
      ret += v[1].(string)
    }
  }

  if len(ret) == 0 {
    ret = strconv.Itoa(n)
  }
  return
}
