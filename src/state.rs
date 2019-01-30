use homeassistant::Client;
use error;

#[derive(Debug, Default)]
pub struct AvrState {
    entity_id: String,
    pub mute: bool,
    pub volume: f64
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

    pub fn run(&mut self, cmd: Command) -> error::Result<()> {
        use self::Command::*;

        let entity_id = self.avr.entity_id.clone();

        match cmd {
            IncreaseVolume => {
                let state = self.client.call_service("media_player", "volume_up", EntityService::new(entity_id))?;

                match state.get(0) {
                    Some(state) => self.update_avr_state(state),
                    _ => {}
                }
            },
            DecreaseVolume => {
                let state = self.client.call_service("media_player", "volume_down", EntityService::new(entity_id))?;

                match state.get(0) {
                    Some(state) => self.update_avr_state(state),
                    _ => {}
                }
            },
            ToggleMute => {
                let state = self.client.call_service("media_player", "volume_mute", MuteService::new(entity_id, self.avr.mute))?;

                match state.get(0) {
                    Some(state) => self.update_avr_state(state),
                    _ => {}
                }
            },
            Refresh => {
                let state = self.client.get_state(&entity_id)?;
                self.update_avr_state(&state);
            }
        }

        Ok(())
    }

    fn update_avr_state(&mut self, state: &homeassistant::structs::State) {
        let volume = state.attributes.get("volume_level").and_then(|value| value.as_f64());
        let muted = state.attributes.get("is_volume_muted").and_then(|value| value.as_bool());

        if let Some(volume) = volume {
            self.avr.volume = volume;
        }
        if let Some(muted) = muted {
            self.avr.mute = muted;
        }
    }
}

#[derive(Debug)]
pub enum Command {
    Refresh,
    IncreaseVolume,
    DecreaseVolume,
    ToggleMute
}
