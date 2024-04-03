use spider_client::{message::{UiElement, UiElementKind, UiPageManager, UiPath}, SpiderId2048};


pub fn build_ui(id: SpiderId2048) -> UiPageManager {
    let mut test_page = UiPageManager::new(id, "Test Group");
    let mut root = test_page
        .get_element_mut(&UiPath::root())
        .expect("all pages have a root");
    root.set_kind(UiElementKind::Columns);

    root.append_child({
        // Left col is controls
        let mut left_col_rows = UiElement::new(UiElementKind::Rows);
        
        left_col_rows.append_child({
            let mut send_message = UiElement::from_string("Send Message");
            send_message.set_kind(UiElementKind::TextEntry);
            send_message.set_selectable(true);
            send_message.set_id("send_message");
            send_message
        });
        
        left_col_rows
    });

    // Right col is list of data entries
    root.append_child({
        let mut msgs = UiElement::from_string("");
        msgs.set_id("msgs");
        msgs
    });

    drop(root);
    test_page
}


