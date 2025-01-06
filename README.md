# Record Store

The Record Store is a server for sharing and searching for songs with others

## Use Cases

### Starting the server
To start the Record Store:

```sh
$ cargo build
$ target/debug/server
```

The output message will be:

```sh
The server is currently listening on localhost:8080.
```

If you send a HTTP request to the Record Store, it will respond as follows:

```sh
$ curl "http://localhost:8080"
Welcome to the Rust-powered web server!
```

Note: If you are using zsh as your shell, you may notice a `%` at the end of the line. This indicates that cURL’s output does not include a newline character. To eliminate the % at the end, you can use `curl -w '\n' http://localhost:8080` instead.

### Visit count

To trigger the visit count:

```sh
$ curl "http://localhost:8080/count"
Visit count: 1

$ curl "http://localhost:8080/count"
Visit count: 2

$ curl "http://localhost:8080/count"
Visit count: 3
```

To run road tests for the visit count using `oha`:

```sh
$ cargo install oha
$ oha -n 1000000 "http://localhost:8080/count"
$ curl -s "http://localhost:8080/count"
Visit count: 1000001
```

During the time when the Record Store is processing these requests, you may use `htop` to check whether all CPU cores on your machine have been fully utilized.


### Sharing songs to others

```sh
$ curl "http://localhost:8080/songs/new" \
  --json '{"title":"Iridescent", "artist":"Linkin Park", "genre":"Rock"}'
{"id":1,"title":"Iridescent","artist":"Linkin Park","genre":"Rock","play_count":0}

$ curl "http://localhost:8080/songs/new" \
  --json '{"title":"Yesterday", "artist":"The Beatles", "genre":"Pop"}'
{"id":2,"title":"Yesterday","artist":"Beatles","genre":"Pop","play_count":0}

$ curl "http://localhost:8080/songs/new" \
  --json '{"title":"Sweet Home Chicago", "artist":"Robert Johnson", "genre":"Blues"}'
{"id":3,"title":"Sweet Home Chicago","artist":"Robert Johnson","genre":"Blues","play_count":0}

$ curl "http://localhost:8080/songs/new" \
  --json '{"title":"One More Light", "artist":"Linkin Park", "genre":"Pop"}'
{"id":4,"title":"One More Light","artist":"Linkin Park","genre":"Pop","play_count":0}
```

### Searching for songs
A web client can send a query to the web server using the /songs/search route, searching for a particular title, artist, genre, or any combination of these attributes. For example:

```sh
$ curl "http://localhost:8080/songs/search?title=Chicago"
[{"id":3,"title":"Sweet Home Chicago","artist":"Robert Johnson","genre":"Blues","play_count":0}]

$ curl "http://localhost:8080/songs/search?artist=Johnson"
[{"id":3,"title":"Sweet Home Chicago","artist":"Robert Johnson","genre":"Blues","play_count":0}]

$ curl "http://localhost:8080/songs/search?genre=Blues"
[{"id":3,"title":"Sweet Home Chicago","artist":"Robert Johnson","genre":"Blues","play_count":0}]
```

```sh
$ curl "http://localhost:8080/songs/search?genre=Rock&artist=Linkin+Park"
[{"id":1,"title":"Iridescent","artist":"Linkin Park","genre":"Rock","play_count":0}]

$ curl "http://localhost:8080/songs/search?artist=Linkin"
[{"id":1,"title":"Iridescent","artist":"Linkin Park","genre":"Rock","play_count":0},{"id":4,"title":"One More Light","artist":"Linkin Park","genre":"Pop","play_count":0}]

$ curl "http://localhost:8080/songs/search?artist=linkin+park"
[{"id":1,"title":"Iridescent","artist":"Linkin Park","genre":"Rock","play_count":0},{"id":4,"title":"One More Light","artist":"Linkin Park","genre":"Pop","play_count":0}]
```

```
$ curl "http://localhost:8080/songs/search?genre=Blues&artist=Linkin+Park"
[]
```

### Playing music

```sh
$ curl "http://localhost:8080/songs/play/1"
{"id":1,"title":"Iridescent","artist":"Linkin Park","genre":"Rock","play_count":1}

$ curl "http://localhost:8080/songs/play/1"
{"id":1,"title":"Iridescent","artist":"Linkin Park","genre":"Rock","play_count":2}
```

```sh
$ curl "http://localhost:8080/songs/play/10"
{"error":"Song not found"}
```
