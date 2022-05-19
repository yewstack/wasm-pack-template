## About

This template shows how to create a web app using Yew, wasm-pack and tailwindCSS.

It is a fork of [this repo.](https://github.com/yewstack/yew-wasm-pack-template)

There is a blog post with more detailed information [here.](http://blog.vomkonstant.in/blog/tailwind-with-webassembly-yew)

## 🚴 Usage

### 🛠️ Build

When building for the first time, ensure to install dependencies first.

```
yarn install
```

```
yarn run build
```

### 🔬 Serve locally

```
yarn run dev
```

## 🔋 Batteries Included

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
- [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
