pub fn get_theme() -> Theme {
    Theme {
        name: "Spring Light".to_string(),
        season: Season::Spring,
        mode: Mode::Light,
        colors: ThemeColors {
            background: "#F8FAF8".to_string(),    // Very light spring green
            surface: "#FFFFFF".to_string(),       // White
            text: "#111827".to_string(),         // Very dark gray
            text_muted: "#6B7280".to_string(),   // Gray
            primary: "#4F46E5".to_string(),      // Indigo
            secondary: "#10B981".to_string(),     // Emerald
            seasonal_primary: "#98FB98".to_string(),    // Pale green
            seasonal_secondary: "#FFB6C1".to_string(),  // Light pink
            seasonal_accent: "#87CEEB".to_string(),     // Sky blue
            accent: "#F59E0B".to_string(),       // Amber
        }
    }
} 