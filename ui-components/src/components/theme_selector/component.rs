use leptos::*;
use crate::theme::{
    Theme, Season, Mode,
    themes::{
        get_spring_light_theme,
        get_spring_dark_theme,
        get_summer_light_theme,
        get_summer_dark_theme
    }
};
use web_sys::{MouseEvent, KeyboardEvent};

#[component]
pub fn ThemeSelector(
    #[prop(into)] current_theme: Signal<Theme>,
    #[prop(into)] on_switch: Callback<Theme>,
) -> impl IntoView {
    let mode_icon = move || {
        match current_theme.get().mode {
            Mode::Light => "🌞",
            Mode::Dark => "🌙",
        }
    };

    let toggle_mode = move |_: MouseEvent| {
        let current = current_theme.get();
        let new_mode = match current.mode {
            Mode::Light => Mode::Dark,
            Mode::Dark => Mode::Light,
        };
        let new_theme = match (current.season, new_mode) {
            (Season::Spring, Mode::Light) => get_spring_light_theme(),
            (Season::Spring, Mode::Dark) => get_spring_dark_theme(),
            (Season::Summer, Mode::Light) => get_summer_light_theme(),
            (Season::Summer, Mode::Dark) => get_summer_dark_theme(),
        };
        on_switch.call(new_theme);
    };

    let handle_season_switch = move |new_season: Season| {
        let current_mode = current_theme.get().mode;
        let new_theme = match (new_season, current_mode) {
            (Season::Spring, Mode::Light) => get_spring_light_theme(),
            (Season::Spring, Mode::Dark) => get_spring_dark_theme(),
            (Season::Summer, Mode::Light) => get_summer_light_theme(),
            (Season::Summer, Mode::Dark) => get_summer_dark_theme(),
        };
        on_switch.call(new_theme);
    };

    // Set up global keyboard shortcuts
    let switch_theme = on_switch.clone();
    window_event_listener(ev::keydown, move |ev: KeyboardEvent| {
        if ev.shift_key() {
            match ev.key().to_lowercase().as_str() {
                "s" => {
                    // Toggle light/dark mode
                    ev.prevent_default();
                    let current = current_theme.get();
                    let new_mode = match current.mode {
                        Mode::Light => Mode::Dark,
                        Mode::Dark => Mode::Light,
                    };
                    let new_theme = match (current.season, new_mode) {
                        (Season::Spring, Mode::Light) => get_spring_light_theme(),
                        (Season::Spring, Mode::Dark) => get_spring_dark_theme(),
                        (Season::Summer, Mode::Light) => get_summer_light_theme(),
                        (Season::Summer, Mode::Dark) => get_summer_dark_theme(),
                    };
                    switch_theme.call(new_theme);
                },
                "x" => {
                    // Next theme
                    ev.prevent_default();
                    let current = current_theme.get();
                    let new_season = match current.season {
                        Season::Spring => Season::Summer,
                        Season::Summer => Season::Spring,
                    };
                    let new_theme = match (new_season, current.mode) {
                        (Season::Spring, Mode::Light) => get_spring_light_theme(),
                        (Season::Spring, Mode::Dark) => get_spring_dark_theme(),
                        (Season::Summer, Mode::Light) => get_summer_light_theme(),
                        (Season::Summer, Mode::Dark) => get_summer_dark_theme(),
                    };
                    switch_theme.call(new_theme);
                },
                "w" => {
                    // Previous theme
                    ev.prevent_default();
                    let current = current_theme.get();
                    let new_season = match current.season {
                        Season::Spring => Season::Summer,
                        Season::Summer => Season::Spring,
                    };
                    let new_theme = match (new_season, current.mode) {
                        (Season::Spring, Mode::Light) => get_spring_light_theme(),
                        (Season::Spring, Mode::Dark) => get_spring_dark_theme(),
                        (Season::Summer, Mode::Light) => get_summer_light_theme(),
                        (Season::Summer, Mode::Dark) => get_summer_dark_theme(),
                    };
                    switch_theme.call(new_theme);
                },
                _ => (),
            }
        }
    });

    view! {
        <>
            <div
                class="relative group"
            >
                // Light/Dark mode toggle button
                <button
                    class="flex items-center gap-3 px-4 py-2.5 rounded-xl bg-surface hover:bg-surface/90 active:bg-surface/75 text-text border border-background/10 shadow-sm hover:shadow-md active:shadow-sm transition-all duration-200"
                    on:click=toggle_mode
                >
                    <div class="flex items-center gap-2.5">
                        <span class="text-xl leading-none">{mode_icon}</span>
                        <span class="text-sm font-medium tracking-wide">
                            {move || match current_theme.get().mode {
                                Mode::Light => "Light",
                                Mode::Dark => "Dark",
                            }}
                        </span>
                    </div>
                    <div class="h-4 w-px bg-text/10"/>
                    <div class="flex gap-1.5">
                        <kbd class="text-[10px] font-medium text-text-muted bg-background/50 px-1.5 py-0.5 rounded-md">{"⇧S"}</kbd>
                        <kbd class="text-[10px] font-medium text-text-muted bg-background/50 px-1.5 py-0.5 rounded-md">{"⇧X"}</kbd>
                        <kbd class="text-[10px] font-medium text-text-muted bg-background/50 px-1.5 py-0.5 rounded-md">{"⇧W"}</kbd>
                    </div>
                </button>

                // Dropdown menu for seasons
                <div class="absolute right-0 mt-2 w-52 bg-surface rounded-xl shadow-lg shadow-background/10 border border-background/10 opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all duration-200 overflow-hidden">
                    <div class="px-3 py-2 border-b border-background/10">
                        <div class="text-[11px] font-medium text-text-muted tracking-wider uppercase">Theme</div>
                    </div>
                    <div class="p-1.5">
                        {move || {
                            let current = current_theme.get();
                            let seasons = vec![Season::Spring, Season::Summer];
                            
                            seasons.into_iter().map(move |season| {
                                let season_clone = season;
                                let icon = match season {
                                    Season::Spring => "🌸",
                                    Season::Summer => "☀️",
                                };
                                let name = match season {
                                    Season::Spring => "Spring",
                                    Season::Summer => "Summer",
                                };
                                let is_current = current.season == season;
                                
                                view! {
                                    <button
                                        class=move || format!(
                                            "w-full px-3 py-2 rounded-lg text-text flex items-center justify-between transition-all duration-150 {} {} {}",
                                            if is_current { "bg-background/30" } else { "hover:bg-background/20 active:bg-background/30" },
                                            if is_current { "font-medium" } else { "font-normal" },
                                            if is_current { "shadow-sm" } else { "" }
                                        )
                                        on:click=move |_| handle_season_switch(season_clone)
                                    >
                                        <span class="flex items-center gap-3">
                                            <span class="text-lg leading-none">{icon}</span>
                                            <span class="text-sm tracking-wide">{name}</span>
                                        </span>
                                        {move || if is_current {
                                            view! { 
                                                <span class="flex items-center text-accent">
                                                    <span class="text-[10px] font-medium tracking-wider uppercase">Current</span>
                                                </span>
                                            }.into_view()
                                        } else {
                                            view! {}.into_view()
                                        }}
                                    </button>
                                }
                            }).collect::<Vec<_>>()
                        }}
                    </div>
                </div>
            </div>
        </>
    }
} 