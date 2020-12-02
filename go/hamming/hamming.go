package hamming

import "errors"

func Distance(a, b string) (int, error) {
  if len(a) != len(b) {
		return -1, errors.New("string lengths differ")
	}

  hamming := 0
  for i := range a {
    if a[i] != b[i] {
      hamming += 1
    }
  } 
  return hamming, nil
}
