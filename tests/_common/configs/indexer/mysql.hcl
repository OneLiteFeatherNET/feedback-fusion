database = {
  mysql = {
    username = "username"
    password = "password"
    database = "database"
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
