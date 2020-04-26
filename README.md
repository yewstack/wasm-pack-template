[![Netlify Status](https://api.netlify.com/api/v1/badges/5ba03ba7-ff8b-4c54-94e7-cd5fd76a6737/deploy-status)](https://app.netlify.com/sites/yew-todomvc/deploys)

## About

This template shows how to create a web app using Yew and wasm-pack. 

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
yarn run start:dev
```


## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
