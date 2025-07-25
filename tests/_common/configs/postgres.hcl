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

preset = {
  targets = [
    {
      id = "target"
      name = "TestTarget"
      description = "A nice Target"
      prompts = [
        {
          id = "prompt"
          title = "Testprompt"
          description = "A nice Prompt"
          active = true
          fields = [
            {
              id = "field1"
              title = "TextField"
              field_type = "text"
              options = {
                type = "text"
                lines = 1
                placeholder = "test"
              }
            }
          ]
        }
      ]
    }
  ]
}
