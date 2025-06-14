database = {
  mssql = {
    username = "sa"
    password = "Password1"
    database = "master"
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
