require "http/server"
require "router"
require "ecr"

class Index
  def initialize(name : String)
    @name = name
  end

  ECR.def_to_s "src/index.ecr"
end

class MyServer
  include Router
  include HTTP

  @log_handler = LogHandler.new(Log.for("http.server"))
  @error_handler = ErrorHandler.new
  @static_file_handler = StaticFileHandler.new(File.expand_path("../public", __FILE__))

  def draw_routes
    get "/hello/:name" do |context, params|
      index = Index.new params["name"]

      context.response.content_type = "text/html"
      context.response.print index.to_s
      context
    end
  end

  def run
    draw_routes

    handlers = [@log_handler, @error_handler, @static_file_handler, route_handler]

    server = Server.new handlers
    address = server.bind_tcp 8080
    puts "Listening on http://#{address}"
    server.listen
  end
end

server = MyServer.new
server.run
