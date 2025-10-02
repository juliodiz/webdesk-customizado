// Configuração personalizada do WebDesk
pub fn get_webdesk_server_config() -> (String, String, String) {
    (
        "webdesk.webspot.com.br:21115".to_string(),  // rendezvous
        "webdesk.webspot.com.br:21116".to_string(),  // relay  
        "zt6UorL6EOviO9s+QNS55HL5XIxJ2aDXzuK2dXT+oaQ=".to_string() // key
    )
}
