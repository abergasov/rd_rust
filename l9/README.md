send file over tcp is in mask `FILE%FILE_TYPE%FILE_LENGTH%FILE_CONTENT`

schema non-ideal, better improvements

* is encoded FILE_CONTENT to base64 and add several types support
* each client is in separate thread, which is not good for big amount of clients, maybe wrap in tokio

```bash
FILE%FILE_TYPE%FILE_LENGTH%FILE_CONTENT
MESSAGE%FILE_LENGTH%FILE_CONTENT
...
```

```bash
make run # run server
make spawn-client # run client
```

