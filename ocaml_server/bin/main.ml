let handleGetRoot _ =
    Dream.html "Hello";;

let handleGetHello request =
    Dream.param request "name"
    |> Template.render
    |> Dream.html;;

let () =
    Dream.run ~port:3333
    @@ Dream.logger
    @@ Dream.router [
        Dream.get "/" handleGetRoot;
        Dream.get "/hello/:name" handleGetHello;
    ];;

