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

	fmt.Printf("%s: got /hello request\n", ctx.Value(keyServerAddr))

	var first = r.URL.Query().Get("first")
	var last = r.URL.Query().Get("last")
	var bio = r.URL.Query().Get("bio")

	if first == "" || last == "" || bio == "" {
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	var person = Person{
		First: first,
		Last:  last,
		Bio:   bio,
	}

	tmpl.Execute(w, person)
}

func run(server *http.Server, cancelCtx func()) {
	fmt.Printf("Listening on http://localhost%s\n", server.Addr)

	var err = server.ListenAndServe()

	if errors.Is(err, http.ErrServerClosed) {
		fmt.Printf("Server closed\n")
	} else if err != nil {
		fmt.Printf("Error starting server: %s\n", err)
		os.Exit(1)
	}

	cancelCtx()
}

func main() {
	var muxOne = http.NewServeMux()
	muxOne.HandleFunc("/", getRoot)

	var muxTwo = http.NewServeMux()
	muxTwo.HandleFunc("/hello", getHello)

	var ctx, cancelCtx = context.WithCancel(context.Background())

	var serverOne = &http.Server{
		Addr:    ":3333",
		Handler: muxOne,
		BaseContext: func(l net.Listener) context.Context {
			return context.WithValue(ctx, keyServerAddr, l.Addr().String())
		},
	}

	var serverTwo = &http.Server{
		Addr:    ":4444",
		Handler: muxTwo,
		BaseContext: func(l net.Listener) context.Context {
			return context.WithValue(ctx, keyServerAddr, l.Addr().String())
		},
	}

	fmt.Printf("Type Ctrl+C to stop\n")

	go run(serverOne, cancelCtx)
	go run(serverTwo, cancelCtx)

	<-ctx.Done()
}
