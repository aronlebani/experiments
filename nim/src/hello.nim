# import prologue
# from prologue/middlewares/staticfile import static_file_middleware
import nimja/parser
import jester

type User = object
    name: string
    age: int

proc render(user: User): string =
    compile_template_file(get_script_dir() / "index.html")

# proc index_handler(ctx: Context) {.async.} =
#     let user = User(name: ctx.get_path_params("name"), age: 29)
#     resp user.render

# var app = new_app()
# app.use(static_file_middleware("public"))
# app.get("/hello/{name}", index_handler)
# app.run()

routes:
    get "/hello/@name":
        let user = User(name: @"name", age: 29)
        resp user.render
