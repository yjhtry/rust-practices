package cli

import (
	"fmt"
	"io"
	"log"
	"net/http"
	"os"

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
					}

					defer res.Body.Close()

					for key, value := range res.Header {
						fmt.Printf("%s: %s\n", key, value)
					}

					body, err := io.ReadAll(res.Body)
					if err != nil {
						fmt.Println("读取响应体失败:", err)
						return nil
					}

					fmt.Println(string(body))

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
