use std::path::PathBuf;

use spider_client::{
    message::{DatasetData, GroupId, Message, RouterMessage, UiMessage, UiPageManager},
    ClientChannel, ClientResponse, SpiderClientBuilder,
};

mod handlers;
use handlers::msg_handler;

mod ui;
use ui::build_ui;

struct State {
    test_page: UiPageManager,

    current_group: Option<GroupId>,
    entries: Vec<DatasetData>,
}

impl State {
    async fn init(client: &mut ClientChannel) -> Self {
        let msg = RouterMessage::SetIdentityProperty("name".into(), "Test Group".into());
        let msg = Message::Router(msg);
        client.send(msg).await;

        let id = client.id().clone();

        let mut test_page = build_ui(id);
        test_page.get_changes(); // clear changes to synch, since we are going to send the whole page at first. This
                                 // Could instead set the initial elements with raw and then recalculate ids
        let msg = Message::Ui(UiMessage::SetPage(test_page.get_page().clone()));
        client.send(msg).await;

        Self {
            test_page,
            current_group: None,
            entries: Vec::new(),
        }
    }

    pub fn current_group(&self) -> Option<&GroupId> {
        self.current_group.as_ref()
    }
}

#[tokio::main]
async fn main() {
    let client_path = PathBuf::from("client_state.dat");

    let mut builder = SpiderClientBuilder::load_or_set(&client_path, |builder| {
        builder.enable_fixed_addrs(true);
        builder.set_fixed_addrs(vec!["localhost:1930".into()]);
    });

    builder.try_use_keyfile("spider_keyfile.json").await;

    let mut client_channel = builder.start(true);
    let mut state = State::init(&mut client_channel).await;

    loop {
        match client_channel.recv().await {
            Some(ClientResponse::Message(msg)) => {
                msg_handler(&mut client_channel, &mut state, msg).await;
            }
            Some(ClientResponse::Denied(_)) => break,
            None => break, //  done!
            _ => {}
        }
    }
}
