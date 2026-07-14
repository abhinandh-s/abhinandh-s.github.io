---
layout: home
title: Blog
permalink: /blog/
---


Here are my latest posts:

<ul>
  {% for post in site.posts %}
    <li>
      <a href="{{ post.url }}">{{ post.title }}</a> 
      - {{ post.date | date: "%B %d, %Y" }}
    </li>
  {% endfor %}
</ul>
