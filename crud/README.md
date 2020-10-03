# "Crud"

Cause it's just a demo, who needs a name. Testing out Mongo + ACTIX for simple crud ops. 

1. Launch mongo: `docker-compo up`
1. Launch service: `cargo run` (or run `./reload.sh` for hot reload on changes.)
1. Make an object: `http POST localhost:3000/txn cents:='{"val":123}' from:='{"type":"CreditCard"}' to:='{"type":"User", "id":123}' -p=BHbh`

```
{
    "cents": {
        "val": 123
    },
    "from": {
        "type": "CreditCard"
    },
    "to": {
        "id": 123,
        "type": "User"
    }
}
```

```
{
    "_id": "5f77ee7100918aae00ae4973"
}
```

1. Reload it: `http GET localhost:3000/txn/5f77ee7100918aae00ae4973`

```
{
    "cents": {
        "val": 123
    },
    "from": {
        "type": "CreditCard"
    },
    "to": {
        "id": 123,
        "type": "User"
    }
}
```