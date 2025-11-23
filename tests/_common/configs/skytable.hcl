database = {
  postgres = {
    username = "postgres"
    password = "password"
    database = "postgres"
    endpoint = "localhost:5150"
  }
}

oidc = {
  provider = "http://localhost:5151"
  issuer   = "http://localhost:5151"

  scopes = [
    {
      name = "api:feedback-fusion"
      grants = [
        {
          endpoint = {
            Custom = ["*", "All"]
          }
          permissions = ["All"]
        }
      ]
    }
  ]

  groups = [
    {
      name = "admin"
      grants = [
        {
          endpoint = {
            Custom = ["*", "All"]
          }
          permissions = ["All"]
        }
      ]
    }
  ]
}

broker = {
  grpc = {
    endpoint = "https://localhost:7000"

    tls = {
      key = "./tests/_common/configs/indexer/key.pem"
      certificate = "./tests/_common/configs/indexer/certificate.crt"
      certificate_authority = "./tests/_common/configs/indexer/certificate_authority.crt"
    } 
  }

  batch_interval = 0
}

cache = {
  skytable = {
    host = "localhost"
    port = 2003
    username = "root"
    password = "passwordpassword"
  }
}
