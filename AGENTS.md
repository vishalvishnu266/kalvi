# Kalvi ERP - Architecture & Roadmap (Rewrite v2)

This document outlines the principles and roadmap for the "Ground Up" rewrite of Kalvi ERP.

## 1. Core Principles
- **Native Rust**: Avoid unnecessary macros and "magical" libraries (e.g., no `sqlx::migrate!`, no `regex`, no `validator`).
- **Lean Dependencies**: Stick to `axum`, `tokio`, `sqlx` (core), `serde`, `uuid`, `bcrypt`, `cookie`.
- **Identity First**: Every request has a unique **Correlation ID** (UUID) for tracing.
- **Exceptions**: 
    - `BusinessException`: User-facing validation errors (inline/panel).
    - `RuntimeException`: Logged to console with ID; generic "Oops" page for users.
- **Theming**: Tailwind CSS + Pure JS switching (LocalStorage + CSS Variables).

## 2. Technical Roadmap

### Phase 1: Foundation (Complete)
- [x] **Step 1**: Scaffolding & Dependency Setup
- [x] **Step 2**: Identity & Correlation Tracking (UUID Middleware)
- [x] **Step 3**: Modern Error Handling (Business vs Runtime Exceptions)

### Phase 2: Multi-Tenant Engine (Complete)
- [x] **Step 4**: Dynamic Database Registry (Master & Tenant DBs)
- [x] **Step 5**: Multi-Tenant Middleware (URL-based resolution)

### Phase 3: UI & Theming (Current)
- [ ] **Step 6**: Responsive Layout & JS Theme Engine
- [ ] **Step 7**: Shared Component Library (Tailwind)

### Phase 4: Auth & Lifecycle (Complete)
- [x] **Step 8**: SaaS Control Plane (Onboarding & Login)
- [x] **Step 9**: Tenant Lifecycle & Session Management

### Phase 5: Verification (Current)
- [ ] **Step 10**: "Add Student" Feature (Proof of Architecture)
