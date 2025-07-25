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

cache = {
  skytable = {
    host = "localhost"
    port = 2003
    username = "root"
    password = "passwordpassword"
  }
}
