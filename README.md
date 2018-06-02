# imm
like npm. but for iota modules

----

### Develop

Watch compile with hot reloading:

```bash
yarn
yarn shadow-cljs watch app
```

Start program:

```bash
node target/index.js
```

### REPL

Start a REPL connected to current running program, `app` for the `:build-id`:

```bash
yarn shadow-cljs cljs-repl app
```

### Build

```bash
shadow-cljs release app
```

Compiles to `target/index.js`.

### Steps

* compile ClojureScript
* run `node target/index.js` to start app and connect reload server
