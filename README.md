# Fibonacci as a Service
This is a serverless app based on [Spin](https://developer.fermyon.com/spin/index) a framework to build WebAssembly components. 
This service provides an API to calculate Fibonacci numbers.
```
/fibonacci/:number
```

## How to execute
1. Install [Spin](https://developer.fermyon.com/spin/install)
2. Compile the app: `spin build`
3. Run the app: `spin up`
4. Call the API: `curl http://localhost:3000/fibonacci/10`

Take in account this is a __wasm32-wasi__ target compilation app, so the maximum value for usize type is __4294967295__.

## How to test
```curl
curl http://localhost:3000/fibonacci/10
```