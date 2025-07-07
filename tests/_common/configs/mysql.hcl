database = {
  mysql = {
    username = "username"
    password = "password"
    database = "database"
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
