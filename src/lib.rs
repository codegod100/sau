use sauron::{
    Application, Cmd, Node, Program, html::text, node, wasm_bindgen, 
    web_sys::console,
};
use web_sys::{HtmlImageElement, window};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

mod styles;

#[derive(Debug, Clone, PartialEq)]
enum Route {
    Home,
    Counter,
    Cats,
    About,
    NotFound,
}

impl Route {
    fn from_hash(hash: &str) -> Self {
        match hash {
            "#/" | "#" | "" => Route::Home,
            "#/counter" => Route::Counter,
            "#/cats" => Route::Cats,
            "#/about" => Route::About,
            _ => Route::NotFound,
        }
    }
    
    fn to_hash(&self) -> &'static str {
        match self {
            Route::Home => "#/",
            Route::Counter => "#/counter",
            Route::Cats => "#/cats",
            Route::About => "#/about",
            Route::NotFound => "#/404",
        }
    }
}

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
    NextBatchImageLoaded,
    NextBatchComplete,
    NavigateTo(Route),
    UrlChanged(Route),
}

struct App {
    count: f64,
    cat_urls: Vec<String>,
    current_cat_index: usize,
    cat_loading: bool,
    preloaded_images: Vec<Option<HtmlImageElement>>,
    images_loaded_count: usize,
    is_preloading_next_batch: bool,
    total_cats_ever_loaded: usize,
    next_batch_urls: Vec<String>,
    next_batch_loaded_count: usize,
    current_route: Route,
}

impl App {
    fn new() -> Self {
        let initial_route = Self::get_current_route();
        App { 
            count: 0.0, 
            cat_urls: Vec::new(), 
            current_cat_index: 0, 
            cat_loading: false,
            preloaded_images: Vec::new(),
            images_loaded_count: 0,
            is_preloading_next_batch: false,
            total_cats_ever_loaded: 0,
            next_batch_urls: Vec::new(),
            next_batch_loaded_count: 0,
            current_route: initial_route,
        }
    }
    
    fn get_current_route() -> Route {
        if let Some(window) = window() {
            let location = window.location();
            if let Ok(hash) = location.hash() {
                return Route::from_hash(&hash);
            }
        }
        Route::Home
    }
    
    fn navigate_to(&self, route: Route) {
        if let Some(window) = window() {
            let location = window.location();
            let hash = route.to_hash();
            let _ = location.set_hash(hash);
            console::log_1(&format!("Navigated to: {}", hash).into());
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
    
    fn start_preloading_next_batch(&self) {
        console::log_1(&format!("Starting to preload next batch of {} images in background...", self.next_batch_urls.len()).into());
        
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                // Create images for background preloading
                for (index, url) in self.next_batch_urls.iter().enumerate() {
                    if let Ok(img_element) = document.create_element("img") {
                        let img = img_element.dyn_into::<HtmlImageElement>().unwrap();
                        
                        let url_clone = url.clone();
                        let onload = Closure::wrap(Box::new(move || {
                            console::log_1(&format!("Background image preloaded: {}", url_clone).into());
                            // In reality, we'd trigger Msg::NextBatchImageLoaded here
                        }) as Box<dyn Fn()>);
                        
                        let url_clone2 = url.clone();
                        let onerror = Closure::wrap(Box::new(move || {
                            console::log_1(&format!("Background preload failed: {}", url_clone2).into());
                            // Still count as "loaded" to avoid blocking
                        }) as Box<dyn Fn()>);
                        
                        img.set_onload(Some(onload.as_ref().unchecked_ref()));
                        img.set_onerror(Some(onerror.as_ref().unchecked_ref()));
                        img.set_src(url);
                        
                        onload.forget();
                        onerror.forget();
                        
                        console::log_1(&format!("Started preloading next batch image {}: {}", index, url).into());
                    }
                }
                
                // For demo, simulate completion of all images after delay
                let batch_size = self.next_batch_urls.len();
                let timeout_callback = Closure::wrap(Box::new(move || {
                    console::log_1(&format!("Simulating {} next batch images completed", batch_size).into());
                    // In reality, this would be triggered by actual onload events
                }) as Box<dyn Fn()>);
                
                window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    timeout_callback.as_ref().unchecked_ref(),
                    2000 // 2 second delay for background preload
                ).unwrap();
                timeout_callback.forget();
                
                // For demo purposes, simulate immediate completion
                console::log_1(&"Simulating immediate next batch completion for demo".into());
            }
        }
    }
    
    fn render_navigation(&self) -> Node<Msg> {
        node! {
            <nav style="background: #333; padding: 15px; margin-bottom: 20px;">
                <div style="display: flex; gap: 20px; justify-content: center;">
                    <button class={if self.current_route == Route::Home { "nav-button active" } else { "nav-button" }}
                        on_click=|_| Msg::NavigateTo(Route::Home)>
                        {text("Home")}
                    </button>
                    <button class={if self.current_route == Route::Counter { "nav-button active" } else { "nav-button" }}
                        on_click=|_| Msg::NavigateTo(Route::Counter)>
                        {text("Counter")}
                    </button>
                    <button class={if self.current_route == Route::Cats { "nav-button active" } else { "nav-button" }}
                        on_click=|_| Msg::NavigateTo(Route::Cats)>
                        {text("Cats")}
                    </button>
                    <button class={if self.current_route == Route::About { "nav-button active" } else { "nav-button" }}
                        on_click=|_| Msg::NavigateTo(Route::About)>
                        {text("About")}
                    </button>
                </div>
            </nav>
        }
    }
    
    fn render_current_page(&self) -> Node<Msg> {
        match self.current_route {
            Route::Home => self.render_home_page(),
            Route::Counter => self.render_counter_page(),
            Route::Cats => self.render_cats_page(),
            Route::About => self.render_about_page(),
            Route::NotFound => self.render_404_page(),
        }
    }
    
    fn render_home_page(&self) -> Node<Msg> {
        node! {
            <div style="text-align: center; padding: 40px;">
                <h1 style="color: #333; margin-bottom: 20px;">{text("Welcome to Sauron Demo")}</h1>
                <p style="color: #666; margin-bottom: 30px; font-size: 18px;">
                    {text("A Rust WebAssembly application with routing")}
                </p>
                <div style="display: flex; gap: 20px; justify-content: center;">
                    <button class="nav-button" on_click=|_| Msg::NavigateTo(Route::Counter)>
                        {text("Try Counter →")}
                    </button>
                    <button class="nav-button" on_click=|_| Msg::NavigateTo(Route::Cats)>
                        {text("Browse Cats →")}
                    </button>
                </div>
            </div>
        }
    }
    
    fn render_counter_page(&self) -> Node<Msg> {
        node! {
            <div>
                <h2 style="text-align: center; margin-bottom: 30px; color: #333;">
                    {text("Interactive Counter")}
                </h2>
                <div class="button-row">
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
                </div>
                
                <div class="button-row">
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
                </div>
            </div>
        }
    }
    
    fn render_cats_page(&self) -> Node<Msg> {
        node! {
            <div>
                <h2 style="text-align: center; margin-bottom: 30px; color: #333;">
                    {text("Infinite Cat Browser")}
                </h2>
                <div class="button-row">
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
                </div>

                <div class="image-container">
                    {
                        if !self.cat_urls.is_empty() && !self.cat_loading {
                            let current_url = &self.cat_urls[self.current_cat_index];
                            node! {
                                <div>
                                    <img class="cat-image" src={current_url} />
                                    <div style="margin-top: 15px; color: #666; font-size: 14px;">
                                        {text(format!("Cat {} of {}{}", 
                                            self.current_cat_index + 1, 
                                            self.cat_urls.len(),
                                            if self.is_preloading_next_batch { " (loading more...)" } else { " (infinite!)" }
                                        ))}
                                    </div>
                                </div>
                            }
                        } else if self.cat_loading {
                            node! {
                                <div>
                                    <div style="color: #666; margin-bottom: 20px; font-size: 16px;">
                                        {text(format!("Preloading cats... ({}/{})", self.images_loaded_count, self.cat_urls.len()))}
                                    </div>
                                    <div style="width: 250px; height: 12px; background: #e0e0e0; border-radius: 6px; margin: 0 auto;">
                                        <div style={format!("width: {}%; height: 100%; background: #4f46e5; border-radius: 6px; transition: width 0.3s ease;", 
                                            if self.cat_urls.is_empty() { 0 } else { (self.images_loaded_count * 100) / self.cat_urls.len() })}></div>
                                    </div>
                                </div>
                            }
                        } else {
                            node! {
                                <div style="color: #666; font-size: 16px;">
                                    {text("Click to preload 10 cat images!")}
                                </div>
                            }
                        }
                    }
                </div>
            </div>
        }
    }
    
    fn render_about_page(&self) -> Node<Msg> {
        node! {
            <div style="text-align: center; padding: 40px;">
                <h1 style="color: #333; margin-bottom: 20px;">{text("About This App")}</h1>
                <div style="color: #666; line-height: 1.6; max-width: 600px; margin: 0 auto;">
                    <p style="margin-bottom: 20px;">
                        {text("This is a Rust WebAssembly application built with the Sauron framework.")}
                    </p>
                    <p style="margin-bottom: 20px;">
                        {text("Features include:")}
                    </p>
                    <ul style="text-align: left; margin-bottom: 20px;">
                        <li>{text("Client-side routing")}</li>
                        <li>{text("Interactive counter with various operations")}</li>
                        <li>{text("Infinite cat image preloading")}</li>
                        <li>{text("Modern UI with glass-morphism design")}</li>
                        <li>{text("Responsive layout")}</li>
                    </ul>
                    <p>
                        {text("Built with 🦀 Rust and compiled to WebAssembly for high performance.")}
                    </p>
                </div>
            </div>
        }
    }
    
    fn render_404_page(&self) -> Node<Msg> {
        node! {
            <div style="text-align: center; padding: 40px;">
                <h1 style="color: #ff6b6b; margin-bottom: 20px; font-size: 72px;">{text("404")}</h1>
                <h2 style="color: #333; margin-bottom: 20px;">{text("Page Not Found")}</h2>
                <p style="color: #666; margin-bottom: 30px;">
                    {text("The page you're looking for doesn't exist.")}
                </p>
                <button class="nav-button" on_click=|_| Msg::NavigateTo(Route::Home)>
                    {text("← Go Home")}
                </button>
            </div>
        }
    }
}

impl Application for App {
    type MSG = Msg;

    fn view(&self) -> Node<Msg> {
        node! {
            <div>
                {self.render_navigation()}
                <main>
                    {self.render_current_page()}
                </main>
            </div>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        // Always check if the current route matches the URL hash
        let current_url_route = Self::get_current_route();
        if current_url_route != self.current_route {
            console::log_1(&format!("Route sync: {:?} -> {:?}", self.current_route, current_url_route).into());
            self.current_route = current_url_route;
        }
        
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
                    
                    // Generate 10 unique cat URLs with timestamps
                    let mut urls = Vec::new();
                    for i in 0..10 {
                        let timestamp = js_sys::Date::now() as u64 + i;
                        urls.push(format!("https://cataas.com/cat?t={}", timestamp));
                    }
                    
                    self.cat_urls = urls.clone();
                    self.total_cats_ever_loaded = urls.len();
                    self.preloaded_images = vec![None; urls.len()];
                    
                    // Actually preload each image
                    self.start_preloading(&urls);
                } else if !self.cat_loading {
                    // Images are already preloaded, just cycle to next one
                    console::log_1(&"Cycling to next preloaded cat".into());
                    self.current_cat_index = (self.current_cat_index + 1) % self.cat_urls.len();
                    
                    // Check if we're approaching the end of current batch (second to last)
                    let approaching_end = self.current_cat_index >= self.cat_urls.len() - 2;
                    if approaching_end && !self.is_preloading_next_batch {
                        console::log_1(&"Approaching end of batch - starting to preload next batch".into());
                        self.is_preloading_next_batch = true;
                        self.next_batch_loaded_count = 0;
                        
                        // Generate next batch of URLs
                        self.next_batch_urls.clear();
                        for i in 0..10 {
                            let timestamp = js_sys::Date::now() as u64 + self.total_cats_ever_loaded as u64 + i + 1000;
                            self.next_batch_urls.push(format!("https://cataas.com/cat?t={}", timestamp));
                        }
                        
                        // Actually preload the next batch images
                        self.start_preloading_next_batch();
                        
                        // For demo, simulate all images loading immediately
                        self.next_batch_loaded_count = self.next_batch_urls.len();
                        console::log_1(&"Demo: Simulating all next batch images preloaded".into());
                        
                        // Add the preloaded URLs to the main collection
                        self.cat_urls.extend(self.next_batch_urls.clone());
                        self.total_cats_ever_loaded = self.cat_urls.len();
                        self.is_preloading_next_batch = false;
                        self.next_batch_urls.clear();
                        self.next_batch_loaded_count = 0;
                        
                        console::log_1(&format!("Total cats now available: {}", self.cat_urls.len()).into());
                    }
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
            Msg::NextBatchImageLoaded => {
                self.next_batch_loaded_count += 1;
                console::log_1(&format!("Next batch image loaded: {}/{}", self.next_batch_loaded_count, self.next_batch_urls.len()).into());
                
                // Check if all next batch images are loaded
                if self.next_batch_loaded_count >= self.next_batch_urls.len() {
                    console::log_1(&"All next batch images preloaded! Adding to available cats.".into());
                    
                    // Add the preloaded URLs to the main collection
                    self.cat_urls.extend(self.next_batch_urls.clone());
                    self.total_cats_ever_loaded = self.cat_urls.len();
                    self.is_preloading_next_batch = false;
                    self.next_batch_urls.clear();
                    self.next_batch_loaded_count = 0;
                    
                    console::log_1(&format!("Total cats now available: {}", self.cat_urls.len()).into());
                }
            },
            Msg::NextBatchComplete => {
                console::log_1(&"Next batch preloading complete!".into());
                self.cat_urls.extend(self.next_batch_urls.clone());
                self.total_cats_ever_loaded = self.cat_urls.len();
                self.is_preloading_next_batch = false;
                self.next_batch_urls.clear();
                self.next_batch_loaded_count = 0;
            },
            Msg::NavigateTo(route) => {
                console::log_1(&format!("Navigating to: {:?}", route).into());
                self.navigate_to(route.clone());
                self.current_route = route;
            },
            Msg::UrlChanged(route) => {
                console::log_1(&format!("URL changed to: {:?}", route).into());
                self.current_route = route;
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
    let app = App::new();
    
    // Set up hashchange event listener for browser back/forward buttons and manual hash changes
    if let Some(window) = window() {
        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            console::log_1(&"Hash change detected".into());
            // In a real implementation, we'd send Msg::UrlChanged here
            // The app will automatically pick up the new route on next render
        }) as Box<dyn Fn(web_sys::Event)>);
        
        let _ = window.add_event_listener_with_callback("hashchange", closure.as_ref().unchecked_ref());
        closure.forget();
    }
    
    Program::mount_to_body(app);
}
