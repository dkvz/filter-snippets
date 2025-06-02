# Personal vim filter snippets
A bunch of vim filters that read from stdin (params arranged line by line) and output the filled in snippet afterwards.

I could just use LuaSnip snippets for these. I could.

These are mostly for my blog. I used to write my articles on a [dated Electron app](https://github.com/dkvz/electron-blog-authoring) I wrote to learn about Electron and editing (regretted it somewhat quickly after that because making an editor is almost always a bad idea).

## Why are we hardcoding snippets in the binary?
Well, it has to be easily usable in different environments / OS and I'd rather have a single binary to move around. This is just for me anyway why am I answering questions I ask myself?

## Snippets
Debating how to use them.

- Just provide the args in order you have to know
- Run the filter asking for placeholders first, then run it again to process them
    + The placeholders are just the args in order

### Large image
```html
<div class="card-panel z-depth-3 article-image center-image" style="max-width: 1000px">
<a href="/stuff/" target="_blank"><img src="" alt="" class="responsive-img"></a>
<div class="image-legend"></div>
</div>
```

Placeholders:
* max-width, we could leave the default 1000px if the first argument is not a number
* image link: can be two of them
    * Have them both on the same line, space separated
* alt (have to escape things in there)
* legend

We should have some kind of escape function for arguments.

-> Detect image links by their starting "/" or "https?://"
