use sauron::{
    Application, Cmd, Node, Program, html::text, node, wasm_bindgen, 
    web_sys::console,
};
use web_sys::{HtmlImageElement, HtmlInputElement, window};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

mod styles;

#[derive(Debug, Clone, PartialEq)]
enum Route {
    Home,
    Counter,
    Cats,
    Games,
    About,
    NotFound,
}

impl Route {
    fn from_hash(hash: &str) -> Self {
        match hash {
            "#/" | "#" | "" => Route::Home,
            "#/counter" => Route::Counter,
            "#/cats" => Route::Cats,
            "#/games" => Route::Games,
            "#/about" => Route::About,
            _ => Route::NotFound,
        }
    }
    
    fn to_hash(&self) -> &'static str {
        match self {
            Route::Home => "#/",
            Route::Counter => "#/counter",
            Route::Cats => "#/cats",
            Route::Games => "#/games",
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
    // Games messages
    RockPaperScissorsPlay(String), // "rock", "paper", or "scissors"
    GuessNumber(i32),
    GuessInputChanged(String),
    SubmitGuess,
    NewGuessGame,
    MemoryCardClick(usize),
    StartMemoryGame,
    ResetMemoryGame,
    HideMemoryCards,
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
    // Games state
    rps_player_choice: Option<String>,
    rps_computer_choice: Option<String>,
    rps_result: Option<String>,
    rps_score: (i32, i32), // (player_wins, computer_wins)
    guess_target: i32,
    guess_attempts: i32,
    guess_message: String,
    guess_game_over: bool,
    guess_input: String,
    memory_cards: Vec<(String, bool, bool)>, // (emoji, revealed, matched)
    memory_moves: i32,
    memory_matches: i32,
    memory_first_card: Option<usize>,
    memory_game_over: bool,
    memory_second_card: Option<usize>,
    memory_showing_pair: bool,
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
            // Games state initialization
            rps_player_choice: None,
            rps_computer_choice: None,
            rps_result: None,
            rps_score: (0, 0),
            guess_target: Self::random_number(1, 100),
            guess_attempts: 0,
            guess_message: "Guess a number between 1 and 100!".to_string(),
            guess_game_over: false,
            guess_input: String::new(),
            memory_cards: Vec::new(),
            memory_moves: 0,
            memory_matches: 0,
            memory_first_card: None,
            memory_game_over: false,
            memory_second_card: None,
            memory_showing_pair: false,
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
    
    fn random_number(min: i32, max: i32) -> i32 {
        let range = max - min + 1;
        let random_value = (js_sys::Math::random() * range as f64) as i32;
        min + random_value
    }
    
    fn get_rps_computer_choice() -> String {
        let choices = ["rock", "paper", "scissors"];
        let index = Self::random_number(0, 2) as usize;
        choices[index].to_string()
    }
    
    fn determine_rps_winner(player: &str, computer: &str) -> String {
        match (player, computer) {
            ("rock", "scissors") | ("paper", "rock") | ("scissors", "paper") => "You win!".to_string(),
            ("scissors", "rock") | ("rock", "paper") | ("paper", "scissors") => "Computer wins!".to_string(),
            _ => "It's a tie!".to_string(),
        }
    }
    
    fn initialize_memory_cards() -> Vec<(String, bool, bool)> {
        let emojis = ["🐶", "🐱", "🐭", "🐹", "🐰", "🦊", "🐻", "🐼"];
        let mut cards = Vec::new();
        
        // Add each emoji twice for pairs
        for emoji in &emojis {
            cards.push((emoji.to_string(), false, false));
            cards.push((emoji.to_string(), false, false));
        }
        
        // Shuffle the cards (simple shuffle using random swaps)
        for i in 0..cards.len() {
            let j = Self::random_number(0, (cards.len() - 1) as i32) as usize;
            cards.swap(i, j);
        }
        
        cards
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
                    <button class={if self.current_route == Route::Games { "nav-button active" } else { "nav-button" }}
                        on_click=|_| Msg::NavigateTo(Route::Games)>
                        {text("Games")}
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
            Route::Games => self.render_games_page(),
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
                <div style="display: flex; gap: 20px; justify-content: center; flex-wrap: wrap;">
                    <button class="nav-button" on_click=|_| Msg::NavigateTo(Route::Counter)>
                        {text("Try Counter →")}
                    </button>
                    <button class="nav-button" on_click=|_| Msg::NavigateTo(Route::Cats)>
                        {text("Browse Cats →")}
                    </button>
                    <button class="nav-button" on_click=|_| Msg::NavigateTo(Route::Games)>
                        {text("Play Games →")}
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
    
    fn render_games_page(&self) -> Node<Msg> {
        node! {
            <div style="padding: 20px;">
                <h2 style="text-align: center; margin-bottom: 30px; color: #333;">
                    {text("Mini Games Collection")}
                </h2>
                
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 30px; max-width: 1200px; margin: 0 auto;">
                    // Rock Paper Scissors Game
                    <div class="game-card">
                        <h3 style="text-align: center; margin-bottom: 20px; color: #4f46e5;">
                            {text("🪨 Rock Paper Scissors ✂️")}
                        </h3>
                        <div style="text-align: center; margin-bottom: 15px;">
                            <div style="font-size: 18px; margin-bottom: 10px;">
                                {text(format!("Score: You {} - {} Computer", self.rps_score.0, self.rps_score.1))}
                            </div>
                            <div style="display: flex; gap: 10px; justify-content: center; margin-bottom: 15px;">
                                <button class="game-button" on_click=|_| Msg::RockPaperScissorsPlay("rock".to_string())>
                                    {text("🪨 Rock")}
                                </button>
                                <button class="game-button" on_click=|_| Msg::RockPaperScissorsPlay("paper".to_string())>
                                    {text("📄 Paper")}
                                </button>
                                <button class="game-button" on_click=|_| Msg::RockPaperScissorsPlay("scissors".to_string())>
                                    {text("✂️ Scissors")}
                                </button>
                            </div>
                            {
                                if let (Some(player), Some(computer), Some(result)) = (&self.rps_player_choice, &self.rps_computer_choice, &self.rps_result) {
                                    node! {
                                        <div style="background: rgba(255,255,255,0.8); padding: 15px; border-radius: 8px;">
                                            <div style="margin-bottom: 10px;">
                                                {text(format!("You played: {}", match player.as_str() {
                                                    "rock" => "🪨 Rock",
                                                    "paper" => "📄 Paper",
                                                    "scissors" => "✂️ Scissors",
                                                    _ => player
                                                }))}
                                            </div>
                                            <div style="margin-bottom: 10px;">
                                                {text(format!("Computer played: {}", match computer.as_str() {
                                                    "rock" => "🪨 Rock",
                                                    "paper" => "📄 Paper",
                                                    "scissors" => "✂️ Scissors",
                                                    _ => computer
                                                }))}
                                            </div>
                                            <div style="font-weight: bold; color: #4f46e5;">
                                                {text(result)}
                                            </div>
                                        </div>
                                    }
                                } else {
                                    node! {
                                        <div style="color: #666;">
                                            {text("Choose your move!")}
                                        </div>
                                    }
                                }
                            }
                        </div>
                    </div>
                    
                    // Number Guessing Game
                    <div class="game-card">
                        <h3 style="text-align: center; margin-bottom: 20px; color: #4f46e5;">
                            {text("🎯 Number Guessing Game")}
                        </h3>
                        <div style="text-align: center;">
                            <div style="margin-bottom: 15px;">
                                {text(format!("Attempts: {}", self.guess_attempts))}
                            </div>
                            <div style="margin-bottom: 15px; color: #666;">
                                {text(&self.guess_message)}
                            </div>
                            {
                                if !self.guess_game_over {
                                    node! {
                                        <div>
                                            <div style="display: flex; gap: 8px; justify-content: center; align-items: center; margin-bottom: 15px; flex-wrap: wrap;">
                                                <input type="number" 
                                                    min="1" 
                                                    max="100" 
                                                    placeholder="1-100" 
                                                    value={&self.guess_input}
                                                    style="padding: 8px; border: 1px solid #ccc; border-radius: 4px; width: 80px; text-align: center;"
                                                    on_input=|event| {
                                                        if let Some(input) = event.event.target() {
                                                            if let Ok(input_element) = input.dyn_into::<HtmlInputElement>() {
                                                                Msg::GuessInputChanged(input_element.value())
                                                            } else {
                                                                Msg::GuessInputChanged(String::new())
                                                            }
                                                        } else {
                                                            Msg::GuessInputChanged(String::new())
                                                        }
                                                    } />
                                                <button class="game-button" 
                                                    disabled={self.guess_input.trim().is_empty()}
                                                    on_click=|_| Msg::SubmitGuess>
                                                    {text("Guess!")}
                                                </button>
                                            </div>
                                            <div style="color: #666; margin-bottom: 15px; font-size: 14px;">
                                                {text("Quick options:")}
                                            </div>
                                            <div style="display: flex; gap: 6px; justify-content: center; flex-wrap: wrap;">
                                                <button class="small-button" on_click=|_| Msg::GuessNumber(25)>{text("25")}</button>
                                                <button class="small-button" on_click=|_| Msg::GuessNumber(50)>{text("50")}</button>
                                                <button class="small-button" on_click=|_| Msg::GuessNumber(75)>{text("75")}</button>
                                                <button class="small-button" on_click=|_| Msg::GuessNumber(1)>{text("1")}</button>
                                                <button class="small-button" on_click=|_| Msg::GuessNumber(100)>{text("100")}</button>
                                            </div>
                                        </div>
                                    }
                                } else {
                                    node! {
                                        <button class="game-button" on_click=|_| Msg::NewGuessGame>
                                            {text("🎯 New Game")}
                                        </button>
                                    }
                                }
                            }
                        </div>
                    </div>
                    
                    // Memory Card Game
                    <div class="game-card" style="grid-column: 1 / -1;">
                        <h3 style="text-align: center; margin-bottom: 20px; color: #4f46e5;">
                            {text("🧠 Memory Card Game")}
                        </h3>
                        <div style="text-align: center;">
                            <div style="display: flex; gap: 20px; justify-content: center; margin-bottom: 20px;">
                                <div>{text(format!("Moves: {}", self.memory_moves))}</div>
                                <div>{text(format!("Matches: {}/{}", self.memory_matches, 8))}</div>
                            </div>
                            {
                                if self.memory_cards.is_empty() {
                                    node! {
                                        <button class="game-button" on_click=|_| Msg::StartMemoryGame>
                                            {text("🧠 Start Game")}
                                        </button>
                                    }
                                } else if self.memory_game_over {
                                    node! {
                                        <div>
                                            <div style="font-size: 24px; color: #4f46e5; margin-bottom: 20px;">
                                                {text(format!("🎉 You won in {} moves!", self.memory_moves))}
                                            </div>
                                            <button class="game-button" on_click=|_| Msg::ResetMemoryGame>
                                                {text("🔄 New Game")}
                                            </button>
                                        </div>
                                    }
                                } else {
                                    self.render_memory_cards()
                                }
                            }
                        </div>
                    </div>
                </div>
            </div>
        }
    }
    
    fn render_memory_cards(&self) -> Node<Msg> {
        node! {
            <div>
                <div style="display: grid; grid-template-columns: repeat(4, 1fr); gap: 10px; max-width: 400px; margin: 0 auto 20px;">
                    {self.render_memory_card(0)}
                    {self.render_memory_card(1)}
                    {self.render_memory_card(2)}
                    {self.render_memory_card(3)}
                    {self.render_memory_card(4)}
                    {self.render_memory_card(5)}
                    {self.render_memory_card(6)}
                    {self.render_memory_card(7)}
                    {self.render_memory_card(8)}
                    {self.render_memory_card(9)}
                    {self.render_memory_card(10)}
                    {self.render_memory_card(11)}
                    {self.render_memory_card(12)}
                    {self.render_memory_card(13)}
                    {self.render_memory_card(14)}
                    {self.render_memory_card(15)}
                </div>
                <button class="game-button" on_click=|_| Msg::ResetMemoryGame>
                    {text("🔄 Reset")}
                </button>
            </div>
        }
    }
    
    fn render_memory_card(&self, index: usize) -> Node<Msg> {
        if index >= self.memory_cards.len() {
            return node! {
                <button class="memory-card" style="background: #f0f0f0; color: #999;" disabled=true>
                    {text("?")}
                </button>
            };
        }
        
        let (emoji, revealed, matched) = &self.memory_cards[index];
        let card_style = if *revealed || *matched { 
            "background: #4f46e5; color: white;" 
        } else { 
            "background: #ddd; color: #ddd;" 
        };
        let card_text = if *revealed || *matched { emoji.clone() } else { "?".to_string() };
        
        node! {
            <button 
                class="memory-card"
                style={card_style}
                disabled={*revealed || *matched}
                on_click=move |_| Msg::MemoryCardClick(index)>
                {text(card_text)}
            </button>
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
                        <li>{text("Mini games collection (Rock Paper Scissors, Number Guessing, Memory Cards)")}</li>
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
            // Games logic
            Msg::RockPaperScissorsPlay(player_choice) => {
                console::log_1(&format!("Player chose: {}", player_choice).into());
                let computer_choice = Self::get_rps_computer_choice();
                let result = Self::determine_rps_winner(&player_choice, &computer_choice);
                
                // Update score
                if result.contains("You win") {
                    self.rps_score.0 += 1;
                } else if result.contains("Computer wins") {
                    self.rps_score.1 += 1;
                }
                
                self.rps_player_choice = Some(player_choice);
                self.rps_computer_choice = Some(computer_choice);
                self.rps_result = Some(result);
            },
            Msg::GuessInputChanged(value) => {
                self.guess_input = value;
            },
            Msg::SubmitGuess => {
                if let Ok(num) = self.guess_input.trim().parse::<i32>() {
                    if num >= 1 && num <= 100 {
                        console::log_1(&format!("Player guessed: {}", num).into());
                        self.guess_attempts += 1;
                        self.guess_input.clear(); // Clear input after guess
                        
                        if num == self.guess_target {
                            self.guess_message = format!("🎉 Correct! You found {} in {} attempts!", self.guess_target, self.guess_attempts);
                            self.guess_game_over = true;
                        } else if num < self.guess_target {
                            self.guess_message = format!("📈 Too low! Try a higher number.");
                        } else {
                            self.guess_message = format!("📉 Too high! Try a lower number.");
                        }
                    } else {
                        self.guess_input.clear();
                        self.guess_message = "Please enter a number between 1 and 100!".to_string();
                    }
                } else {
                    self.guess_input.clear();
                    self.guess_message = "Please enter a valid number!".to_string();
                }
            },
            Msg::GuessNumber(guess) => {
                console::log_1(&format!("Player guessed: {}", guess).into());
                self.guess_attempts += 1;
                self.guess_input.clear(); // Clear input after guess
                
                if guess == self.guess_target {
                    self.guess_message = format!("🎉 Correct! You found {} in {} attempts!", self.guess_target, self.guess_attempts);
                    self.guess_game_over = true;
                } else if guess < self.guess_target {
                    self.guess_message = format!("📈 Too low! Try a higher number.");
                } else {
                    self.guess_message = format!("📉 Too high! Try a lower number.");
                }
            },
            Msg::NewGuessGame => {
                console::log_1(&"Starting new guess game".into());
                self.guess_target = Self::random_number(1, 100);
                self.guess_attempts = 0;
                self.guess_message = "Guess a number between 1 and 100!".to_string();
                self.guess_game_over = false;
                self.guess_input.clear();
            },
            Msg::StartMemoryGame => {
                console::log_1(&"Starting memory game".into());
                self.memory_cards = Self::initialize_memory_cards();
                self.memory_moves = 0;
                self.memory_matches = 0;
                self.memory_first_card = None;
                self.memory_game_over = false;
                self.memory_second_card = None;
                self.memory_showing_pair = false;
            },
            Msg::MemoryCardClick(index) => {
                console::log_1(&format!("Memory card {} clicked", index).into());
                
                // Don't allow clicks if card is already revealed/matched
                if self.memory_cards[index].1 || self.memory_cards[index].2 {
                    return Cmd::none();
                }
                
                // If we're showing a non-matching pair, hide them first
                if self.memory_showing_pair {
                    if let (Some(first_index), Some(second_index)) = (self.memory_first_card, self.memory_second_card) {
                        if !self.memory_cards[first_index].2 && !self.memory_cards[second_index].2 {
                            self.memory_cards[first_index].1 = false;
                            self.memory_cards[second_index].1 = false;
                        }
                    }
                    self.memory_first_card = None;
                    self.memory_second_card = None;
                    self.memory_showing_pair = false;
                }
                
                // Reveal the clicked card
                self.memory_cards[index].1 = true;
                
                match self.memory_first_card {
                    None => {
                        // First card of the pair
                        self.memory_first_card = Some(index);
                        console::log_1(&format!("First card selected: {}", index).into());
                    },
                    Some(first_index) => {
                        // Second card of the pair
                        self.memory_second_card = Some(index);
                        self.memory_moves += 1;
                        self.memory_showing_pair = true;
                        
                        console::log_1(&format!("Second card selected: {}, checking match...", index).into());
                        
                        if self.memory_cards[first_index].0 == self.memory_cards[index].0 {
                            // Match found!
                            console::log_1(&"Match found!".into());
                            self.memory_cards[first_index].2 = true; // Mark as matched
                            self.memory_cards[index].2 = true; // Mark as matched
                            self.memory_matches += 1;
                            
                            // Reset for next turn
                            self.memory_first_card = None;
                            self.memory_second_card = None;
                            self.memory_showing_pair = false;
                            
                            // Check if game is complete
                            if self.memory_matches >= 8 {
                                self.memory_game_over = true;
                                console::log_1(&"Game completed!".into());
                            }
                        } else {
                            // No match - cards will be hidden after a brief delay
                            console::log_1(&"No match, cards will be hidden shortly".into());
                            // The cards remain visible for now - they'll be hidden when the next card is clicked
                            // or when the user resets. This provides the visual feedback needed.
                        }
                    }
                }
            },
            Msg::HideMemoryCards => {
                console::log_1(&"Hiding non-matching cards".into());
                if let (Some(first_index), Some(second_index)) = (self.memory_first_card, self.memory_second_card) {
                    // Hide both cards if they don't match
                    if !self.memory_cards[first_index].2 && !self.memory_cards[second_index].2 {
                        self.memory_cards[first_index].1 = false;
                        self.memory_cards[second_index].1 = false;
                    }
                }
                
                // Reset for next turn
                self.memory_first_card = None;
                self.memory_second_card = None;
                self.memory_showing_pair = false;
            },
            Msg::ResetMemoryGame => {
                console::log_1(&"Resetting memory game".into());
                self.memory_cards.clear();
                self.memory_moves = 0;
                self.memory_matches = 0;
                self.memory_first_card = None;
                self.memory_game_over = false;
                self.memory_second_card = None;
                self.memory_showing_pair = false;
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
