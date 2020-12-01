package acronym
import "strings"

func Abbreviate(s string) (result string) {
  parsed := strings.Replace(s, "-", " ", -1)
  parsed = strings.Replace(parsed, "_", " ", -1)

  words := strings.Fields(parsed)
  for _, word := range words {
    result += strings.ToUpper(string(word[0]))
  }

  return 
}
