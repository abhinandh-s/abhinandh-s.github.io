#import "@local/ui:0.1.0": *

#webpage("index.html", [Home], pdf-path: "book.pdf")[
  #html.head([
  #html.link(rel: "icon", href: "favicon.svg", type: "image/svg+xml")
  #html.link(rel: "icon", href: "favicon.ico", type: "image/x-icon")
])


Home Page

] <home>

#webpage("excel.html", [Excel], pdf-path: "book.pdf")[
  #include "excel/mod.typ"
] <excel>

#asset("favicon.ico", read("/images/favicon.ico", encoding: none))
#asset("favicon.svg", read("/images/favicon.svg", encoding: none))