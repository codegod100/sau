use sauron::{html::text, node, Node, wasm_bindgen::JsCast};
use web_sys::{console, HtmlInputElement};

#[derive(Debug, Clone, PartialEq)]
pub enum GameMsg {
    RockPaperScissorsPlay(String),
    GuessNumber(i32),
    GuessInputChanged(String),
    SubmitGuess,
    NewGuessGame,
    MemoryCardClick(usize),
    StartMemoryGame,
    ResetMemoryGame,
    HideMemoryCards,
}

#[derive(Debug, Clone)]
pub struct GameState {
    // Rock Paper Scissors
    pub rps_player_choice: Option<String>,
    pub rps_computer_choice: Option<String>,
    pub rps_result: Option<String>,
    pub rps_score: (i32, i32), // (player_wins, computer_wins)
    
    // Number Guessing
    pub guess_target: i32,
    pub guess_attempts: i32,
    pub guess_message: String,
    pub guess_game_over: bool,
    pub guess_input: String,
    
    // Memory Cards
    pub memory_cards: Vec<(String, bool, bool)>, // (emoji, revealed, matched)
    pub memory_moves: i32,
    pub memory_matches: i32,
    pub memory_first_card: Option<usize>,
    pub memory_game_over: bool,
    pub memory_second_card: Option<usize>,
    pub memory_showing_pair: bool,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
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
    
    pub fn update(&mut self, msg: GameMsg) {
        match msg {
            GameMsg::RockPaperScissorsPlay(player_choice) => {
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
            GameMsg::GuessInputChanged(value) => {
                self.guess_input = value;
            },
            GameMsg::SubmitGuess => {
                if let Ok(num) = self.guess_input.trim().parse::<i32>() {
                    if num >= 1 && num <= 100 {
                        console::log_1(&format!("Player guessed: {}", num).into());
                        self.guess_attempts += 1;
                        self.guess_input.clear();
                        
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
            GameMsg::GuessNumber(guess) => {
                console::log_1(&format!("Player guessed: {}", guess).into());
                self.guess_attempts += 1;
                self.guess_input.clear();
                
                if guess == self.guess_target {
                    self.guess_message = format!("🎉 Correct! You found {} in {} attempts!", self.guess_target, self.guess_attempts);
                    self.guess_game_over = true;
                } else if guess < self.guess_target {
                    self.guess_message = format!("📈 Too low! Try a higher number.");
                } else {
                    self.guess_message = format!("📉 Too high! Try a lower number.");
                }
            },
            GameMsg::NewGuessGame => {
                console::log_1(&"Starting new guess game".into());
                self.guess_target = Self::random_number(1, 100);
                self.guess_attempts = 0;
                self.guess_message = "Guess a number between 1 and 100!".to_string();
                self.guess_game_over = false;
                self.guess_input.clear();
            },
            GameMsg::StartMemoryGame => {
                console::log_1(&"Starting memory game".into());
                self.memory_cards = Self::initialize_memory_cards();
                self.memory_moves = 0;
                self.memory_matches = 0;
                self.memory_first_card = None;
                self.memory_game_over = false;
                self.memory_second_card = None;
                self.memory_showing_pair = false;
            },
            GameMsg::MemoryCardClick(index) => {
                console::log_1(&format!("Memory card {} clicked", index).into());
                
                // Don't allow clicks if card is already revealed/matched
                if self.memory_cards[index].1 || self.memory_cards[index].2 {
                    return;
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
                            self.memory_cards[first_index].2 = true;
                            self.memory_cards[index].2 = true;
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
                            // No match - cards will be hidden when next card is clicked
                            console::log_1(&"No match, cards will be hidden shortly".into());
                        }
                    }
                }
            },
            GameMsg::HideMemoryCards => {
                console::log_1(&"Hiding non-matching cards".into());
                if let (Some(first_index), Some(second_index)) = (self.memory_first_card, self.memory_second_card) {
                    if !self.memory_cards[first_index].2 && !self.memory_cards[second_index].2 {
                        self.memory_cards[first_index].1 = false;
                        self.memory_cards[second_index].1 = false;
                    }
                }
                
                self.memory_first_card = None;
                self.memory_second_card = None;
                self.memory_showing_pair = false;
            },
            GameMsg::ResetMemoryGame => {
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
    }
    
    pub fn render_games_page<MSG>(&self) -> Node<MSG> 
    where 
        MSG: From<GameMsg> + 'static
    {
        node! {
            <div>
                <h2 class="text-center mb-8 text-gray-800">
                    {text("Mini Games Collection")}
                </h2>
                
                <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-5 max-w-6xl mx-auto">
                    {self.render_rock_paper_scissors()}
                    {self.render_number_guessing()}
                    {self.render_memory_game()}
                </div>
            </div>
        }
    }
    
    fn render_rock_paper_scissors<MSG>(&self) -> Node<MSG> 
    where 
        MSG: From<GameMsg> + 'static
    {
        node! {
            <div class="game-card">
                <h3 class="text-center mb-5 text-blue-600">
                    {text("🪨 Rock Paper Scissors ✂️")}
                </h3>
                <div class="text-center mb-4">
                    <div class="text-lg mb-2.5">
                        {text(format!("Score: You {} - {} Computer", self.rps_score.0, self.rps_score.1))}
                    </div>
                    <div class="flex gap-2 justify-center mb-4 flex-wrap">
                        <button class="game-button" on_click=|_| MSG::from(GameMsg::RockPaperScissorsPlay("rock".to_string()))>
                            {text("🪨 Rock")}
                        </button>
                        <button class="game-button" on_click=|_| MSG::from(GameMsg::RockPaperScissorsPlay("paper".to_string()))>
                            {text("📄 Paper")}
                        </button>
                        <button class="game-button" on_click=|_| MSG::from(GameMsg::RockPaperScissorsPlay("scissors".to_string()))>
                            {text("✂️ Scissors")}
                        </button>
                    </div>
                    {
                        if let (Some(player), Some(computer), Some(result)) = (&self.rps_player_choice, &self.rps_computer_choice, &self.rps_result) {
                            node! {
                                <div class="bg-white/80 p-4 rounded-lg">
                                    <div class="mb-2.5">
                                        {text(format!("You played: {}", match player.as_str() {
                                            "rock" => "🪨 Rock",
                                            "paper" => "📄 Paper",
                                            "scissors" => "✂️ Scissors",
                                            _ => player
                                        }))}
                                    </div>
                                    <div class="mb-2.5">
                                        {text(format!("Computer played: {}", match computer.as_str() {
                                            "rock" => "🪨 Rock",
                                            "paper" => "📄 Paper",
                                            "scissors" => "✂️ Scissors",
                                            _ => computer
                                        }))}
                                    </div>
                                    <div class="font-bold text-blue-600">
                                        {text(result)}
                                    </div>
                                </div>
                            }
                        } else {
                            node! {
                                <div class="text-gray-600">
                                    {text("Choose your move!")}
                                </div>
                            }
                        }
                    }
                </div>
            </div>
        }
    }
    
    fn render_number_guessing<MSG>(&self) -> Node<MSG> 
    where 
        MSG: From<GameMsg> + 'static
    {
        
        node! {
            <div class="game-card">
                <h3 class="text-center mb-5 text-blue-600">
                    {text("🎯 Number Guessing Game")}
                </h3>
                <div class="text-center">
                    <div class="mb-4">
                        {text(format!("Attempts: {}", self.guess_attempts))}
                    </div>
                    <div class="mb-4 text-gray-600">
                        {text(&self.guess_message)}
                    </div>
                    {
                        if !self.guess_game_over {
                            node! {
                                <div>
                                    <div class="flex gap-2 justify-center items-center mb-4 flex-wrap">
                                        <input type="number" 
                                            min="1" 
                                            max="100" 
                                            placeholder="1-100" 
                                            value={&self.guess_input}
                                            class="p-2 border border-gray-300 rounded w-20 text-center"
                                            on_input=|event| {
                                                if let Some(input) = event.event.target() {
                                                    if let Ok(input_element) = input.dyn_into::<HtmlInputElement>() {
                                                        MSG::from(GameMsg::GuessInputChanged(input_element.value()))
                                                    } else {
                                                        MSG::from(GameMsg::GuessInputChanged(String::new()))
                                                    }
                                                } else {
                                                    MSG::from(GameMsg::GuessInputChanged(String::new()))
                                                }
                                            } />
                                        <button class="game-button" 
                                            disabled={self.guess_input.trim().is_empty()}
                                            on_click=|_| MSG::from(GameMsg::SubmitGuess)>
                                            {text("Guess!")}
                                        </button>
                                    </div>
                                    <div class="text-gray-600 mb-4 text-sm">
                                        {text("Quick options:")}
                                    </div>
                                    <div class="flex gap-1.5 justify-center flex-wrap">
                                        <button class="small-button" on_click=|_| MSG::from(GameMsg::GuessNumber(25))>{text("25")}</button>
                                        <button class="small-button" on_click=|_| MSG::from(GameMsg::GuessNumber(50))>{text("50")}</button>
                                        <button class="small-button" on_click=|_| MSG::from(GameMsg::GuessNumber(75))>{text("75")}</button>
                                        <button class="small-button" on_click=|_| MSG::from(GameMsg::GuessNumber(1))>{text("1")}</button>
                                        <button class="small-button" on_click=|_| MSG::from(GameMsg::GuessNumber(100))>{text("100")}</button>
                                    </div>
                                </div>
                            }
                        } else {
                            node! {
                                <button class="game-button" on_click=|_| MSG::from(GameMsg::NewGuessGame)>
                                    {text("🎯 New Game")}
                                </button>
                            }
                        }
                    }
                </div>
            </div>
        }
    }
    
    fn render_memory_game<MSG>(&self) -> Node<MSG> 
    where 
        MSG: From<GameMsg> + 'static
    {
        node! {
            <div class="game-card col-span-full">
                <h3 class="text-center mb-5 text-blue-600">
                    {text("🧠 Memory Card Game")}
                </h3>
                <div class="text-center">
                    <div class="flex gap-5 justify-center mb-5">
                        <div>{text(format!("Moves: {}", self.memory_moves))}</div>
                        <div>{text(format!("Matches: {}/{}", self.memory_matches, 8))}</div>
                    </div>
                    {
                        if self.memory_cards.is_empty() {
                            node! {
                                <button class="game-button" on_click=|_| MSG::from(GameMsg::StartMemoryGame)>
                                    {text("🧠 Start Game")}
                                </button>
                            }
                        } else if self.memory_game_over {
                            node! {
                                <div>
                                    <div class="text-2xl text-blue-600 mb-5">
                                        {text(format!("🎉 You won in {} moves!", self.memory_moves))}
                                    </div>
                                    <button class="game-button" on_click=|_| MSG::from(GameMsg::ResetMemoryGame)>
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
        }
    }
    
    fn render_memory_cards<MSG>(&self) -> Node<MSG> 
    where 
        MSG: From<GameMsg> + 'static
    {
        node! {
            <div>
                <div class="grid grid-cols-4 gap-2 max-w-80 mx-auto mb-5 px-2.5">
                    {self.render_memory_card::<MSG>(0)}
                    {self.render_memory_card::<MSG>(1)}
                    {self.render_memory_card::<MSG>(2)}
                    {self.render_memory_card::<MSG>(3)}
                    {self.render_memory_card::<MSG>(4)}
                    {self.render_memory_card::<MSG>(5)}
                    {self.render_memory_card::<MSG>(6)}
                    {self.render_memory_card::<MSG>(7)}
                    {self.render_memory_card::<MSG>(8)}
                    {self.render_memory_card::<MSG>(9)}
                    {self.render_memory_card::<MSG>(10)}
                    {self.render_memory_card::<MSG>(11)}
                    {self.render_memory_card::<MSG>(12)}
                    {self.render_memory_card::<MSG>(13)}
                    {self.render_memory_card::<MSG>(14)}
                    {self.render_memory_card::<MSG>(15)}
                </div>
                <button class="game-button" on_click=|_| MSG::from(GameMsg::ResetMemoryGame)>
                    {text("🔄 Reset")}
                </button>
            </div>
        }
    }
    
    fn render_memory_card<MSG>(&self, index: usize) -> Node<MSG> 
    where 
        MSG: From<GameMsg> + 'static
    {
        if index >= self.memory_cards.len() {
            return node! {
                <button class="memory-card bg-gray-200 text-gray-500" disabled=true>
                    {text("?")}
                </button>
            };
        }
        
        let (emoji, revealed, matched) = &self.memory_cards[index];
        let card_classes = if *revealed || *matched { 
            "memory-card bg-blue-600 text-white" 
        } else { 
            "memory-card bg-gray-300 text-gray-300" 
        };
        let card_text = if *revealed || *matched { emoji.clone() } else { "?".to_string() };
        
        node! {
            <button 
                class={card_classes}
                disabled={*revealed || *matched}
                on_click=move |_| MSG::from(GameMsg::MemoryCardClick(index))>
                {text(card_text)}
            </button>
        }
    }
}