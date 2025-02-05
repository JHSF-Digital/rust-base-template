# rust-service-template

Um repositório com um template para a estrutura de diretórios e códigos
de um serviço em rust.

## Diretórios principais

* [api](api): Uma crate de aplicação para o caso de um serviço do tipo API.
* [worker](worker): Uma crate de aplicação para o caso de um serviço do tipo worker.
* [components](components): Um diretório contendo crates diversas próprias ao serviço.

### Components

* [adapters](components/adapters): Um subdiretório com crates do meio que o serviço faz uso.
* [infra](components/infra)
* [kernel](components/kernel)
