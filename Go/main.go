package main

import (
	"encoding/json"
	"fmt"
	"io"
	"math/rand"
	"net/http"
	"os/exec"
	"strings"
	"time"
)

func main() {

	var input int
	fmt.Println("Select option")
	for {
		fmt.Scanf("%d", &input)
		switch input {
		case 1:
			r, _ := http.Get("https://api.senpy.club/v2/language/Go")
			defer r.Body.Close()
			b, _ := io.ReadAll(r.Body)

			var arr []string
			_ = json.Unmarshal([]byte(b), &arr)

			rand.Seed(time.Now().UnixNano())

			url := arr[rand.Intn(len(arr))]
			cmd := exec.Command("open", url)
			cmd.Start()

		case 2:
			r, _ := http.Get("https://api.senpy.club/v2/languages")
			defer r.Body.Close()
			b, _ := io.ReadAll(r.Body)

			var arr []string
			_ = json.Unmarshal([]byte(b), &arr)

			rand.Seed(time.Now().UnixNano())
			language_url := "https://api.senpy.club/v2/language/" + arr[rand.Intn(len(arr))]

			r, _ = http.Get(language_url)
			defer r.Body.Close()
			b, _ = io.ReadAll(r.Body)

			_ = json.Unmarshal([]byte(b), &arr)

			rand.Seed(time.Now().UnixNano())

			url := arr[rand.Intn(len(arr))]
			strings.ReplaceAll(url, "#", "%23")
			strings.ReplaceAll(url, "+", "%2B")
			cmd := exec.Command("open", url)
			cmd.Start()

		default:
			fmt.Println("Uknown option")
		}
	}

}
