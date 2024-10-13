enum ServerAction {
    CREATE,
    UPDATE,
    DELETE,
}

pub struct ServerEvent_Base {
    target_server_id: u32,
    action: ServerAction,
}

pub struct ServerEvent_CreateServer {
    base_event: ServerEvent_Base,
}
