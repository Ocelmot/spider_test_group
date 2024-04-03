use super::State;

use spider_client::message::DatasetData;
use spider_client::message::DatasetPath;
use spider_client::message::GroupMessage;
use spider_client::message::Message;
use spider_client::message::Proposal;
use spider_client::message::ProposalDatasetChange;
use spider_client::message::UiElementContent;
use spider_client::message::UiInput;
use spider_client::message::UiMessage;
use spider_client::ClientChannel;

pub(crate) async fn msg_handler(client: &mut ClientChannel, state: &mut State, msg: Message) {
    match msg {
        Message::Ui(msg) => ui_handler(client, state, msg).await,
        Message::Dataset(_) => {}
        Message::Router(_) => {}
        Message::Error(_) => {}
        Message::Group(msg) => group_handler(client, state, msg).await,
    }
}

pub(crate) async fn ui_handler(client: &mut ClientChannel, state: &mut State, msg: UiMessage) {
    match msg {
        UiMessage::Input(element_id, _, change) => {
            match element_id.as_str() {
                // Add an entry to the group dataset
                "send_message" => {
                    // add the message to the group here,
                    // change the ui in the group section
                    if let Some(group_id) = state.current_group() {
                        if let UiInput::Text(msg_text) = change {
                            let dataset = DatasetPath::new_private(vec!["msgs".to_string()]);
                            let data = DatasetData::String(msg_text);
                            let dataset_change = ProposalDatasetChange::AppendData(data);
                            let proposal = Proposal::propose(dataset, dataset_change);
                            let msg = GroupMessage::Propose(group_id.clone(), proposal);
                            let msg = Message::Group(msg);
                            client.send(msg).await;
                        }
                    }
                }

                _ => return,
            }

            // send updates
            let changes = state.test_page.get_changes();
            let msg = Message::Ui(UiMessage::UpdateElements(changes));
            client.send(msg).await;
        }
        _ => {}
    }
}

pub(crate) async fn group_handler(
    client: &mut ClientChannel,
    state: &mut State,
    msg: GroupMessage,
) {
    match msg {
        GroupMessage::Proposal(id, proposal) => {
            if let Some(current_group) = state.current_group {
                if id != current_group {
                    return;
                }
            } else {
                return;
            }
            let (path, change) = proposal.to_parts();
            if path != DatasetPath::new_private(vec![String::from("msgs")]) {
                return;
            }
            match change {
                ProposalDatasetChange::AddMember(_) => {}
                ProposalDatasetChange::RemoveMember(_) => {}
                ProposalDatasetChange::SetMetadata(_) => {}
                ProposalDatasetChange::ClearMetadata => {}
                ProposalDatasetChange::SetData(index, value) => {
                    if let Some(entry) = state.entries.get_mut(index) {
                        *entry = value;
                        update_entries(client, state).await;
                    }
                }
                ProposalDatasetChange::AppendData(value) => {
                    state.entries.push(value);
                    update_entries(client, state).await;
                }
                ProposalDatasetChange::RemoveData(index) => {
                    state.entries.remove(index);
                    update_entries(client, state).await;
                }
            }
        }
        _ => {}
    }
}

async fn update_entries(client: &mut ClientChannel, state: &mut State) {
    let mut new_string = String::new();
    for entry in &state.entries {
        new_string.push('\n');
        new_string.push_str(&entry.to_string());
    }
    if let Some(mut element) = state.test_page.get_by_id_mut("msgs") {
        element.set_content(UiElementContent::new_text(new_string));
    }
    // send updates
    let changes = state.test_page.get_changes();
    let msg = Message::Ui(UiMessage::UpdateElements(changes));
    client.send(msg).await;
}
