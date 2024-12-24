use bevy::prelude::*;
use iyes_perf_ui::prelude::PerfUiAllEntries;
use mechanics::{Atom, AtomHitbox};

use crate::{
    AtomicMassText, AtomicNumberText, ElementCard, ElementElectronsText,
    ElementNameText, ElementNeutronsText, ElementSymbolText, HudCamera,
};

pub fn setup_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let jbm_regular: Handle<Font> =
        asset_server.load("fonts/JetBrainsMonoNerdFont-Regular.ttf");
    let jbm_bold: Handle<Font> =
        asset_server.load("fonts/JetBrainsMonoNerdFont-Bold.ttf");

    let hud_camera = commands
        .spawn((
            Camera2d,
            Camera {
                order: 1,
                clear_color: ClearColorConfig::None,
                ..default()
            },
            HudCamera,
        ))
        .id();

    commands.spawn((TargetCamera(hud_camera), PerfUiAllEntries::default()));

    commands
        .spawn((
            TargetCamera(hud_camera),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            PickingBehavior::IGNORE,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(275.0),
                        height: Val::Px(275.0),
                        border: UiRect::all(Val::Px(10.0)),
                        padding: UiRect::axes(Val::Px(15.0), Val::Px(10.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    BorderRadius::all(Val::Px(30.0)),
                    BackgroundColor(Color::srgba_u8(69, 71, 90, 200)),
                    Visibility::Hidden,
                    ElementCard,
                ))
                .with_children(|parent| {
                    parent
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                Text::default(),
                                TextFont {
                                    font: jbm_regular.clone(),
                                    font_size: 36.0,
                                    ..Default::default()
                                },
                                TextColor(Color::srgb_u8(180, 190, 254)),
                                AtomicNumberText,
                            ));

                            parent.spawn((
                                Text::default(),
                                TextFont {
                                    font: jbm_regular.clone(),
                                    font_size: 36.0,
                                    ..Default::default()
                                },
                                TextColor(Color::srgb_u8(180, 190, 254)),
                                AtomicMassText,
                            ));
                        });

                    parent
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                Text::default(),
                                TextFont {
                                    font: jbm_bold.clone(),
                                    font_size: 96.0,
                                    ..Default::default()
                                },
                                TextColor(Color::srgb_u8(180, 190, 254)),
                                ElementSymbolText,
                            ));
                        });

                    parent
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                Text::default(),
                                TextFont {
                                    font: jbm_regular.clone(),
                                    font_size: 36.0,
                                    ..Default::default()
                                },
                                Node {
                                    justify_self: JustifySelf::Center,
                                    ..default()
                                },
                                TextColor(Color::srgb_u8(180, 190, 254)),
                                ElementNameText,
                            ));
                        });
                });
        });
}

pub fn update_hud(
    mut query_element_name_text: Query<
        (&mut Text, &mut TextFont),
        (
            With<ElementNameText>,
            Without<ElementSymbolText>,
            Without<AtomicNumberText>,
            Without<AtomicMassText>,
            Without<ElementNeutronsText>,
            Without<ElementElectronsText>,
        ),
    >,
    mut query_atomic_number_text: Query<
        &mut Text,
        (
            With<AtomicNumberText>,
            Without<ElementSymbolText>,
            Without<ElementNameText>,
            Without<AtomicMassText>,
            Without<ElementNeutronsText>,
            Without<ElementElectronsText>,
        ),
    >,
    mut query_atomic_mass_text: Query<
        &mut Text,
        (
            With<AtomicMassText>,
            Without<ElementSymbolText>,
            Without<ElementNameText>,
            Without<AtomicNumberText>,
            Without<ElementNeutronsText>,
            Without<ElementElectronsText>,
        ),
    >,
    mut query_element_symbol_text: Query<
        (&mut Text, &mut TextColor),
        (
            With<ElementSymbolText>,
            Without<ElementNameText>,
            Without<AtomicNumberText>,
            Without<AtomicMassText>,
            Without<ElementNeutronsText>,
            Without<ElementElectronsText>,
        ),
    >,
    mut query_element_card: Query<&mut Visibility, With<ElementCard>>,
    query_atoms: Query<(&Atom, &AtomHitbox)>,
) {
    let Ok(mut element_card_visibility) = query_element_card.get_single_mut()
    else {
        return;
    };

    let Some((atom, _hitbox)) =
        query_atoms.iter().find(|(_atom, hitbox)| hitbox.selected)
    else {
        *element_card_visibility = Visibility::Hidden;
        return;
    };

    *element_card_visibility = Visibility::Inherited;

    if let Ok((mut text, mut font)) = query_element_name_text.get_single_mut() {
        text.0 = format!("{}", atom.element);
        let char_count = atom.element.to_string().len();
        font.font_size = if char_count > 10 {
            36.0 - (char_count - 10) as f32 * 2.5
        } else {
            36.0
        };
    };

    if let Ok(mut text) = query_atomic_number_text.get_single_mut() {
        text.0 = format!("{}", atom.proton_count);
    };

    if let Ok(mut text) = query_atomic_mass_text.get_single_mut() {
        text.0 = format!("{:3.2}", atom.mass());
    };

    if let Ok((mut text, mut color)) =
        query_element_symbol_text.get_single_mut()
    {
        text.0 = format!("{}", atom.element.symbol());
        color.0 = atom.element.color();
    };
}
