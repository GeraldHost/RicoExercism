package twelve

import (
  "fmt"
  "strings"
)

var lines = []string{
  "a Partridge in a Pear Tree",
  "two Turtle Doves",
  "three French Hens",
  "four Calling Birds",
  "five Gold Rings",
  "six Geese-a-Laying",
  "seven Swans-a-Swimming",
  "eight Maids-a-Milking",
  "nine Ladies Dancing",
  "ten Lords-a-Leaping",
  "eleven Pipers Piping",
  "twelve Drummers Drumming"}

var days = []string{
  "first",
  "second",
  "third",
  "fourth",
  "fifth",
  "sixth",
  "seventh",
  "eighth",
  "ninth",
  "tenth",
  "eleventh",
  "twelfth"}

func Verse(n int) string {
  out := make([]string, 0)
  template := "On the %s day of Christmas my true love gave to me: %s."
  for i := n-1; i >= 0; i-- {
    line := lines[i]
    if n > 1 && i == 0 {
      line = "and " + line
    }
    out = append(out, line)
  }

  return fmt.Sprintf(template, days[n-1], strings.Join(out, ", "))
}

func Song() string {
  arr := make([]string, 0)
  for i := 1; i <= 12; i++ {
    arr = append(arr, Verse(i))
  }
  return strings.Join(arr, "\n")
}
