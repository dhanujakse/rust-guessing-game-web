# rust-guessing-game-web
# 🎯 Rust Number Guessing Game (Web Version)

A modern, interactive web-based number guessing game built with Rust and served through a lightweight web server. Experience the classic guessing game with a beautiful, responsive interface and real-time gameplay.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![HTML5](https://img.shields.io/badge/html5-%23E34F26.svg?style=for-the-badge&logo=html5&logoColor=white)
![CSS3](https://img.shields.io/badge/css3-%23157096.svg?style=for-the-badge&logo=css3&logoColor=white)
![JavaScript](https://img.shields.io/badge/javascript-%23323330.svg?style=for-the-badge&logo=javascript&logoColor=%23F7DF1E)

## ✨ Features

- 🎮 **Interactive Web Interface** - Beautiful, responsive design that works on all devices
- 🚀 **Fast Rust Backend** - Powered by Warp web framework for lightning-fast performance
- 🎨 **Modern UI/UX** - Gradient backgrounds, smooth animations, and intuitive controls
- 📱 **Mobile Responsive** - Optimized for both desktop and mobile gameplay
- 🎯 **Smart Feedback** - Real-time hints (too high/too low) with attempt tracking
- 📊 **Game Statistics** - Visual display of previous guesses and attempt counter
- ⚡ **Real-time Updates** - No page refreshes needed, instant feedback
- 🔒 **Input Validation** - Prevents invalid inputs and duplicate guesses

## 🚀 Quick Start

### Prerequisites

Before you begin, ensure you have the following installed:
- **Rust** (1.70.0 or later) - [Install Rust](https://rustup.rs/)
- **Git** - [Install Git](https://git-scm.com/)
- A modern web browser (Chrome, Firefox, Safari, Edge)

### Installation

1. **Clone the repository:**
git clone https://github.com/yourusername/rust-guessing-game-web.git
cd rust-guessing-game-web

2. **Install dependencies:**
cargo build

3. **Run the development server:**
cargo run


4. **Open your browser:**
Navigate to `http://localhost:8000`

5. **Start playing:**
Click "New Game" and start guessing!

## 🎮 How to Play

1. **Start a New Game** - Click the "🔄 New Game" button to begin
2. **Make Your Guess** - Enter a number between 1 and 100 in the input field
3. **Get Feedback** - Receive instant hints:
- 📈 "Too low!" - Your guess is smaller than the secret number
- 📉 "Too high!" - Your guess is larger than the secret number
- 🎉 "You win!" - Congratulations, you found it!
4. **Track Progress** - View your attempt count and previous guesses
5. **Play Again** - Start a new game anytime with a fresh secret number

## 🛠️ Technology Stack

| Component | Technology |
|-----------|------------|
| **Backend** | Rust with Warp web framework |
| **Frontend** | HTML5, CSS3, Vanilla JavaScript |
| **Styling** | Modern CSS with Flexbox and animations |
| **API** | RESTful JSON endpoints |
| **Build Tool** | Cargo (Rust's package manager) |

## 📡 API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/` | Serves the main game interface |
| `POST` | `/new_game` | Initializes a new game session |
| `POST` | `/guess` | Submits a guess and returns feedback |

### API Response Examples

**New Game Response:**
{
"message": "📈 Too low! Try a higher number. (Attempt 3)",
"attempts": 3,
"game_won": false,
"previous_guesses":
}

## 🏗️ Project Structure

rust-guessing-game-web/
├── README.md # Project documentation
├── Cargo.toml # Rust dependencies and metadata
├── src/
│ ├── main.rs # Main Rust server and game logic
│ └── index.html # Frontend interface and styling
├── .gitignore # Git ignore rules
└── LICENSE # MIT license file


## 🔧 Development

### Running in Development Mode
cargo run
Server starts on `http://localhost:8000` with auto-compilation on code changes.




