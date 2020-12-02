package twofer

import (
  "fmt"
  "strings"
)

func ShareWith(name string) string {
  if v := strings.TrimSpace(name); v == "" {
    return "One for you, one for me."
  }
  return fmt.Sprintf("One for %s, one for me.", name)
}
