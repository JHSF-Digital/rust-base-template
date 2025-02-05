
// handler do axum para o endpoint
// deveria ser algo assim:
// pub async fn handler_one(
//      Json(request): Json<inbound::handler_one::HandlerOneRequest>,
// ) -> Result<outbound::handler_one::HandlerOneResponse, ServiceError> {}
pub async fn handler_one() {}
