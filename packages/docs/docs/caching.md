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

```hcl
cache = {
  skytable = {
    host        = "skytable.example.com"
    port        = 2003
    certificate = <<-EOT
-----BEGIN CERTIFICATE-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...
-----END CERTIFICATE-----
EOT
    username    = "skytable_user"
    password    = "skytable_password"
    space       = "cache"
    model       = "feedbackfusion"
  }
}

```

##### Reference

| Parameter   | Description                        | Default        | Data Type |
|-------------|------------------------------------|----------------|-----------|
| host        | Hostname of the Skytable server    | N/A            | String    |
| port        | Port of the Skytable server        | 2003           | Integer   |
| certificate | PEM certificate for Skytable       | N/A            | String    |
| username    | Username for Skytable authentication | N/A          | String    |
| password    | Password for Skytable authentication | N/A          | String    |
| space       | Skytable space to use              | "cache"        | String    |
| model       | Skytable model to use              | "feedbackfusion" | String    |
