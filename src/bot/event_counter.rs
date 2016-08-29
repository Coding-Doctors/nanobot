use discord::model::Event;
use std::collections::{BTreeMap, HashMap};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EventType {
    Any,
    CallCreate,
    CallDelete,
    CallUpdate,
    ChannelCreate,
    ChannelDelete,
    ChannelPinsAck,
    ChannelPinsUpdate,
    ChannelRecipientAdd,
    ChannelRecipientRemove,
    ChannelUpdate,
    MessageAck,
    MessageCreate,
    MessageDelete,
    MessageUpdate,
    PresenceUpdate,
    PresencesReplace,
    Ready,
    RelationshipAdd,
    RelationshipRemove,
    Resumed,
    ServerBanAdd,
    ServerBanRemove,
    ServerCreate,
    ServerDelete,
    ServerEmojisUpdate,
    ServerMemberAdd,
    ServerMemberRemove,
    ServerMemberUpdate,
    ServerMembersChunk,
    ServerIntegrationsUpdate,
    ServerRoleCreate,
    ServerRoleDelete,
    ServerRoleUpdate,
    ServerSync,
    ServerUpdate,
    TypingStart,
    Unknown,
    UserNoteUpdate,
    UserServerSettingsUpdate,
    UserSettingsUpdate,
    UserUpdate,
    VoiceServerUpdate,
    VoiceStateUpdate,
}

pub struct EventCounter {
    counter: HashMap<EventType, u64>,
}

pub fn event_types() -> [EventType; 43] {
    [
        EventType::CallCreate,
        EventType::CallDelete,
        EventType::CallUpdate,
        EventType::ChannelCreate,
        EventType::ChannelDelete,
        EventType::ChannelPinsAck,
        EventType::ChannelPinsUpdate,
        EventType::ChannelRecipientAdd,
        EventType::ChannelRecipientRemove,
        EventType::ChannelUpdate,
        EventType::MessageAck,
        EventType::MessageCreate,
        EventType::MessageDelete,
        EventType::MessageUpdate,
        EventType::PresenceUpdate,
        EventType::PresencesReplace,
        EventType::Ready,
        EventType::RelationshipAdd,
        EventType::RelationshipRemove,
        EventType::Resumed,
        EventType::ServerBanAdd,
        EventType::ServerBanRemove,
        EventType::ServerCreate,
        EventType::ServerDelete,
        EventType::ServerEmojisUpdate,
        EventType::ServerMemberAdd,
        EventType::ServerMemberRemove,
        EventType::ServerMemberUpdate,
        EventType::ServerMembersChunk,
        EventType::ServerIntegrationsUpdate,
        EventType::ServerRoleCreate,
        EventType::ServerRoleDelete,
        EventType::ServerRoleUpdate,
        EventType::ServerSync,
        EventType::ServerUpdate,
        EventType::TypingStart,
        EventType::Unknown,
        EventType::UserNoteUpdate,
        EventType::UserServerSettingsUpdate,
        EventType::UserSettingsUpdate,
        EventType::UserUpdate,
        EventType::VoiceServerUpdate,
        EventType::VoiceStateUpdate,
    ]
}

impl EventCounter {
    pub fn new() -> EventCounter {
        EventCounter {
            counter: HashMap::new(),
        }
    }

    fn increment_type(&mut self, event_type: EventType) {
        let entry = self.counter.entry(event_type).or_insert(0);
        *entry += 1;
    }

    pub fn increment(&mut self, event: &Event) {
        self.increment_type(EventType::Any);

        self.increment_type(match *event {
            Event::CallCreate(_) => EventType::CallCreate,
            Event::CallDelete(_) => EventType::CallDelete,
            Event::CallUpdate { .. } => EventType::CallUpdate,
            Event::ChannelCreate(_) => EventType::ChannelCreate,
            Event::ChannelDelete(_) => EventType::ChannelDelete,
            Event::ChannelPinsAck { .. } => EventType::ChannelPinsAck,
            Event::ChannelPinsUpdate { .. } => EventType::ChannelPinsUpdate,
            Event::ChannelRecipientAdd(_, _) => EventType::ChannelRecipientAdd,
            Event::ChannelRecipientRemove(_, _) => EventType::ChannelRecipientRemove,
            Event::ChannelUpdate(_) => EventType::ChannelUpdate,
            Event::MessageAck { .. } => EventType::MessageAck,
            Event::MessageCreate(_) => EventType::MessageCreate,
            Event::MessageDelete { .. } => EventType::MessageDelete,
            Event::MessageUpdate { .. } => EventType::MessageUpdate,
            Event::PresenceUpdate { .. } => EventType::PresenceUpdate,
            Event::PresencesReplace(_) => EventType::PresencesReplace,
            Event::Ready(_) => EventType::Ready,
            Event::RelationshipAdd(_) => EventType::RelationshipAdd,
            Event::RelationshipRemove(_, _) => EventType::RelationshipRemove,
            Event::Resumed { .. } => EventType::Resumed,
            Event::ServerBanAdd(_, _) => EventType::ServerBanAdd,
            Event::ServerBanRemove(_, _) => EventType::ServerBanRemove,
            Event::ServerCreate(_) => EventType::ServerCreate,
            Event::ServerDelete(_) => EventType::ServerDelete,
            Event::ServerEmojisUpdate(_, _) => EventType::ServerEmojisUpdate,
            Event::ServerIntegrationsUpdate(_) => EventType::ServerIntegrationsUpdate,
            Event::ServerMemberAdd(_, _) => EventType::ServerMemberAdd,
            Event::ServerMemberRemove(_, _) => EventType::ServerMemberRemove,
            Event::ServerMemberUpdate { .. } => EventType::ServerMemberUpdate,
            Event::ServerMembersChunk(_, _) => EventType::ServerMembersChunk,
            Event::ServerRoleCreate(_, _) => EventType::ServerRoleCreate,
            Event::ServerRoleDelete(_, _) => EventType::ServerRoleDelete,
            Event::ServerRoleUpdate(_, _) => EventType::ServerRoleUpdate,
            Event::ServerSync { .. } => EventType::ServerSync,
            Event::ServerUpdate(_) => EventType::ServerUpdate,
            Event::TypingStart { .. } => EventType::TypingStart,
            Event::Unknown(_, _) => EventType::Unknown,
            Event::UserNoteUpdate(_, _) => EventType::UserNoteUpdate,
            Event::UserServerSettingsUpdate(_) => EventType::UserServerSettingsUpdate,
            Event::UserSettingsUpdate { .. } => EventType::UserSettingsUpdate,
            Event::UserUpdate(_) => EventType::UserUpdate,
            Event::VoiceServerUpdate { .. } => EventType::VoiceServerUpdate,
            Event::VoiceStateUpdate(_, _) => EventType::VoiceStateUpdate,
            Event::__Nonexhaustive => return,
        });
    }

    #[allow(or_fun_call)]
    pub fn map(&self, kinds: Vec<EventType>) -> BTreeMap<u64, Vec<String>> {
        let mut map: BTreeMap<u64, Vec<String>> = BTreeMap::new();

        for kind in kinds {
            if let Some(amount) = self.counter.get(&kind) {
                let entry = map.entry(*amount).or_insert(vec![]);
                entry.push(format!("{:?}", kind));
            }
        }

        map
    }
}
