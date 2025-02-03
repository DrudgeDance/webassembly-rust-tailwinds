pub fn get_theme() -> Theme {
    Theme {
        name: "Spring Dark".to_string(),
        season: Season::Spring,
        mode: Mode::Dark,
        colors: ThemeColors {
            background: "#1A1C2A".to_string(),    // Dark blue-gray
            surface: "#2D2D3D".to_string(),      // Lighter blue-gray
            text: "#F9FAFB".to_string(),         // Very light gray
            text_muted: "#9CA3AF".to_string(),   // Light gray
            primary: "#4F46E5".to_string(),      // Indigo
            secondary: "#10B981".to_string(),     // Emerald
            seasonal_primary: "#98FB98".to_string(),    // Pale green
            seasonal_secondary: "#FFB6C1".to_string(),  // Light pink
            seasonal_accent: "#87CEEB".to_string(),     // Sky blue
            accent: "#F59E0B".to_string(),       // Amber
        }
    }
} 