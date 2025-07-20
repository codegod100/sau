use sauron::{html::units::px, jss};

pub fn get_styles() -> Vec<String> {
    vec![jss! {
        "body":{
            font_family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif",
            background: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)",
            margin: 0,
            padding: 0,
            height: "100vh",
            overflow: "hidden",
            display: "flex",
            align_items: "center",
            justify_content: "center",
        },

        "main":{
            background: "rgba(255, 255, 255, 0.95)",
            border_radius: px(20),
            padding: px(40),
            box_shadow: "0 20px 40px rgba(0, 0, 0, 0.1)",
            backdrop_filter: "blur(10px)",
            text_align: "center",
            min_width: px(300),
            width: px(500),
            height: px(600),
            display: "flex",
            flex_direction: "column",
            justify_content: "space-between",
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
            flex: "1",
            display: "flex",
            flex_direction: "column",
            justify_content: "center",
            align_items: "center",
            min_height: px(400),
            max_height: px(450),
            overflow: "hidden",
            margin: format!("{} 0", px(10)),
        },

        ".cat-image":{
            max_width: "100%",
            max_height: "100%",
            width: "auto",
            height: "auto",
            object_fit: "contain",
            border_radius: px(15),
            box_shadow: "0 8px 25px rgba(0, 0, 0, 0.1)",
        },

        ".button-row":{
            display: "flex",
            flex_wrap: "wrap",
            justify_content: "center",
            align_items: "center",
            gap: px(5),
            margin: format!("{} 0", px(8)),
        }
    }]
}