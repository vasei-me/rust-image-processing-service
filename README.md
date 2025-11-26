# Image Processing Service

A Rust-based image processing service that provides upload, transformation, and management capabilities for images.

## ✨ Features

### 🔐 User Authentication
- User registration and login
- JWT-based authentication
- Token-protected endpoints

### 📁 Image Management
- Upload images in various formats
- List uploaded images with metadata
- Retrieve images by ID
- Cloud storage for images

### 🎨 Image Transformations
- Resize
- Crop
- Rotate
- Watermark
- Color filters (grayscale, sepia)
- Compression
- Format conversion (JPEG, PNG, etc.)

## 🛠 Technology Stack

- **Language**: Rust
- **Framework**: Actix-web
- **Database**: SQLite
- **Image Processing**: image-rs
- **Authentication**: JWT
- **Storage**: Local file system (upgradable to S3/R2)

## 🚀 Installation & Setup

### Prerequisites
- Rust 1.70 or higher
- Cargo

### Running the Project

```bash
# Clone repository
git clone <repository-url>
cd image-processing-service

# Run server
cargo run

# Or for development mode with auto-reload
cargo watch -x run
```

Server runs on `http://localhost:8080`.

## 📚 API Endpoints

### Authentication

#### Register User
```http
POST /register
Content-Type: application/json

{
  "username": "username",
  "password": "password"
}
```

#### Login User
```http
POST /login
Content-Type: application/json

{
  "username": "username",
  "password": "password"
}
```

### Image Management

#### Upload Image
```http
POST /images
Authorization: Bearer <jwt-token>
Content-Type: multipart/form-data
```

#### List Images
```http
GET /images?page=1&limit=10
Authorization: Bearer <jwt-token>
```

#### Get Image
```http
GET /images/{id}
Authorization: Bearer <jwt-token>
```

#### Apply Image Transformations
```http
POST /images/{id}/transform
Authorization: Bearer <jwt-token>
Content-Type: application/json

{
  "transformations": {
    "resize": {
      "width": 800,
      "height": 600
    },
    "crop": {
      "width": 400,
      "height": 300,
      "x": 100,
      "y": 100
    },
    "rotate": 90,
    "format": "jpeg",
    "filters": {
      "grayscale": true,
      "sepia": false
    }
  }
}
```


## 🔧 Configuration

The project uses `.env` file for configuration:

```env
DATABASE_URL=sqlite:images.db
JWT_SECRET=your-secret-key
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
UPLOAD_DIR=./uploads
```

## 🧪 Testing

To run tests:

```bash
cargo test
```

## 📦 Production Build

```bash
cargo build --release
```

This is a solution for the Image Processing Service project on roadmap.sh.

**Project URL:** https://roadmap.sh/projects/image-processing-service
