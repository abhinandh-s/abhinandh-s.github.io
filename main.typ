#import "@local/ui:0.1.0": *

#webpage("index.html", [Home], show-pdf: false)[
  #html.head([
  #html.link(rel: "icon", href: "favicon.svg", type: "image/svg+xml")
  #html.link(rel: "icon", href: "favicon.ico", type: "image/x-icon")
])

=== Hello, I'am
= Abhinandh S.

*Welcome to my corner of Internet.*

== About Me.

I am a 23-years-old guy from India. I'm currently a CMA student at Biswas Institute of Management Studies, Kerala, India. I spend some of my time messing around with computers and software. This site is a home for my psychological dysfunctioning. It's a place where I'm in control; no algorithms, no censorship, and no manipulation. Just raw thoughts and code.

] <home>

#webpage("excel.html", [Excel], show-pdf: false)[
  #include "excel/mod.typ"
] <excel>

#asset("favicon.ico", read("/images/favicon.ico", encoding: none))
#asset("favicon.svg", read("/images/favicon.svg", encoding: none))