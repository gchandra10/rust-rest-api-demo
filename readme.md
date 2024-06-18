

```
cargo run --bin 01_simple_get

cargo run --bin 02_json_get

cargo run --bin 03_json_get
```

First run the web app cargo run --bin 06_rust_api_web_app
and then run 08_simple_get_from_api and 07_json_post to test the output.

### List Existing data

```
curl http://127.0.0.1/

curl http://127.0.0.1/items
```

### Post a new data

```
curl -X POST -H "Content-Type: application/json" -d '{"name":"item 99"}' http://127.0.0.1:5002/items 
```

### Update Existing Item

```
curl -X PUT -H "Content-Type: application/json" -d '{"id":3,"name":"item 3"}' http://127.0.0.1:5002/items/3
```

### Delete Existing Item

```
curl -X DELETE http://127.0.0.1:5002/items/3
```