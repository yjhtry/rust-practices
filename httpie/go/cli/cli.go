package cli

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"strings"

	"github.com/fatih/color"
	"github.com/urfave/cli/v2"
)

func Run() {
	app := &cli.App{
		Name:  "Httpie",
		Usage: "Fetch and print response from url",
		Commands: []*cli.Command{
			{
				Name:    "get",
				Aliases: []string{"g"},
				Usage:   "print url response by get",
				Action: func(cCtx *cli.Context) error {
					url := cCtx.Args().First()

					res, err := http.Get(url)

					if err != nil {
						fmt.Println("Fetch Error: ", err)

						return err
					}

					defer res.Body.Close()

					printStatus(res)
					printHeaders(res)
					printBody(res)

					return nil
				},
			},
			{
				Name:    "post",
				Aliases: []string{"p"},
				Usage:   "print url response by post",
				Action: func(cCtx *cli.Context) error {
					fmt.Println("completed task: ", cCtx.Args().First())
					return nil
				},
			},
		},
	}

	if err := app.Run(os.Args); err != nil {
		log.Fatal(err)
	}
}

func PrettyString(str string) (string, error) {
	var prettyJSON bytes.Buffer

	if err := json.Indent(&prettyJSON, []byte(str), "", "    "); err != nil {
		return "", err
	}

	return prettyJSON.String(), nil
}

func printStatus(res *http.Response) {
	var statusCodeText string

	switch res.StatusCode {
	case 200:
		statusCodeText = color.GreenString(fmt.Sprint(res.StatusCode))
	case 400:
		statusCodeText = color.YellowString(fmt.Sprint(res.StatusCode))
	case 404, 500:
		statusCodeText = color.RedString(fmt.Sprint(res.StatusCode))
	}

	fmt.Println(
		color.GreenString("Response Status:"),
		statusCodeText,
		color.BlueString(fmt.Sprint(res.Proto)),
	)
}

func printHeaders(res *http.Response) {
	for key, value := range res.Header {
		fmt.Printf("%s: %s\n", color.GreenString(key), color.CyanString(strings.Join(value, ",")))
	}

	fmt.Println()
}
func printBody(res *http.Response) {
	body, err := io.ReadAll(res.Body)

	if err != nil {
		fmt.Println("读取响应体失败:", err)
		return
	}

	stringBody := string(body)

	if strings.Contains(res.Header.Get("Content-Type"), "application/json") {
		stringJson, err := PrettyString(stringBody)

		if err == nil {
			stringBody = stringJson
		}
	}

	fmt.Println(color.CyanString(stringBody))

}
