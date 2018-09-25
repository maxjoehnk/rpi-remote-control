use homeassistant::Client;

#[derive(Debug, Default)]
pub struct AvrState {
    entity_id: String,
    pub mute: bool,
    pub volume: i32
}

#[derive(Serialize)]
struct EntityService {
    entity_id: String
}

impl EntityService {
    fn new<S: Into<String>>(entity_id: S) -> EntityService {
        EntityService {
            entity_id: entity_id.into()
        }
    }
}

#[derive(Serialize)]
struct MuteService {
    entity_id: String,
    is_volume_muted: bool
}

impl MuteService {
    fn new<S: Into<String>>(entity_id: S, mute: bool) -> MuteService {
        MuteService {
            entity_id: entity_id.into(),
            is_volume_muted: mute
        }
    }
}

pub struct ApplicationState {
    client: Client,
    pub avr: AvrState
}

impl ApplicationState {
    pub fn new<A: Into<String>, B: Into<String>, C: Into<String>>(ha_url: A, ha_token: B, avr_entity_id: C) -> ApplicationState {
        ApplicationState {
            client: Client::new(ha_url.into(), Some(ha_token.into())),
            avr: AvrState {
                entity_id: avr_entity_id.into(),
                ..AvrState::default()
            }
        }
    }

    pub fn run(&mut self, cmd: Command) {
        use self::Command::*;

        let entity_id = self.avr.entity_id.clone();

        match cmd {
            IncreaseVolume => {
                self.client.call_service("media_player", "volume_up", EntityService::new(entity_id));
            },
            DecreaseVolume => {
                self.client.call_service("media_player", "volume_down", EntityService::new(entity_id));
            },
            ToggleMute => {
                self.client.call_service("media_player", "volume_mute", MuteService::new(entity_id, !self.avr.mute));
            }
        }
    }
}

#[derive(Debug)]
pub enum Command {
    IncreaseVolume,
    DecreaseVolume,
    ToggleMute
}
