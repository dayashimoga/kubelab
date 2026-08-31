# KubeLab User Guide

## Getting Started

### Registration
1. Navigate to the web app at `http://localhost:3000`
2. Click **Sign Up** and enter email, password, and display name
3. You'll be assigned the **Learner** role by default

### Login
1. Enter your email and password
2. Receive a JWT token for authenticated API access
3. Sessions are cached in Redis with automatic expiry

## Learning Workflow

### 1. Browse Tracks
- Navigate to the **Learn** section to see all 12 curriculum tracks
- Tracks are ordered by difficulty: Foundations → Core → Advanced → Expert
- Each track shows lesson count, estimated time, and completion percentage

### 2. Study Lessons
- Select a track and lesson to begin reading
- Lessons include theory, diagrams, code examples, and key concepts
- Mark lessons complete to earn XP and track progress

### 3. Take Quizzes
- Each module includes assessment quizzes
- Multiple choice and scenario-based questions
- Instant scoring with explanations for incorrect answers

### 4. Launch Labs
- Click **Start Lab** on any lab-enabled lesson
- A disposable Kubernetes namespace is provisioned for you
- You'll see a split view: instructions on the left, terminal on the right

### 5. Use the Terminal
- The terminal is a real xterm.js shell connected via WebSocket
- Run `kubectl`, `helm`, and other CLI tools directly
- Your session is isolated — you cannot affect other learners

### 6. Edit Manifests
- Use the Monaco editor to write YAML manifests
- Click **Apply** to run `kubectl apply` against your sandbox
- Changes are reflected in the real cluster immediately

### 7. Submit for Grading
- Click **Grade** when you've completed the lab tasks
- The grading engine queries actual Kubernetes API state
- Receive a score, XP, and detailed feedback per task
- Hints are available if you're stuck

### 8. Track Progress
- View your **Profile** for XP, level, completed lessons, and badges
- The **Skill Tree** shows your competency across all domains
- Earn certifications by completing entire tracks

## Mobile App

### Installation
- Download the Android APK from GitHub Releases
- Or build from source: `cd apps/mobile && flutter build apk --release`

### Features
- Browse tracks and read lessons
- Take quizzes on the go
- View your skill tree and progress
- **Continue on Desktop** — labs requiring terminals hand off to the web app

## PWA / Offline Support
- The web app registers a service worker for offline access
- Previously loaded lessons are available without network
- Full lab functionality requires an active connection
