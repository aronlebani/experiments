#1 "bin/template.eml.html"
let render name =
let ___eml_buffer = Buffer.create 4096 in
(Buffer.add_string ___eml_buffer "<html>\n<body>\n    <h1>Hello, ");
(Printf.bprintf ___eml_buffer "%s" (Dream_pure.Formats.html_escape (
#4 "bin/template.eml.html"
                   name 
)));
(Buffer.add_string ___eml_buffer "!</h1>\n</body>\n</html>\n");
(Buffer.contents ___eml_buffer)
