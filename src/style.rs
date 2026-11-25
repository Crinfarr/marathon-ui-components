use std::{
    collections::{BTreeMap, HashMap},
    sync::LazyLock,
};

use egui::{
    Color32 as C32, CornerRadius,
    FontFamily::{Monospace as Mono, Proportional as Propo},
    FontId, Margin, Shadow, Spacing, Stroke, Style, TextStyle, Visuals,
    style::{ScrollStyle, Selection, WidgetVisuals, Widgets},
};

pub static COLORS: LazyLock<HashMap<&'static str, C32>> = LazyLock::new(|| {
    const R_TINT: u8 = 0x00;
    const G_TINT: u8 = 0x00;
    const B_TINT: u8 = 0x00;
    HashMap::from([
        (
            "black",
            C32::from_rgb(0x05 + R_TINT, 0x05 + G_TINT, 0x05 + B_TINT),
        ),
        (
            "panel",
            C32::from_rgb(0x0a + R_TINT, 0x0a + G_TINT, 0x0a + B_TINT),
        ),
        (
            "surface",
            C32::from_rgb(
                0x12 + (2 * R_TINT),
                0x12 + (2 * G_TINT),
                0x12 + (2 * B_TINT),
            ),
        ),
        (
            "raised",
            C32::from_rgb(
                0x1c + (4 * R_TINT),
                0x1c + (4 * G_TINT),
                0x1c + (4 * B_TINT),
            ),
        ),
        (
            "line",
            C32::from_rgb(
                0x32 + (6 * R_TINT),
                0x32 + (6 * G_TINT),
                0x32 + (6 * B_TINT),
            ),
        ),
        (
            "line_bright",
            C32::from_rgb(
                0x32 + (8 * R_TINT),
                0x32 + (8 * G_TINT),
                0x32 + (8 * B_TINT),
            ),
        ),
        (
            "text",
            C32::from_rgb(
                0xe6 + (4 * R_TINT),
                0xe6 + (4 * G_TINT),
                0xe6 + (4 * B_TINT),
            ),
        ),
        (
            "text_bright",
            C32::from_rgb(
                0x84 + (8 * R_TINT),
                0x84 + (8 * G_TINT),
                0x84 + (8 * B_TINT),
            ),
        ),
        ("primary", C32::from_rgb(0xc2, 0xfe, 0x0c)), // acid green
        ("accent", C32::from_rgb(0x00, 0xf0, 0xff)),  // cyan
        ("warn", C32::from_rgb(0xff, 0x6a, 0x00)),    // orange
        ("error", C32::from_rgb(0xff, 0x2a, 0x2a)),   // red
        //named colors
        ("green", C32::from_rgb(0x2a, 0xff, 0x2a)),
        ("blue", C32::from_rgb(0x2a, 0x2a, 0xff)),
        ("magenta", C32::from_rgb(0xff, 0x2e, 0x93)),
    ])
});

pub static STYLE: LazyLock<Style> = LazyLock::new(|| Style {
    text_styles: BTreeMap::from([
        (TextStyle::Heading, FontId::new(24., Propo)),
        (TextStyle::Body, FontId::new(14., Propo)),
        (TextStyle::Button, FontId::new(13., Mono)),
        (TextStyle::Small, FontId::new(10.5, Mono)),
        (TextStyle::Monospace, FontId::new(13., Mono)),
    ]),
    spacing: Spacing {
        button_padding: (14., 6.).into(),
        combo_width: 140.,
        indent: 16.,
        interact_size: (40., 24.).into(),
        item_spacing: (8., 6.).into(),
        menu_margin: Margin::same(6),
        scroll: ScrollStyle {
            bar_width: 6.,
            floating: false,
            ..Default::default()
        },
        slider_width: 140.,
        window_margin: Margin::same(12),
        ..Default::default()
    },
    visuals: Visuals {
        button_frame: true,
        code_bg_color: *COLORS.get("surface").unwrap(),
        collapsing_header_frame: true,
        dark_mode: true,
        error_fg_color: *COLORS.get("error").unwrap(),
        extreme_bg_color: C32::BLACK,
        faint_bg_color: *COLORS.get("panel").unwrap(),
        hyperlink_color: *COLORS.get("accent").unwrap(),
        indent_has_left_vline: true,
        menu_corner_radius: CornerRadius::ZERO,
        panel_fill: *COLORS.get("panel").unwrap(),
        popup_shadow: Shadow::NONE,
        resize_corner_size: 10.,
        selection: Selection {
            bg_fill: *COLORS.get("primary").unwrap(),
            stroke: Stroke::new(1., *COLORS.get("black").unwrap()),
        },
        slider_trailing_fill: true,
        striped: true,
        warn_fg_color: *COLORS.get("warn").unwrap(),
        widgets: Widgets {
            active: WidgetVisuals {
                bg_fill: *COLORS.get("accent").unwrap(),
                weak_bg_fill: *COLORS.get("accent").unwrap(),
                bg_stroke: Stroke::new(1., *COLORS.get("accent").unwrap()),
                corner_radius: CornerRadius::ZERO,
                fg_stroke: Stroke::new(2., *COLORS.get("black").unwrap()),
                expansion: 0.,
            },
            hovered: WidgetVisuals {
                bg_fill: *COLORS.get("primary").unwrap(),
                weak_bg_fill: *COLORS.get("primary").unwrap(),
                bg_stroke: Stroke::new(1., *COLORS.get("primary").unwrap()),
                corner_radius: CornerRadius::ZERO,
                fg_stroke: Stroke::new(1.5, *COLORS.get("black").unwrap()),
                expansion: 1.,
            },
            inactive: WidgetVisuals {
                bg_fill: *COLORS.get("raised").unwrap(),
                weak_bg_fill: *COLORS.get("raised").unwrap(),
                bg_stroke: Stroke::new(1., *COLORS.get("line_bright").unwrap()),
                corner_radius: CornerRadius::ZERO,
                fg_stroke: Stroke::new(1., *COLORS.get("text").unwrap()),
                expansion: 0.,
            },
            noninteractive: WidgetVisuals {
                bg_fill: *COLORS.get("surface").unwrap(),
                weak_bg_fill: *COLORS.get("surface").unwrap(),
                bg_stroke: Stroke::new(1., *COLORS.get("line").unwrap()),
                corner_radius: CornerRadius::ZERO,
                fg_stroke: Stroke::new(1., *COLORS.get("text").unwrap()),
                expansion: 0.,
            },
            open: WidgetVisuals {
                bg_fill: *COLORS.get("surface").unwrap(),
                weak_bg_fill: *COLORS.get("surface").unwrap(),
                bg_stroke: Stroke::new(1., *COLORS.get("primary").unwrap()),
                corner_radius: CornerRadius::ZERO,
                fg_stroke: Stroke::new(1., *COLORS.get("primary").unwrap()),
                expansion: 0.,
            },
            ..Default::default()
        },
        window_corner_radius: CornerRadius::ZERO,
        window_fill: *COLORS.get("surface").unwrap(),
        window_highlight_topmost: true,
        window_shadow: Shadow::NONE,
        window_stroke: Stroke::new(1., *COLORS.get("line_bright").unwrap()),
        ..Visuals::dark()
    },
    ..Default::default()
});
