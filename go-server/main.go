package main

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"html/template"
	"io"
	"net"
	"net/http"
	"os"
)

const (
	keyServerAddr = "serverAddr"
	pgUsername    = "admin"
	pgPassword    = "secret"
	pgHost        = "localhost"
	pgPort        = "5432"
	pgName        = "go-server"
	appPort       = ":3333"
)

var tmpl = template.Must(template.ParseFiles("hello.html"))

type Person struct {
	First string
	Last  string
	Bio   string
}

func db() *sql.DB {
	var conn = fmt.Sprintf(
		"postgresql://%s:%s@%s:%s/%s?sslmode=disable",
		pgUsername,
		pgPassword,
		pgHost,
		pgPort,
		pgName,
	)

	var db, err = sql.Open("postgres", conn)

	if err != nil {
		panic(err)
	}

	return db
}

func getRoot(w http.ResponseWriter, r *http.Request) {
	var ctx = r.Context()

	fmt.Printf("%s: got / request\n", ctx.Value(keyServerAddr))
	io.WriteString(w, "This is my website!\n")
}

func getHello(w http.ResponseWriter, r *http.Request) {
	var ctx = r.Context()

	var first = r.URL.Query().Get("first")
	var last = r.URL.Query().Get("last")
	var bio = r.URL.Query().Get("bio")

	if first == "" || last == "" || bio == "" {
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	fmt.Printf("%s: got /hello request\n", ctx.Value(keyServerAddr))

	var person = Person{
		First: first,
		Last:  last,
		Bio:   bio,
	}

	tmpl.Execute(w, person)
}

func main() {
	var mux = http.NewServeMux()

	mux.HandleFunc("/", getRoot)
	mux.HandleFunc("/hello", getHello)

	var ctx = context.Background()

	var server = &http.Server{
		Addr:    appPort,
		Handler: mux,
		BaseContext: func(l net.Listener) context.Context {
			return context.WithValue(ctx, keyServerAddr, l.Addr().String())
		},
	}

	fmt.Printf("Listening on http://localhost%s\n", appPort)
	fmt.Printf("Type Ctrl+C to stop\n")

	var err = server.ListenAndServe()

	if errors.Is(err, http.ErrServerClosed) {
		fmt.Printf("Server closed\n")
	} else if err != nil {
		fmt.Printf("Error starting server: %s\n", err)
		os.Exit(1)
	}
}
