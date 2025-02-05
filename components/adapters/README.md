# adapters

Um diretório contendo crates the fazem as conexões que o serviço utiliza.

* http\_in: é uma crate the implementa os handlers dos endpoints que o serviço expõe.
* rabbit\_in: é uma crate para tratar todos os eventos que um serviço do tipo _worker_ ouve.
* rabbit\_out: é uma crate responsável por emitir todos os eventos que o serviço possa publicar.
