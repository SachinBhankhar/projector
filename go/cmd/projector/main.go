package main

import (
	"fmt"
	"log"

	"github.com/sachinbhankhar/golearn/pkg/projector"
)

func main() {
	opts, err := projector.GetOpts()

	if err != nil {
		log.Fatalf("unable to get options %v", err)
	}

	fmt.Printf("%v", opts)

	config, err := projector.NewConfig(opts)

	if err != nil {
		log.Fatalf("unable to get options %v", err)
	}

	fmt.Printf("%v", config)
}
