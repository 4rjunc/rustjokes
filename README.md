# 🃏 Rustacean Jokes API 🦀

## Project Overview

A fun Rust-based microservice that serves random tech and programmer jokes using Actix Web and Shuttle for deployment.

## Features

- 🌐 Simple REST API
- 🎲 Random joke generator
- 🚀 Easy deployment with Shuttle
- 🔤 Full emoji support

## Prerequisites

- Rust (latest stable version)
- Cargo
- shuttle-cli (`cargo install shuttle-cli`)

## Project Structure

```
rustjokes/
│
├── Cargo.toml
├── src/
│   └── main.rs
└── jokes.json
```

## Installation

1. Clone the repository:

```bash
git clone https://github.com/4rjunc/rustjokes.git
cd rustjokes
```

2. Install dependencies:

```bash
cargo build
```

## Running Locally

```bash
shuttle run
```

## Endpoints

### 1. Home Endpoint

- **URL:** `/`
- **Method:** GET
- **Response:** "hello rustaceans! 🦀"

### 2. Joke Endpoint

- **URL:** `/joke`
- **Method:** GET
- **Response:** Random joke from `jokes.json`

## Sample GET Requests

### Using curl

```bash
# Home endpoint
curl http://localhost:8000/

# Joke endpoint
curl http://localhost:8000/joke
```

### Using httpie

```bash
# Home endpoint
http GET http://localhost:8000/

# Joke endpoint
http GET http://localhost:8000/joke
```

### Example Response for Joke Endpoint

```
Why do programmers prefer dark mode? Because light attracts bugs! 🐞🌚
```

## Deployment

Deploy using Shuttle:

```bash
shuttle deploy
```

## Dependencies

- `actix-web`: Web framework
- `rand`: Random selection
- `serde`: JSON serialization
- `shuttle-actix-web`: Deployment platform

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Contact

Your Name - [@4rjunc](https://x.com/4rjunc)

Project Link: [https://github.com/4rjunc/rustjokes](https://github.com/4rjunc/rustjokes)
