use sauron::{html::units::px, jss};

pub fn get_styles() -> Vec<String> {
    vec![jss! {
        "body":{
            font_family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif",
            background: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)",
            margin: 0,
            padding: px(20),
            min_height: "100vh",
            width: "100vw",
            overflow_x: "hidden",
            overflow_y: "auto",
            display: "flex",
            align_items: "center",
            justify_content: "center",
        },

        "main":{
            background: "rgba(255, 255, 255, 0.95)",
            border_radius: px(20),
            padding: "2vh 3vw",
            box_shadow: "0 20px 40px rgba(0, 0, 0, 0.1)",
            backdrop_filter: "blur(10px)",
            text_align: "center",
            min_width: "280px",
            min_height: "600px",
            width: "90vw",
            max_width: "600px",
            height: "auto",
            max_height: "none",
            display: "flex",
            flex_direction: "column",
            justify_content: "flex-start",
            gap: px(20),
            box_sizing: "border-box",
            margin: format!("{} 0", px(20)),
        },

        "input[type='button']":{
            font_size: px(16),
            padding: format!("{} {}", px(8), px(12)),
            margin: px(4),
            border: "none",
            border_radius: px(25),
            background: "linear-gradient(45deg, #ff6b6b, #ee5a52)",
            color: "white",
            cursor: "pointer",
            font_weight: "600",
            transition: "all 0.3s ease",
            box_shadow: "0 2px 8px rgba(238, 90, 82, 0.3)",
            min_width: px(40),
            height: px(40),
        },

        "input[type='button']:hover":{
            transform: "translateY(-2px)",
            box_shadow: "0 8px 25px rgba(238, 90, 82, 0.4)",
            background: "linear-gradient(45deg, #ff5252, #e53935)",
        },

        "input[type='button']:active":{
            transform: "translateY(0)",
            box_shadow: "0 2px 10px rgba(238, 90, 82, 0.3)",
        },

        ".count":{
            font_size: px(20),
            padding: format!("{} {}", px(8), px(16)),
            margin: px(4),
            border: "2px solid #4f46e5",
            border_radius: px(12),
            background: "white",
            color: "#4f46e5",
            cursor: "pointer",
            font_weight: "700",
            transition: "all 0.3s ease",
            box_shadow: "0 2px 8px rgba(79, 70, 229, 0.2)",
            min_width: px(60),
            height: px(40),
        },

        ".count:hover":{
            background: "#4f46e5",
            color: "white",
            transform: "scale(1.05)",
            box_shadow: "0 8px 25px rgba(79, 70, 229, 0.3)",
        },

        ".image-container":{
            flex: "0 0 auto",
            display: "flex",
            flex_direction: "column",
            justify_content: "center",
            align_items: "center",
            width: "100%",
            height: "400px",
            overflow: "hidden",
            margin: format!("{} 0", px(10)),
            box_sizing: "border-box",
            background: "#fff",
            border_radius: px(15),
            padding: px(10),
        },

        ".cat-image":{
            display: "block",
            max_width: "100%",
            max_height: "100%",
            width: "auto",
            height: "auto",
            object_fit: "contain",
            object_position: "center center",
            border_radius: px(10),
            box_shadow: "0 8px 25px rgba(0, 0, 0, 0.1)",
        },

        ".button-row":{
            display: "flex",
            flex_wrap: "wrap",
            justify_content: "center",
            align_items: "center",
            gap: px(5),
            margin: format!("{} 0", px(8)),
        },

        ".nav-button":{
            background: "linear-gradient(45deg, #4f46e5, #6366f1)",
            color: "white",
            border: "none",
            padding: format!("{} {}", px(12), px(20)),
            border_radius: px(8),
            font_size: px(14),
            font_weight: "600",
            cursor: "pointer",
            transition: "all 0.3s ease",
            box_shadow: "0 2px 8px rgba(79, 70, 229, 0.3)",
        },

        ".nav-button:hover":{
            background: "linear-gradient(45deg, #6366f1, #8b5cf6)",
            transform: "translateY(-2px)",
            box_shadow: "0 4px 12px rgba(79, 70, 229, 0.4)",
        },

        ".nav-button.active":{
            background: "linear-gradient(45deg, #f59e0b, #f97316)",
            box_shadow: "0 2px 8px rgba(245, 158, 11, 0.3)",
        },

        ".nav-button.active:hover":{
            background: "linear-gradient(45deg, #f97316, #ea580c)",
            box_shadow: "0 4px 12px rgba(245, 158, 11, 0.4)",
        },

        ".game-card":{
            background: "rgba(255, 255, 255, 0.9)",
            border_radius: px(15),
            padding: px(20),
            box_shadow: "0 8px 25px rgba(0, 0, 0, 0.1)",
            backdrop_filter: "blur(10px)",
            border: "1px solid rgba(255, 255, 255, 0.2)",
        },

        ".game-button":{
            background: "linear-gradient(45deg, #4f46e5, #6366f1)",
            color: "white",
            border: "none",
            padding: format!("{} {}", px(10), px(16)),
            border_radius: px(8),
            font_size: px(14),
            font_weight: "600",
            cursor: "pointer",
            transition: "all 0.3s ease",
            box_shadow: "0 2px 8px rgba(79, 70, 229, 0.3)",
            margin: px(2),
        },

        ".game-button:hover":{
            background: "linear-gradient(45deg, #6366f1, #8b5cf6)",
            transform: "translateY(-1px)",
            box_shadow: "0 4px 12px rgba(79, 70, 229, 0.4)",
        },

        ".small-button":{
            background: "linear-gradient(45deg, #10b981, #059669)",
            color: "white",
            border: "none",
            padding: format!("{} {}", px(6), px(10)),
            border_radius: px(6),
            font_size: px(12),
            font_weight: "600",
            cursor: "pointer",
            transition: "all 0.2s ease",
            box_shadow: "0 1px 4px rgba(16, 185, 129, 0.3)",
            margin: px(1),
            min_width: px(32),
            height: px(28),
        },

        ".small-button:hover":{
            background: "linear-gradient(45deg, #059669, #047857)",
            transform: "translateY(-1px)",
            box_shadow: "0 2px 8px rgba(16, 185, 129, 0.4)",
        },

        ".memory-card":{
            width: px(60),
            height: px(60),
            font_size: px(24),
            border: "none",
            border_radius: px(8),
            cursor: "pointer",
            transition: "all 0.3s ease",
            box_shadow: "0 2px 8px rgba(0, 0, 0, 0.1)",
            font_weight: "bold",
        },

        ".memory-card:hover":{
            transform: "scale(1.05)",
            box_shadow: "0 4px 12px rgba(0, 0, 0, 0.2)",
        },

        ".memory-card:disabled":{
            cursor: "not-allowed",
            transform: "none",
        },

        "input[type='number']":{
            font_family: "inherit",
            font_size: px(14),
            font_weight: "600",
            transition: "all 0.3s ease",
            box_shadow: "0 1px 4px rgba(0, 0, 0, 0.1)",
        },

        "input[type='number']:focus":{
            outline: "none",
            border_color: "#4f46e5",
            box_shadow: "0 0 0 2px rgba(79, 70, 229, 0.2)",
        }
    }]
}
