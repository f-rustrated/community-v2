# Installation(MAC)
## Run EventStoreDB and PostgreSQL
```shell
docker-compose up -d 
```

## Run Application 
```shell
cargo run --bin web
```
- Install sqlx-cli 
```shell
cargo install sqlx-cli
```

- To migrate, run below command 
```
cargo sqlx migrate run --database-url postgres://frustacean:abc123@localhost:5434/community
```

- After adding queries, run below command 
```shell
cargo sqlx prepare --database-url postgres://frustacean:abc123@localhost:5434/community 
```

## EventStoreDB Admin UI 
- [Admin UI](http://localhost:2113)

# Snapshoting 
- EventStoreDB doesn't support snapshotting.

## How to implement snapshoting
1. Events are appended to the stream `post-123`
2. Every nth event, a snapshot is appended to the stream e.g. `post_snapshot-123`
   - use `_` instead of `-` in the suffix `_snapshot`, `-` is used by the system projection 
3. Read backwards a single result from `post_snapshot-123` stream 
    ```java 
    ReadStreamOptions options = ReadStreamOptions.get()
        .backwards()
        .fromEnd();

    ReadResult result = client.readStream("post_snapshot-123", 1 /*maxCount*/, options)
        .get();

    List<ResolvedEvent> events = result.getEvents();
    ```
4. Read forwards from nth revisino from `post-123` stream 
    ```java
    ReadStreamOptions options = ReadStreamOptions.get()
        .forwards()
        .fromRevision(n);

    ReadResult result = client.readStream("post-123", options)
        .get();

    List<ResolvedEvent> events = result.getEvents();
    ```

# Links 
- [Event Sourcing with EventStoreDB](https://github.com/eugene-khyst/eventstoredb-event-sourcing)
- [EventStoreDB Rust Client](https://github.com/EventStore/EventStoreDB-Client-Rust)