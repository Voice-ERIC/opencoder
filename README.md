# Qwen Coder Web

A Rust-based web application inspired by qwen-coder, featuring task decomposition, multi-prompt management, Gitea integration, and OpenAI-compatible API support.

## Features

- **Chat Interface**: Interact with AI models using OpenAI-compatible API
- **Task Decomposition**: Break down complex tasks into manageable subtasks using AI
- **Multi-Prompt Management**: Create, save, and reuse prompt templates with variable support
- **Gitea Integration**: Connect to Gitea instances to manage issues and pull requests
- **Customizable API**: Configure your own API endpoint, key, model name, and more

## Project Structure

```
qwen-coder-web/
├── Cargo.toml          # Rust dependencies and project metadata
├── src/
│   ├── main.rs         # Application entry point and routing
│   ├── config.rs       # Configuration management
│   ├── handlers.rs     # HTTP request handlers
│   ├── models.rs       # Data models and structures
│   ├── services.rs     # Service layer (Gitea models)
│   └── gitea/
│       ├── mod.rs      # Gitea module
│       └── handlers.rs # Gitea API handlers
├── static/
│   └── index.html      # Frontend web interface
└── README.md           # This file
```

## Requirements

- Rust 1.70 or later
- An OpenAI-compatible API endpoint (OpenAI, local LLM server, etc.)
- (Optional) Gitea instance for repository management

## Configuration

### Environment Variables

Create a `.env` file in the project root:

```env
# API Configuration
API_ENDPOINT=https://api.openai.com/v1
API_KEY=your-api-key-here
MODEL_NAME=gpt-3.5-turbo
ORGANIZATION_ID=org-xxx (optional)
TIMEOUT_SECS=60

# Gitea Configuration (optional)
GITEA_URL=https://gitea.com
GITEA_TOKEN=your-gitea-token
GITEA_DEFAULT_OWNER=username
```

### Runtime Configuration

The web interface provides a configuration tab where you can:
- Set API endpoint URL
- Configure API key
- Choose default model
- Set timeout values
- Configure Gitea connection

## Building and Running

```bash
# Build the project
cargo build --release

# Run the server
cargo run --release

# Or with custom port
PORT=8080 cargo run --release
```

The server will start on `http://localhost:3000` by default.

## API Endpoints

### Core API

- `GET /health` - Health check
- `GET /api/config` - Get current API configuration
- `POST /api/config` - Save API configuration
- `POST /api/chat/completions` - Chat completion (OpenAI-compatible)
- `POST /api/tasks/decompose` - Decompose a task into subtasks

### Prompt Management

- `GET /api/prompts` - List all prompts
- `POST /api/prompts` - Create a new prompt
- `GET /api/prompts/:id` - Get a specific prompt
- `PUT /api/prompts/:id` - Update a prompt
- `DELETE /api/prompts/:id` - Delete a prompt

### Gitea Integration

- `GET /api/gitea/repos` - List repositories
- `GET /api/gitea/repos/:owner/:repo/issues` - List issues
- `POST /api/gitea/repos/:owner/:repo/issues` - Create an issue
- `GET /api/gitea/repos/:owner/:repo/pulls` - List pull requests
- `GET /api/gitea/config` - Get Gitea configuration
- `POST /api/gitea/config` - Save Gitea configuration

## Usage

### Chat Interface

1. Navigate to the Chat tab
2. Configure the model and system prompt if needed
3. Enter your message and click "Send Message"
4. View the AI response in the result panel

### Task Decomposition

1. Go to the Tasks tab
2. Enter a task title and description
3. Set the maximum number of subtasks
4. Click "Decompose Task"
5. Review the generated subtasks

### Prompt Management

1. Navigate to the Prompts tab
2. Create a new prompt with a name, system message, and content
3. Use `{{variable}}` syntax for placeholders
4. Click on saved prompts to use them in the chat

### Gitea Integration

1. Configure Gitea credentials in the Config tab
2. Go to the Gitea tab
3. Enter owner/organization and repository name
4. List issues or pull requests

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
