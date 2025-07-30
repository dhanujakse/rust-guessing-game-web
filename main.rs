use warp::Filter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rand::Rng;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct GameState {
    secret_number: u32,
    attempts: u32,
    game_won: bool,
    previous_guesses: Vec<u32>,
}

#[derive(Debug, Deserialize)]
struct GuessRequest {
    guess: u32,
}

#[derive(Debug, Serialize)]
struct GuessResponse {
    message: String,
    attempts: u32,
    game_won: bool,
    previous_guesses: Vec<u32>,
}

type Games = Arc<Mutex<HashMap<String, GameState>>>;

#[tokio::main]
async fn main() {
    let games: Games = Arc::new(Mutex::new(HashMap::new()));

    // Serve static HTML file
    let static_files = warp::path::end()
        .map(|| {
            warp::reply::html(include_str!("index.html"))
        });

    // New game endpoint
    let new_game = warp::path("new_game")
        .and(warp::post())
        .and(with_games(games.clone()))
        .and_then(create_new_game);

    // Make guess endpoint
    let make_guess = warp::path("guess")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_games(games.clone()))
        .and_then(handle_guess);

    let routes = static_files
        .or(new_game)
        .or(make_guess)
        .with(warp::cors().allow_any_origin());

    println!("🎯 Guessing Game Server running at http://localhost:8000");
    println!("Open your browser and go to: http://localhost:8000");

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8000))
        .await;
}

fn with_games(games: Games) -> impl Filter<Extract = (Games,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || games.clone())
}

async fn create_new_game(games: Games) -> Result<impl warp::Reply, warp::Rejection> {
    let game_id = "player1".to_string(); // Simple single-player for now
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    let new_game = GameState {
        secret_number,
        attempts: 0,
        game_won: false,
        previous_guesses: Vec::new(),
    };

    games.lock().unwrap().insert(game_id, new_game);
    
    Ok(warp::reply::json(&serde_json::json!({
        "message": "New game started! Guess a number between 1 and 100.",
        "game_id": "player1"
    })))
}

async fn handle_guess(
    guess_req: GuessRequest,
    games: Games,
) -> Result<impl warp::Reply, warp::Rejection> {
    let game_id = "player1".to_string();
    let mut games_map = games.lock().unwrap();
    
    if let Some(game) = games_map.get_mut(&game_id) {
        if game.game_won {
            return Ok(warp::reply::json(&GuessResponse {
                message: "Game already won! Start a new game.".to_string(),
                attempts: game.attempts,
                game_won: true,
                previous_guesses: game.previous_guesses.clone(),
            }));
        }

        let user_guess = guess_req.guess;
        
        if user_guess < 1 || user_guess > 100 {
            return Ok(warp::reply::json(&GuessResponse {
                message: "❌ Please enter a number between 1 and 100!".to_string(),
                attempts: game.attempts,
                game_won: false,
                previous_guesses: game.previous_guesses.clone(),
            }));
        }

        if game.previous_guesses.contains(&user_guess) {
            return Ok(warp::reply::json(&GuessResponse {
                message: "🔄 You already guessed that number! Try a different one.".to_string(),
                attempts: game.attempts,
                game_won: false,
                previous_guesses: game.previous_guesses.clone(),
            }));
        }

        game.attempts += 1;
        game.previous_guesses.push(user_guess);

        let message = if user_guess == game.secret_number {
            game.game_won = true;
            format!("🎉 Congratulations! You guessed it in {} attempts!", game.attempts)
        } else if user_guess < game.secret_number {
            format!("📈 Too low! Try a higher number. (Attempt {})", game.attempts)
        } else {
            format!("📉 Too high! Try a lower number. (Attempt {})", game.attempts)
        };

        Ok(warp::reply::json(&GuessResponse {
            message,
            attempts: game.attempts,
            game_won: game.game_won,
            previous_guesses: game.previous_guesses.clone(),
        }))
    } else {
        Ok(warp::reply::json(&GuessResponse {
            message: "No game found. Please start a new game.".to_string(),
            attempts: 0,
            game_won: false,
            previous_guesses: Vec::new(),
        }))
    }
}
