# Caching 

## Background 

Feedback-Fusion is able to cache often used data, which is espacially used 
on public endpoints (the endpoints where the clients send their feedback to).

The cached data persists of different database requests required to process a 
single prompt response. Utilizing this cache functionality we can provide a slightly better 
performance under load as we do not need that much database requests. 

## Caching variants 

Feedback fusion supports **2 types** of caching:
- **Memory caching:** the entire cache is hold in the server memory, therefore we cant work with multiple server instances here
- **Distributed caching:** here we utilize the speed of the **Skytable Database**. The Skytable database stores all of our cache data and all instances can update / refresh this cache at all time.

## Configuration

### Memory 

If you use the default provided image it is automatically enabled.

### Skytable 

#### Prerequisites

- A fully configured Skytable database. [Setup a skytable database](https://docs.skytable.io/installation)

#### Values 

If you set the following values feedback-fusion will automatically enable distributed caching 
via skytable 

| Environment Variable   | Type             | Default   | Description                                       |
|------------------------|------------------|-----------|---------------------------------------------------|
| `skytable_host`        | `Option<String>` | `None`    | The hostname or IP address of the Skytable server |
| `skytable_port`        | `Option<u16>`    | `None`    | The port on which the Skytable server is running  |
| `skytable_certificate` | `Option<String>` | `None`    | The Skytable servers ca certificate               |
| `skytable_username`    | `Option<String>` | `None`    | The username for authentication                   |
| `skytable_password`    | `Option<String>` | `None`    | The password for authentication                   | 
