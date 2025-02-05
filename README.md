# rust-service-template

Um repositório com um template para a estrutura de diretórios e códigos
de um serviço em rust.

## Diretórios principais

* [api](api): Uma crate de aplicação para o caso de um serviço do tipo API.
* [worker](worker): Uma crate de aplicação para o caso de um serviço do tipo worker.
* [components](components): Um diretório contendo crates diversas próprias ao serviço.

### Components

* [adapters](components/adapters/README.md): Um subdiretório com crates do meio
  que o serviço faz uso.

* [infra](components/infra): Uma crate que implementa a base do serviço. Suas
  configurações, setup de log, setup de tracing. Além disso, exporta, através do
  _factories_ API para inicialização do serviço (tanto api quanto worker).

* [kernel](components/kernel): Crate principal do serviço, onde são definidos os
  contratos (traits) de um handler/endpoint, estruturas de domínio e as implementações
  efetivas dos handlers.
