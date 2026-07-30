use school_erp::web::ui::{Button, Icon, Card, Render};

#[test]
fn test_button_render() {
    let btn = Button::new("Click Me")
        .variant("primary")
        .size("lg")
        .render();
    
    assert!(btn.contains("<ui-button"));
    assert!(btn.contains("variant=\"primary\""));
    assert!(btn.contains("size=\"lg\""));
    assert!(btn.contains("Click Me"));
    assert!(btn.contains("</ui-button>"));
}

#[test]
fn test_icon_render() {
    let icon = Icon::new("star")
        .size("24")
        .color("gold")
        .render();
    
    assert!(icon.contains("<ui-icon"));
    assert!(icon.contains("name=\"star\""));
    assert!(icon.contains("size=\"24\""));
    assert!(icon.contains("style=\"color: gold;\""));
}

#[test]
fn test_card_render() {
    let card = Card::new("Hello World")
        .title("Greetings")
        .render();
    
    assert!(card.contains("<ui-card>"));
    assert!(card.contains("<div slot=\"header\">Greetings</div>"));
    assert!(card.contains("Hello World"));
    assert!(card.contains("</ui-card>"));
}
