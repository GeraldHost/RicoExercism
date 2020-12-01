package gigasecond

import "time"

func AddGigasecond(t time.Time) time.Time {
  giga := 1000000000
  t = t.Add(time.Second * time.Duration(giga))
  return t
}
