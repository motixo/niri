use knuffel::errors::DecodeError;
use niri_ipc::{ColumnDisplay, SizeChange};

use crate::appearance::{
    Border, FocusRing, InsertHint, Shadow, TabIndicator, DEFAULT_BACKGROUND_COLOR,
};
use crate::utils::{expect_only_children, Flag, MergeWith};
use crate::{BorderRule, Color, FloatOrInt, InsertHintPart, ShadowRule, TabIndicatorPart};

#[derive(Debug, Clone, PartialEq)]
pub struct Layout {
    pub ssd_titlebar: SsdTitlebar,
    pub focus_ring: FocusRing,
    pub border: Border,
    pub shadow: Shadow,
    pub tab_indicator: TabIndicator,
    pub insert_hint: InsertHint,
    pub preset_column_widths: Vec<PresetSize>,
    pub default_column_width: Option<PresetSize>,
    pub preset_window_heights: Vec<PresetSize>,
    pub center_focused_column: CenterFocusedColumn,
    pub always_center_single_column: bool,
    pub empty_workspace_above_first: bool,
    pub default_column_display: ColumnDisplay,
    pub gaps: f64,
    pub struts: Struts,
    pub background_color: Color,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            ssd_titlebar: SsdTitlebar::default(),
            focus_ring: FocusRing::default(),
            border: Border::default(),
            shadow: Shadow::default(),
            tab_indicator: TabIndicator::default(),
            insert_hint: InsertHint::default(),
            preset_column_widths: vec![
                PresetSize::Proportion(1. / 3.),
                PresetSize::Proportion(0.5),
                PresetSize::Proportion(2. / 3.),
            ],
            default_column_width: Some(PresetSize::Proportion(0.5)),
            center_focused_column: CenterFocusedColumn::Never,
            always_center_single_column: false,
            empty_workspace_above_first: false,
            default_column_display: ColumnDisplay::Normal,
            gaps: 16.,
            struts: Struts::default(),
            preset_window_heights: vec![
                PresetSize::Proportion(1. / 3.),
                PresetSize::Proportion(0.5),
                PresetSize::Proportion(2. / 3.),
            ],
            background_color: DEFAULT_BACKGROUND_COLOR,
        }
    }
}

impl MergeWith<LayoutPart> for Layout {
    fn merge_with(&mut self, part: &LayoutPart) {
        merge!(
            (self, part),
            ssd_titlebar,
            focus_ring,
            border,
            shadow,
            tab_indicator,
            insert_hint,
            always_center_single_column,
            empty_workspace_above_first,
            gaps,
        );

        merge_clone!(
            (self, part),
            preset_column_widths,
            preset_window_heights,
            center_focused_column,
            default_column_display,
            struts,
            background_color,
        );

        if let Some(x) = part.default_column_width {
            self.default_column_width = x.0;
        }

        if self.preset_column_widths.is_empty() {
            self.preset_column_widths = Layout::default().preset_column_widths;
        }

        if self.preset_window_heights.is_empty() {
            self.preset_window_heights = Layout::default().preset_window_heights;
        }
    }
}

#[derive(knuffel::Decode, Debug, Default, Clone, PartialEq)]
pub struct LayoutPart {
    #[knuffel(child)]
    pub ssd_titlebar: Option<SsdTitlebarPart>,
    #[knuffel(child)]
    pub focus_ring: Option<BorderRule>,
    #[knuffel(child)]
    pub border: Option<BorderRule>,
    #[knuffel(child)]
    pub shadow: Option<ShadowRule>,
    #[knuffel(child)]
    pub tab_indicator: Option<TabIndicatorPart>,
    #[knuffel(child)]
    pub insert_hint: Option<InsertHintPart>,
    #[knuffel(child, unwrap(children))]
    pub preset_column_widths: Option<Vec<PresetSize>>,
    #[knuffel(child)]
    pub default_column_width: Option<DefaultPresetSize>,
    #[knuffel(child, unwrap(children))]
    pub preset_window_heights: Option<Vec<PresetSize>>,
    #[knuffel(child, unwrap(argument))]
    pub center_focused_column: Option<CenterFocusedColumn>,
    #[knuffel(child)]
    pub always_center_single_column: Option<Flag>,
    #[knuffel(child)]
    pub empty_workspace_above_first: Option<Flag>,
    #[knuffel(child, unwrap(argument, str))]
    pub default_column_display: Option<ColumnDisplay>,
    #[knuffel(child, unwrap(argument))]
    pub gaps: Option<FloatOrInt<0, 65535>>,
    #[knuffel(child)]
    pub struts: Option<Struts>,
    #[knuffel(child)]
    pub background_color: Option<Color>,
}

/// Decoration drawn by niri for clients that negotiated server-side decorations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SsdTitlebar {
    pub off: bool,
    pub height: f64,
    pub button_size: f64,
    pub button_gap: f64,
    pub button_radius: f64,
    pub button_margin: f64,
    pub button_icon_size: f64,
    pub button_icon_color: Option<Color>,
    pub close_icon_color: Option<Color>,
    pub maximize_icon_color: Option<Color>,
    pub background_color: Option<Color>,
    pub inactive_background_color: Option<Color>,
    pub close_color: Option<Color>,
    pub maximize_color: Option<Color>,
    pub inactive_button_color: Option<Color>,
    pub title_color: Option<Color>,
    pub inactive_title_color: Option<Color>,
}

impl Default for SsdTitlebar {
    fn default() -> Self {
        Self {
            off: false,
            height: 28.,
            button_size: 16.,
            button_gap: 12.,
            button_radius: 8.,
            button_margin: 12.,
            button_icon_size: 10.,
            button_icon_color: None,
            close_icon_color: None,
            maximize_icon_color: None,
            background_color: None,
            inactive_background_color: None,
            close_color: None,
            maximize_color: None,
            inactive_button_color: None,
            title_color: None,
            inactive_title_color: None,
        }
    }
}

impl MergeWith<SsdTitlebarPart> for SsdTitlebar {
    fn merge_with(&mut self, part: &SsdTitlebarPart) {
        self.off |= part.off;
        if part.on {
            self.off = false;
        }
        merge!(
            (self, part),
            height,
            button_size,
            button_gap,
            button_radius,
            button_margin,
            button_icon_size,
        );
        if let Some(color) = part.button_icon_color {
            self.button_icon_color = Some(color);
        }
        if let Some(color) = part.close_icon_color {
            self.close_icon_color = Some(color);
        }
        if let Some(color) = part.maximize_icon_color {
            self.maximize_icon_color = Some(color);
        }
        if let Some(color) = part.background_color {
            self.background_color = Some(color);
        }
        if let Some(color) = part.inactive_background_color {
            self.inactive_background_color = Some(color);
        }
        if let Some(color) = part.close_color {
            self.close_color = Some(color);
        }
        if let Some(color) = part.maximize_color {
            self.maximize_color = Some(color);
        }
        if let Some(color) = part.inactive_button_color {
            self.inactive_button_color = Some(color);
        }
        if let Some(color) = part.title_color {
            self.title_color = Some(color);
        }
        if let Some(color) = part.inactive_title_color {
            self.inactive_title_color = Some(color);
        }
    }
}

#[derive(knuffel::Decode, Debug, Default, Clone, Copy, PartialEq)]
pub struct SsdTitlebarPart {
    #[knuffel(child)]
    pub off: bool,
    #[knuffel(child)]
    pub on: bool,
    #[knuffel(child, unwrap(argument))]
    pub height: Option<FloatOrInt<16, 96>>,
    #[knuffel(child, unwrap(argument))]
    pub button_size: Option<FloatOrInt<8, 64>>,
    #[knuffel(child, unwrap(argument))]
    pub button_gap: Option<FloatOrInt<0, 64>>,
    #[knuffel(child, unwrap(argument))]
    pub button_radius: Option<FloatOrInt<0, 32>>,
    #[knuffel(child, unwrap(argument))]
    pub button_margin: Option<FloatOrInt<0, 64>>,
    #[knuffel(child, unwrap(argument))]
    pub button_icon_size: Option<FloatOrInt<4, 32>>,
    #[knuffel(child)]
    pub button_icon_color: Option<Color>,
    #[knuffel(child)]
    pub close_icon_color: Option<Color>,
    #[knuffel(child)]
    pub maximize_icon_color: Option<Color>,
    #[knuffel(child)]
    pub background_color: Option<Color>,
    #[knuffel(child)]
    pub inactive_background_color: Option<Color>,
    #[knuffel(child)]
    pub close_color: Option<Color>,
    #[knuffel(child)]
    pub maximize_color: Option<Color>,
    #[knuffel(child)]
    pub inactive_button_color: Option<Color>,
    #[knuffel(child)]
    pub title_color: Option<Color>,
    #[knuffel(child)]
    pub inactive_title_color: Option<Color>,
}

#[derive(knuffel::Decode, Debug, Clone, Copy, PartialEq)]
pub enum PresetSize {
    Proportion(#[knuffel(argument)] f64),
    Fixed(#[knuffel(argument)] i32),
}

impl From<PresetSize> for SizeChange {
    fn from(value: PresetSize) -> Self {
        match value {
            PresetSize::Proportion(prop) => SizeChange::SetProportion(prop * 100.),
            PresetSize::Fixed(fixed) => SizeChange::SetFixed(fixed),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DefaultPresetSize(pub Option<PresetSize>);

#[derive(knuffel::Decode, Debug, Default, Clone, Copy, PartialEq)]
pub struct Struts {
    #[knuffel(child, unwrap(argument), default)]
    pub left: FloatOrInt<-65535, 65535>,
    #[knuffel(child, unwrap(argument), default)]
    pub right: FloatOrInt<-65535, 65535>,
    #[knuffel(child, unwrap(argument), default)]
    pub top: FloatOrInt<-65535, 65535>,
    #[knuffel(child, unwrap(argument), default)]
    pub bottom: FloatOrInt<-65535, 65535>,
}

#[derive(knuffel::DecodeScalar, Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum CenterFocusedColumn {
    /// Focusing a column will not center the column.
    #[default]
    Never,
    /// The focused column will always be centered.
    Always,
    /// Focusing a column will center it if it doesn't fit on the screen together with the
    /// previously focused column.
    OnOverflow,
}

impl<S> knuffel::Decode<S> for DefaultPresetSize
where
    S: knuffel::traits::ErrorSpan,
{
    fn decode_node(
        node: &knuffel::ast::SpannedNode<S>,
        ctx: &mut knuffel::decode::Context<S>,
    ) -> Result<Self, DecodeError<S>> {
        expect_only_children(node, ctx);

        let mut children = node.children();

        if let Some(child) = children.next() {
            if let Some(unwanted_child) = children.next() {
                ctx.emit_error(DecodeError::unexpected(
                    unwanted_child,
                    "node",
                    "expected no more than one child",
                ));
            }
            PresetSize::decode_node(child, ctx).map(Some).map(Self)
        } else {
            Ok(Self(None))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Config;

    #[test]
    fn ssd_titlebar_config_parses() {
        let config = Config::parse_mem(
            r##"
            layout {
                ssd-titlebar {
                    on
                    height 30
                    button-size 20
                    button-gap 10
                    button-radius 6
                    button-margin 14
                    button-icon-size 11
                    button-icon-color "#241e1e"
                    close-icon-color "#690005"
                    maximize-icon-color "#171212"
                    background-color "#171212"
                    close-color "#ffb4ab"
                }
            }
            "##,
        )
        .unwrap();

        assert!(!config.layout.ssd_titlebar.off);
        assert_eq!(config.layout.ssd_titlebar.height, 30.);
        assert_eq!(config.layout.ssd_titlebar.button_size, 20.);
        assert_eq!(config.layout.ssd_titlebar.button_gap, 10.);
        assert_eq!(config.layout.ssd_titlebar.button_radius, 6.);
        assert_eq!(config.layout.ssd_titlebar.button_margin, 14.);
        assert_eq!(config.layout.ssd_titlebar.button_icon_size, 11.);
        assert_eq!(
            config.layout.ssd_titlebar.button_icon_color.unwrap().r,
            36. / 255.
        );
        assert_eq!(
            config.layout.ssd_titlebar.close_icon_color.unwrap().r,
            105. / 255.
        );
        assert_eq!(
            config.layout.ssd_titlebar.maximize_icon_color.unwrap().r,
            23. / 255.
        );
        assert_eq!(
            config.layout.ssd_titlebar.background_color.unwrap().r,
            23. / 255.
        );
        assert_eq!(config.layout.ssd_titlebar.close_color.unwrap().r, 1.);
    }
}
