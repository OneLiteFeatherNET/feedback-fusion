database = {
  postgres = {
    username = "postgres"
    password = "password"
    database = "postgres"
    endpoint = "localhost:5150"
  }
}

broker = {
  grpc = {
    key = "./tests/_common/configs/indexer/key.pem"
    certificate = "./tests/_common/configs/indexer/certificate.crt"
    certificate_authority = "./tests/_common/configs/indexer/certificate_authority.crt"
  }
}
