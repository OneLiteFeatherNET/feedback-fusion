database = {
  postgres = {
    username = "postgres"
    password = "password"
    database = "postgres"
    endpoint = "localhost:5150"
  }
}

broker = {
  fluvio = {
    topic = "feedback-fusion"

    fluvio = {
      endpoint = "127.0.0.1:9003"
      use_spu_local_address = false

      tls = {
        tls_policy = "disabled"
      }
    }
  } 
}
