use crate::prelude::*;

#[derive(Resource, Clone, Debug, Reflect)]
pub struct TownUiState {
    pub town: Entity,
}

impl TownUiState {
    pub fn new(town: Entity) -> Self {
        TownUiState { town }
    }
}

pub fn setup_town_uis(world: &mut World) {
    let events = world.get_resource::<Events<OpenTownEvent>>().unwrap();
    let Some(town) = events
        .get_cursor()
        .read(events)
        .next()
        .map(|e| e.entity.clone())
    else {
        return;
    };
    world.insert_resource(TownUiState { town });
    let town_name = world
        .get::<Town>(town)
        .map(|town| town.name.clone())
        .unwrap_or_else(|| "Unknown Town".to_string());
    println!("Opening town UI for {:?}", town_name);
    town_ui(&town_name).spawn(world);
}

fn town_ui(town_name: &str) -> impl Element {
    El::<Node>::new()
        .width(Val::Percent(50.))
        .height(Val::Percent(80.))
        .align_content(Align::new().center_x())
        .child(
            Column::<Node>::new()
                .item(title_card(town_name))
                .item(town_controls()),
        )
}

fn title_card(town_name: &str) -> impl Element {
    El::<Node>::new()
        .width(Val::Percent(100.))
        .height(Val::Px(40.))
        .align_content(Align::center())
        .child(
            El::<Text>::new()
                .text(Text::new(town_name))
                .text_color(TextColor(Color::WHITE)),
        )
}

fn town_controls() -> impl Element {
    Grid::<Node>::new()
        .width(Val::Percent(100.))
        .height(Val::Auto)
        .background_color(BackgroundColor(Color::srgb(0., 1., 0.)))
        .align_content(Align::center())
        .with_node(|mut node| node.grid_template_columns = vec![GridTrack::fr(1.); 2])
        .cell(
            El::<Text>::new()
                .width(Val::Percent(100.))
                .height(Val::Percent(100.))
                .background_color(BackgroundColor(Color::srgb(0., 0., 1.)))
                .text_layout(TextLayout::new_with_justify(JustifyText::Center))
                .text(Text::new("Left"))
                .text_color(TextColor(Color::WHITE)),
        )
        .cell(
            El::<Text>::new()
                .width(Val::Percent(100.))
                .height(Val::Percent(100.))
                .background_color(BackgroundColor(Color::srgb(1., 0., 0.)))
                .text_layout(TextLayout::new_with_justify(JustifyText::Center))
                .text(Text::new("Right"))
                .text_color(TextColor(Color::WHITE)),
        )
}
