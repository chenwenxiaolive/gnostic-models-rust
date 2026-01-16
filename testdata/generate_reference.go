// +build ignore

// This program generates reference JSON output from Go gnostic-models
// for comparison with the Rust implementation.
package main

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"

	discovery "github.com/google/gnostic-models/discovery"
	openapiv2 "github.com/google/gnostic-models/openapiv2"
	openapiv3 "github.com/google/gnostic-models/openapiv3"
	"google.golang.org/protobuf/encoding/protojson"
)

func main() {
	// Get the directory where this script is located
	dir := "."
	if len(os.Args) > 1 {
		dir = os.Args[1]
	}

	// Test OpenAPI v3
	v3File := filepath.Join(dir, "petstore-v3.yaml")
	if err := testOpenAPIv3(v3File, filepath.Join(dir, "petstore-v3-reference.json")); err != nil {
		fmt.Fprintf(os.Stderr, "OpenAPI v3 error: %v\n", err)
		os.Exit(1)
	}
	fmt.Println("Generated petstore-v3-reference.json")

	// Test OpenAPI v2
	v2File := filepath.Join(dir, "petstore-v2.json")
	if err := testOpenAPIv2(v2File, filepath.Join(dir, "petstore-v2-reference.json")); err != nil {
		fmt.Fprintf(os.Stderr, "OpenAPI v2 error: %v\n", err)
		os.Exit(1)
	}
	fmt.Println("Generated petstore-v2-reference.json")

	// Test Discovery
	discoveryFile := filepath.Join(dir, "books-discovery.json")
	if err := testDiscovery(discoveryFile, filepath.Join(dir, "books-discovery-reference.json")); err != nil {
		fmt.Fprintf(os.Stderr, "Discovery error: %v\n", err)
		os.Exit(1)
	}
	fmt.Println("Generated books-discovery-reference.json")
}

func testOpenAPIv3(inputFile, outputFile string) error {
	data, err := os.ReadFile(inputFile)
	if err != nil {
		return fmt.Errorf("read file: %w", err)
	}

	doc, err := openapiv3.ParseDocument(data)
	if err != nil {
		return fmt.Errorf("parse: %w", err)
	}

	// Convert to JSON using protojson
	jsonData, err := protojson.MarshalOptions{
		Multiline:       true,
		Indent:          "  ",
		EmitUnpopulated: false,
	}.Marshal(doc)
	if err != nil {
		return fmt.Errorf("marshal: %w", err)
	}

	if err := os.WriteFile(outputFile, jsonData, 0644); err != nil {
		return fmt.Errorf("write: %w", err)
	}

	return nil
}

func testOpenAPIv2(inputFile, outputFile string) error {
	data, err := os.ReadFile(inputFile)
	if err != nil {
		return fmt.Errorf("read file: %w", err)
	}

	doc, err := openapiv2.ParseDocument(data)
	if err != nil {
		return fmt.Errorf("parse: %w", err)
	}

	// Convert to JSON using protojson
	jsonData, err := protojson.MarshalOptions{
		Multiline:       true,
		Indent:          "  ",
		EmitUnpopulated: false,
	}.Marshal(doc)
	if err != nil {
		return fmt.Errorf("marshal: %w", err)
	}

	if err := os.WriteFile(outputFile, jsonData, 0644); err != nil {
		return fmt.Errorf("write: %w", err)
	}

	return nil
}

func testDiscovery(inputFile, outputFile string) error {
	data, err := os.ReadFile(inputFile)
	if err != nil {
		return fmt.Errorf("read file: %w", err)
	}

	doc, err := discovery.ParseDocument(data)
	if err != nil {
		return fmt.Errorf("parse: %w", err)
	}

	// Convert to JSON using protojson
	jsonData, err := protojson.MarshalOptions{
		Multiline:       true,
		Indent:          "  ",
		EmitUnpopulated: false,
	}.Marshal(doc)
	if err != nil {
		return fmt.Errorf("marshal: %w", err)
	}

	if err := os.WriteFile(outputFile, jsonData, 0644); err != nil {
		return fmt.Errorf("write: %w", err)
	}

	return nil
}

// Summary struct for simplified comparison
type Summary struct {
	OpenAPI     string   `json:"openapi,omitempty"`
	Swagger     string   `json:"swagger,omitempty"`
	Title       string   `json:"title"`
	Version     string   `json:"version"`
	Description string   `json:"description,omitempty"`
	PathCount   int      `json:"pathCount"`
	Paths       []string `json:"paths"`
}

func prettyJSON(v interface{}) string {
	b, _ := json.MarshalIndent(v, "", "  ")
	return string(b)
}
