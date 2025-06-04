# Personal vim filter snippets
A bunch of vim filters that read from stdin (params arranged line by line) and output the filled in snippet afterwards.

I could just use LuaSnip snippets for these. I could.

These are mostly for my blog. I used to write my articles on a [dated Electron app](https://github.com/dkvz/electron-blog-authoring) I wrote to learn about Electron and editing (regretted it somewhat quickly after that because making an editor is almost always a bad idea).

## Features
- Show the list of available snippets with some arguments
    + Requires having a structured way to store the hardcoded snippets 
- Should we have snippet categories? Let's start with naming conventions

Call the binary with the "-p" argument and a snippet name to generate the placeholders - Just outputs the result if there are no placeholders for that snippet.

## Why are we hardcoding snippets in the binary?
Well, it has to be easily usable in different environments / OS and I'd rather have a single binary to move around. This is just for me anyway why am I answering questions I ask myself?

## Snippets
Debating how to use them.

- Just provide the args in order you have to know
- Run the filter asking for placeholders first, then run it again to process them
    + The placeholders are just the args in order
    + Running the filter with no lines being fed in prints the args placeholders

### Large image and placeholder discussion
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
    * If only one, will be repeated in both places
* alt (have to escape things in there)
* legend

We should have some kind of escape function for arguments.

The alt text can be an empty line to ignore it (NOT COOL THO), we also allow legend being empty.

-> We should probably always allow empty values.

The thing that parses these placeholders is a bit too modular to be some sort of descriptive language. It could be a function at first - We could just give it lines from stdin and it figures out what do output.

## TODO
- CLI completion would be really nice for this project
