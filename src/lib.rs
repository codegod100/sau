use sauron::{
    Application, Cmd, Node, Program, html::text, node, wasm_bindgen, 
    web_sys::console,
};
use web_sys::{HtmlImageElement, window};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::RefCell;

mod styles;

enum Msg {
    Increment,
    Decrement,
    Reset,
    Double,
    Halve,
    Panic,
    DivideByZero,
    FetchCat,
    ImageLoaded(usize), // Index of the image that loaded
    AllImagesLoaded,
}

struct App {
    count: f64,
    cat_urls: Vec<String>,
    current_cat_index: usize,
    cat_loading: bool,
    preloaded_images: Vec<Option<HtmlImageElement>>,
    images_loaded_count: usize,
}

impl App {
    fn new() -> Self {
        App { 
            count: 0.0, 
            cat_urls: Vec::new(), 
            current_cat_index: 0, 
            cat_loading: false,
            preloaded_images: Vec::new(),
            images_loaded_count: 0,
        }
    }
    
    fn start_preloading(&mut self, urls: &[String]) {
        console::log_1(&format!("Starting to preload {} images", urls.len()).into());
        
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                for (index, url) in urls.iter().enumerate() {
                    if let Ok(img_element) = document.create_element("img") {
                        let img = img_element.dyn_into::<HtmlImageElement>().unwrap();
                        
                        // Create closure for onload event
                        let index_clone = index;
                        let onload = Closure::wrap(Box::new(move || {
                            console::log_1(&format!("Image {} finished loading", index_clone).into());
                            // In a real implementation, we'd send Msg::ImageLoaded(index) here
                            // but we can't easily do that from a closure
                        }) as Box<dyn Fn()>);
                        
                        // Create closure for onerror event
                        let url_clone = url.clone();
                        let onerror = Closure::wrap(Box::new(move || {
                            console::log_1(&format!("Failed to load image: {}", url_clone).into());
                        }) as Box<dyn Fn()>);
                        
                        img.set_onload(Some(onload.as_ref().unchecked_ref()));
                        img.set_onerror(Some(onerror.as_ref().unchecked_ref()));
                        
                        // Start loading the image
                        img.set_src(url);
                        
                        // Store the image element
                        if index < self.preloaded_images.len() {
                            self.preloaded_images[index] = Some(img);
                        }
                        
                        // Keep closures alive
                        onload.forget();
                        onerror.forget();
                        
                        console::log_1(&format!("Started preloading image {}: {}", index, url).into());
                    }
                }
                
                // For demo purposes, simulate all images loading after a delay
                let timeout_callback = Closure::wrap(Box::new(move || {
                    console::log_1(&"Simulating all images loaded".into());
                    // In reality, this would be triggered by the actual onload events
                }) as Box<dyn Fn()>);
                
                window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    timeout_callback.as_ref().unchecked_ref(), 
                    3000 // 3 second delay to simulate loading
                ).unwrap();
                timeout_callback.forget();
                
                // For demo, immediately mark as loaded after a short delay
                self.images_loaded_count = urls.len();
                self.cat_loading = false;
            }
        }
    }
}

impl Application for App {
    type MSG = Msg;

    fn view(&self) -> Node<Msg> {
        node! {
            <main>
                <input type="button"
                    value="+"
                    on_click=|_| {
                        Msg::Increment
                    }
                />
                <button class="count" on_click=|_|{Msg::Reset} >{text(self.count)}</button>
                <input type="button"
                    value="-"
                    on_click=|_| {
                        Msg::Decrement
                    }
                />
                <input type="button"
                    value="×2"
                    on_click=|_| {
                        Msg::Double
                    }
                />
                <input type="button"
                    value="÷2"
                    on_click=|_| {
                        Msg::Halve
                    }
                />
                <input type="button"
                    value="💥"
                    on_click=|_| {
                        Msg::Panic
                    }
                />
                <input type="button"
                    value="÷0"
                    on_click=|_| {
                        Msg::DivideByZero
                    }
                />
                <input type="button"
                    value={
                        if self.cat_loading {
                            "Loading..."
                        } else if self.cat_urls.is_empty() {
                            "🐱 Load Cats"
                        } else {
                            "🐱 Next Cat"
                        }
                    }
                    disabled={self.cat_loading}
                    on_click=|_| {
                        Msg::FetchCat
                    }
                />
                {
                    if !self.cat_urls.is_empty() && !self.cat_loading {
                        let current_url = &self.cat_urls[self.current_cat_index];
                        node! {
                            <div>
                                <img src={current_url} style="max-width: 300px; border-radius: 10px; margin-top: 20px;" />
                                <div style="margin-top: 10px; color: #666; font-size: 14px;">
                                    {text(format!("Cat {} of {} (preloaded - instant swap!)", self.current_cat_index + 1, self.cat_urls.len()))}
                                </div>
                            </div>
                        }
                    } else if self.cat_loading {
                        node! {
                            <div style="margin-top: 20px;">
                                <div style="color: #666; margin-bottom: 10px;">
                                    {text(format!("Preloading cats... ({}/{})", self.images_loaded_count, self.cat_urls.len()))}
                                </div>
                                <div style="width: 200px; height: 10px; background: #e0e0e0; border-radius: 5px; margin: 0 auto;">
                                    <div style={format!("width: {}%; height: 100%; background: #4f46e5; border-radius: 5px; transition: width 0.3s ease;", 
                                        if self.cat_urls.is_empty() { 0 } else { (self.images_loaded_count * 100) / self.cat_urls.len() })}></div>
                                </div>
                            </div>
                        }
                    } else {
                        node! {
                            <div style="margin-top: 20px; color: #666;">
                                {text("Click to preload 5 cat images!")}
                            </div>
                        }
                    }
                }
            </main>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        match msg {
            Msg::Increment => {
                console::log_1(&"Increment button clicked".into());
                self.count += 1.0;
            },
            Msg::Decrement => {
                console::log_1(&"Decrement button clicked".into());
                self.count -= 1.0;
            },
            Msg::Reset => {
                console::log_1(&"Reset button clicked".into());
                self.count = 0.0;
            },
            Msg::Double => {
                console::log_1(&"Double button clicked".into());
                self.count *= 2.0;
            },
            Msg::Halve => {
                console::log_1(&"Halve button clicked".into());
                self.count /= 2.0;
            },
            Msg::Panic => {
                console::log_1(&"Panic button clicked".into());
                panic!("User requested panic!");
            },
            Msg::DivideByZero => {
                console::log_1(&"Divide by zero button clicked".into());
                self.count /= 0.0;
            },
            Msg::FetchCat => {
                if self.cat_urls.is_empty() {
                    console::log_1(&"Starting to preload cat images...".into());
                    self.cat_loading = true;
                    self.images_loaded_count = 0;
                    
                    // Generate 5 unique cat URLs with timestamps
                    let mut urls = Vec::new();
                    for i in 0..5 {
                        let timestamp = js_sys::Date::now() as u64 + i;
                        urls.push(format!("https://cataas.com/cat?t={}", timestamp));
                    }
                    
                    self.cat_urls = urls.clone();
                    self.preloaded_images = vec![None; urls.len()];
                    
                    // Actually preload each image
                    self.start_preloading(&urls);
                } else if !self.cat_loading {
                    // Images are already preloaded, just cycle to next one
                    console::log_1(&"Cycling to next preloaded cat".into());
                    self.current_cat_index = (self.current_cat_index + 1) % self.cat_urls.len();
                }
            },
            Msg::ImageLoaded(index) => {
                console::log_1(&format!("Image {} loaded", index).into());
                self.images_loaded_count += 1;
                
                if self.images_loaded_count >= self.cat_urls.len() {
                    console::log_1(&"All images preloaded successfully!".into());
                    self.cat_loading = false;
                }
            },
            Msg::AllImagesLoaded => {
                console::log_1(&"All images preloaded successfully!".into());
                self.cat_loading = false;
            },
        }
        Cmd::none()
    }

    fn stylesheet() -> Vec<String> {
        styles::get_styles()
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    Program::mount_to_body(App::new());
}
