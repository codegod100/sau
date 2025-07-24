use sauron::{html::units::px, jss};

pub fn get_styles() -> Vec<String> {
    vec![jss! {
        "body":{
            font_family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif",
            background: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)",
            margin: 0,
            padding: "10px 20px",
            min_height: "100vh",
            width: "100vw",
            overflow_x: "hidden",
            overflow_y: "auto",
            display: "flex",
            align_items: "flex-start",
            justify_content: "center",
        },

        "main":{
            background: "rgba(255, 255, 255, 0.95)",
            border_radius: "15px",
            padding: "15px",
            box_shadow: "0 10px 30px rgba(0, 0, 0, 0.1)",
            backdrop_filter: "blur(10px)",
            text_align: "center",
            min_width: "280px",
            width: "100%",
            max_width: "600px",
            height: "auto",
            display: "flex",
            flex_direction: "column",
            justify_content: "flex-start",
            gap: "15px",
            box_sizing: "border-box",
            margin: "10px auto",
        },

        "input[type='button']":{
            font_size: "16px",
            padding: "12px 16px",
            margin: "6px",
            border: "none",
            border_radius: "8px",
            background: "linear-gradient(45deg, #ff6b6b, #ee5a52)",
            color: "white",
            cursor: "pointer",
            font_weight: "600",
            transition: "all 0.3s ease",
            box_shadow: "0 2px 8px rgba(238, 90, 82, 0.3)",
            min_width: "44px",
            min_height: "44px",
            touch_action: "manipulation",
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
            font_size: "18px",
            padding: "12px 20px",
            margin: "6px",
            border: "2px solid #4f46e5",
            border_radius: "8px",
            background: "white",
            color: "#4f46e5",
            cursor: "pointer",
            font_weight: "700",
            transition: "all 0.3s ease",
            box_shadow: "0 2px 8px rgba(79, 70, 229, 0.2)",
            min_width: "60px",
            min_height: "44px",
            touch_action: "manipulation",
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
            height: "min(60vh, 400px)",
            overflow: "hidden",
            margin: "10px 0",
            box_sizing: "border-box",
            background: "#fff",
            border_radius: "12px",
            padding: "10px",
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
            padding: "12px 16px",
            border_radius: "8px",
            font_size: "14px",
            font_weight: "600",
            cursor: "pointer",
            transition: "all 0.3s ease",
            box_shadow: "0 2px 8px rgba(79, 70, 229, 0.3)",
            min_height: "44px",
            touch_action: "manipulation",
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
            padding: "12px 20px",
            border_radius: "8px",
            font_size: "14px",
            font_weight: "600",
            cursor: "pointer",
            transition: "all 0.3s ease",
            box_shadow: "0 2px 8px rgba(79, 70, 229, 0.3)",
            margin: "4px",
            min_height: "44px",
            touch_action: "manipulation",
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
            padding: "8px 12px",
            border_radius: "6px",
            font_size: "12px",
            font_weight: "600",
            cursor: "pointer",
            transition: "all 0.2s ease",
            box_shadow: "0 1px 4px rgba(16, 185, 129, 0.3)",
            margin: "2px",
            min_width: "40px",
            min_height: "44px",
            touch_action: "manipulation",
        },

        ".small-button:hover":{
            background: "linear-gradient(45deg, #059669, #047857)",
            transform: "translateY(-1px)",
            box_shadow: "0 2px 8px rgba(16, 185, 129, 0.4)",
        },

        ".memory-card":{
            width: "min(15vw, 60px)",
            height: "min(15vw, 60px)",
            font_size: "min(6vw, 24px)",
            border: "none",
            border_radius: "8px",
            cursor: "pointer",
            transition: "all 0.3s ease",
            box_shadow: "0 2px 8px rgba(0, 0, 0, 0.1)",
            font_weight: "bold",
            min_width: "44px",
            min_height: "44px",
            touch_action: "manipulation",
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
        },

        "h1":{
            font_size: "clamp(24px, 5vw, 32px)",
            line_height: "1.2",
            margin: "0 0 20px 0",
        },

        "h2":{
            font_size: "clamp(20px, 4vw, 28px)",
            line_height: "1.3",
            margin: "0 0 15px 0",
        },

        "h3":{
            font_size: "clamp(16px, 3.5vw, 20px)",
            line_height: "1.4",
            margin: "0 0 10px 0",
        },

        "p":{
            font_size: "clamp(14px, 2.5vw, 16px)",
            line_height: "1.5",
            margin: "0 0 15px 0",
        }
    }]
}
