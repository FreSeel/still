# Still. Real-Time Image Gallery

SMall Lightweight real time image gallery using **Rust** for the backend and **React** for the frontend. Built on a simple architecture using no service layers, databases or extra complexity.

## Setup

### Prerequisites

- pnpm
- Rust and Cargo

### Frontend

From the project root:

```bash
pnpm install
pnpm run dev
```

Vite server runs normally at `http://localhost:5173`

### Backend

Open separate terminal and run

```bash
cd backend
cargo run
```

Vite proxy is configured to connect to the backend at `http://127.0.0.1:8080`

## Design Decisions

<!-- Add Draw.io Diagram here -->

### Frontend

Built with React, TypeScript, Tailwind CSS and shadcn/ui components.

- **Gallery**: Displays the images on newest first and supports filtering.
- **Upload Dialog:** Uploads images with required title and tags.
- **WebSocket connection** Listens for new image notifications and refreshes gallery.

### Backend

Built with Rust, Axum and Tokio.

- `handlers/uploads.rs` - handles multipart image uploads.
- `handlers/images.rs` - retrieves gallery metadata.
- `handlers/websocket.rs` - manages live WebSocket connections.
- `image_storage.rs` - Handles image validation, file storage and JSON metada.
- `routes.rs` - Registers HTTP and WebSocket routes.

#### API Documentation

| Endpoint             | Purpose                          | Response                |
| -------------------- | -------------------------------- | ----------------------- |
| GET /images          | Fetch initial feed, newest first | 200 Ok                  |
| POST /uploads        | Upload and save an image         | 201 Created             |
| GET /images/:id/file | Retrieve image contents          | 200 Ok                  |
| GET /ws/images       | Establish WebSocket connection   | 101 Switching Protocols |
| GET /health          | Backend health check             | 200 Ok                  |

### Storage

Images and metadata are stored locally:

```bash
public/
└── images/
    ├── <uuid1>.jpg
    ├── <uuid1>.json
    ├── <uuid2>.png
    └── <uuid2>.json
```

Each image has a corresponding JSON file containing title and tags:

```json
{
    "title": "Photo Name"
    "tags": ["Some", "tags"]
}
```
