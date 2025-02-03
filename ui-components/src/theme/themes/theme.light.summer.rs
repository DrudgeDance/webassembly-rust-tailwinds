pub fn get_theme() -> Theme {
    Theme {
        name: "Summer Light".to_string(),
        season: Season::Summer,
        mode: Mode::Light,
        colors: ThemeColors {
            background: "#FFFAF0".to_string(),    // Floral white
            surface: "#FFFFFF".to_string(),       // White
            text: "#111827".to_string(),         // Very dark gray
            text_muted: "#6B7280".to_string(),   // Gray
            primary: "#0EA5E9".to_string(),      // Sky blue
            secondary: "#F59E0B".to_string(),     // Amber
            seasonal_primary: "#FFD700".to_string(),    // Gold
            seasonal_secondary: "#FF6B6B".to_string(),  // Coral red
            seasonal_accent: "#98FB98".to_string(),     // Pale green
            accent: "#EC4899".to_string(),       // Pink
        }
    }
} 