pub fn get_theme() -> Theme {
    Theme {
        name: "Summer Dark".to_string(),
        season: Season::Summer,
        mode: Mode::Dark,
        colors: ThemeColors {
            background: "#1F2937".to_string(),    // Dark cool gray
            surface: "#374151".to_string(),      // Cool gray
            text: "#F9FAFB".to_string(),         // Very light gray
            text_muted: "#9CA3AF".to_string(),   // Light gray
            primary: "#0EA5E9".to_string(),      // Sky blue
            secondary: "#F59E0B".to_string(),     // Amber
            seasonal_primary: "#FFD700".to_string(),    // Gold
            seasonal_secondary: "#FF6B6B".to_string(),  // Coral red
            seasonal_accent: "#98FB98".to_string(),     // Pale green
            accent: "#EC4899".to_string(),       // Pink
        }
    }
} 