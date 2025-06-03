# Technical Plan
## 1. Detailed Scope Definition

### 1.1. In-Scope Features

#### 1.1.1. User Module
*   **Self-Registration:** Users can register with name, email, username, and password. Passwords will be securely hashed.
*   **Login/Authentication:** Registered users can log in using username and password. Session or token-based authentication will be implemented.
*   **Profile Management:** Authenticated users can view and edit their own name, email, and username.
*   **Logout:** Users can terminate their session.

#### 1.1.2. Admin Module
*   **Admin Login:** Administrators will have a separate or role-differentiated login mechanism.
*   **User Listing:** Admins can view a paginated and sortable list of all registered users, displaying key information like username, email, and registration date.
*   **View User Details (Admin):** Admins can view the full profile details of any specific user.
*   **User Account Management (Admin):** Admins can activate or deactivate user accounts.

### 1.2. Explicitly Excluded Features
*   Biometric authentication
*   Complex multi-factor authentication (MFA)
*   UIN (Unique Identification Number) generation or management
*   Credential issuance and verification workflows (e.g., digital certificates)
*   Advanced audit trails and logging (beyond basic request/error logging)
*   Complex role and permission systems (beyond the basic 'user' and 'admin' roles)
*   Password change/reset by admin (users manage their own passwords)
*   Direct database manipulation by admins through the UI
*   Integration with external identity providers or systems
*   Self-service password reset for users (can be considered for a future phase)
*   Email verification during registration (can be considered for a future phase)
*   File uploads or avatar management

### 1.3. Key User Flows

#### 1.3.1. Regular User Flows
1.  **Registration:**
    *   User navigates to the registration page.
    *   User fills in name, email, username, and password.
    *   System validates input.
    *   System creates a new user account with the 'user' role and stores the hashed password.
    *   User is informed of successful registration, potentially auto-logged in or redirected to login.
2.  **Login:**
    *   User navigates to the login page.
    *   User enters username and password.
    *   System validates credentials against stored hashed password.
    *   Upon success, a session/token is generated and returned to the user.
    *   User is redirected to their dashboard/profile page.
3.  **View/Edit Profile:**
    *   Authenticated user navigates to their profile page.
    *   User views their current profile information (name, email, username).
    *   User clicks an 'Edit' button.
    *   User modifies their name, email, or username.
    *   System validates input.
    *   System updates the user's profile information in the database.
    *   User is informed of the successful update.
4.  **Logout:**
    *   Authenticated user clicks the 'Logout' button.
    *   System invalidates the user's session/token.
    *   User is redirected to the login page or homepage.

#### 1.3.2. Admin User Flows
1.  **Admin Login:**
    *   Admin navigates to a specific admin login page or uses the general login page (role determined post-login).
    *   Admin enters their username and password.
    *   System validates credentials.
    *   Upon success (and if the user has 'admin' role), an admin session/token is generated.
    *   Admin is redirected to the admin dashboard.
2.  **View User List:**
    *   Authenticated admin navigates to the 'User Management' or 'Users' section.
    *   System displays a list of users with pagination and sorting options.
    *   Admin can see username, email, registration date, and account status (e.g., active/inactive).
3.  **View Specific User Details:**
    *   From the user list, admin clicks on a specific user.
    *   System displays the detailed profile of the selected user (all fields available to the user, plus administrative info like account status).
4.  **Activate/Deactivate User Account:**
    *   From the user list or user detail view, admin has an option to change the account status.
    *   Admin selects 'Activate' or 'Deactivate' for a user.
    *   System updates the user's account status in the database.
    *   The change is reflected in the user list and impacts the user's ability to log in.

## 2. System Architecture

### 2.1. Overview
The system will follow a classic client-server architecture. The frontend (React single-page application) will run in the user's browser, and the backend (Rust Axum application) will expose a RESTful API. Communication between the frontend and backend will be over HTTPS using JSON.

### 2.2. Components
*   **Frontend (React SPA):**
    *   Built with React, TypeScript, Vite.
    *   Handles user interface and user experience.
    *   Manages client-side state (Zustand).
    *   Communicates with the backend API via Axios (using Tanstack Query for data fetching and caching).
    *   Implements routing (Tanstack Router) to differentiate between public pages, user-authenticated pages, and admin-authenticated pages.
*   **Backend (Rust REST API):**
    *   Built with Axum framework.
    *   Handles business logic, data persistence, authentication, and authorization.
    *   Interacts with the MySQL database via SQLx.
    *   Exposes RESTful API endpoints.
*   **Database (MySQL):**
    *   Stores user information, including roles and credentials (hashed passwords).

### 2.3. Frontend-Backend Interaction Diagram

```
+---------------------+      HTTPS/JSON      +---------------------+
|   Frontend (React)  |<-------------------->|   Backend (Axum)    |
|---------------------|      (REST API)      |---------------------|
| - User Views        |                      | - API Endpoints     |
| - Admin Views       |                      | - Auth Middleware   |
| - Auth Pages (Login,|                      | - Role Middleware   |
|   Register)         |                      | - Business Logic    |
| - State (Zustand)   |                      | - SQLx (DB Access)  |
| - API Client (Axios)|                      |                     |
+---------------------+                      +----------^----------+
                                                       |
                                                       | MySQL
                                                       |
                                             +---------v---------+
                                             |  Database (MySQL) |
                                             |-------------------|
                                             | - Users Table     |
                                             |   (with roles)    |
                                             +-------------------+
```

### 2.4. Access Control (User vs. Admin)

Access control will be primarily enforced at the backend API level, with the frontend dynamically rendering views based on authentication status and user role.

1.  **Authentication:**
    *   Users (including admins) authenticate via a login endpoint (e.g., `/api/v1/auth/login`).
    *   Upon successful authentication, the backend issues a JWT (JSON Web Token) or a session token. This token will contain user identifying information, including their role (e.g., 'user' or 'admin').
    *   The frontend stores this token securely (e.g., in an HttpOnly cookie or local storage, depending on session/token strategy) and includes it in the `Authorization` header for subsequent API requests.

2.  **Backend Authorization Middleware:**
    *   **Token Validation Middleware:** All protected API endpoints will first pass through middleware that validates the token. If the token is missing, expired, or invalid, a `401 Unauthorized` error is returned.
    *   **Role-Based Access Control (RBAC) Middleware:**
        *   Admin-specific API endpoints (e.g., `/api/v1/admin/*`) will have an additional layer of middleware.
        *   This middleware will inspect the validated token to extract the user's role.
        *   If the role is not 'admin', a `403 Forbidden` error is returned, even if the token is valid.
        *   User-specific endpoints (e.g., `/api/v1/users/me`) will ensure the authenticated user can only access/modify their own data.

3.  **Frontend Routing and UI:**
    *   The frontend will use Tanstack Router to define protected routes.
    *   After login, the user's role (obtained from the login response or a `/users/me` call) will be stored in the Zustand store.
    *   The router will check for authentication status and role:
        *   If not authenticated, users are redirected to the login page.
        *   If authenticated as 'user', they can access user-specific routes (e.g., `/profile`). Access to admin routes will be blocked.
        *   If authenticated as 'admin', they can access both user-specific routes (if applicable) and admin-specific routes (e.g., `/admin/users`).
    *   The UI will conditionally render navigation links and components based on the user's role (e.g., an "Admin Panel" link will only be visible to admins).

This layered approach ensures that even if a non-admin user attempts to access an admin API endpoint directly (e.g., using cURL or developer tools), the backend will block the request. The frontend UI changes are for user experience but not the primary security mechanism.

## 3. API Design (RESTful)

All API endpoints will be versioned under `/api/v1/`.

### 3.1. Standard Response Formats

#### 3.1.1. Success Responses
*   **200 OK:** Standard response for successful GET, PUT, PATCH requests.
    ```json
    {
        "status": "success",
        "data": {
            // Requested data or updated resource
        }
    }
    ```
*   **201 Created:** Response for successful POST requests that create a new resource.
    ```json
    {
        "status": "success",
        "data": {
            // Created resource, often including its new ID
        }
    }
    ```
*   **204 No Content:** Response for successful requests that don't return a body (e.g., DELETE).

#### 3.1.2. Error Responses
Error responses will follow a consistent structure:
```json
{
    "status": "error",
    "message": "A human-readable error message",
    "code": "ERROR_CODE_SLUG", // Optional: A machine-readable error code
    "details": { // Optional: Additional details about the error
        // e.g., field validation errors
    }
}
```
Common HTTP status codes for errors:
*   **400 Bad Request:** Client-side error (e.g., invalid JSON, validation errors).
*   **401 Unauthorized:** Authentication is required and has failed or has not yet been provided.
*   **403 Forbidden:** Authenticated user does not have permission to access the resource.
*   **404 Not Found:** The requested resource could not be found.
*   **409 Conflict:** Request conflicts with the current state of the server (e.g., duplicate username).
*   **422 Unprocessable Entity:** The server understands the content type of the request entity, and the syntax of the request entity is correct, but it was unable to process the contained instructions (often for validation errors).
*   **500 Internal Server Error:** A generic error message, given when an unexpected condition was encountered.

### 3.2. User-Facing API Endpoints

#### 3.2.1. Authentication (`/api/v1/auth`)
*   **POST `/api/v1/auth/register`**
    *   **Description:** Registers a new user.
    *   **Request Body:**
        ```json
        {
            "name": "John Doe",
            "email": "john.doe@example.com",
            "username": "johndoe",
            "password": "securepassword123"
        }
        ```
    *   **Response (201 Created):**
        ```json
        {
            "status": "success",
            "data": {
                "id": "uuid-string",
                "username": "johndoe",
                "email": "john.doe@example.com",
                "name": "John Doe",
                "role": "user"
            }
        }
        ```
    *   **Authorization:** Public.
*   **POST `/api/v1/auth/login`**
    *   **Description:** Logs in an existing user.
    *   **Request Body:**
        ```json
        {
            "username": "johndoe",
            "password": "securepassword123"
        }
        ```
    *   **Response (200 OK):** Returns a token (e.g., JWT) and user information. The token should be sent back in an HttpOnly cookie or in the response body for client storage.
        ```json
        {
            "status": "success",
            "data": {
                "token": "your.jwt.token", // Or session ID
                "user": {
                    "id": "uuid-string",
                    "username": "johndoe",
                    "email": "john.doe@example.com",
                    "name": "John Doe",
                    "role": "user" // or "admin"
                }
            }
        }
        ```
    *   **Authorization:** Public.
*   **POST `/api/v1/auth/logout`**
    *   **Description:** Logs out the current user (invalidates token/session).
    *   **Request Body:** None.
    *   **Response (204 No Content):**
    *   **Authorization:** Authenticated (User or Admin).

#### 3.2.2. User Profile (`/api/v1/users`)
*   **GET `/api/v1/users/me`**
    *   **Description:** Gets the profile of the currently authenticated user.
    *   **Response (200 OK):**
        ```json
        {
            "status": "success",
            "data": {
                "id": "uuid-string",
                "username": "johndoe",
                "email": "john.doe@example.com",
                "name": "John Doe",
                "role": "user", // or "admin"
                "isActive": true, // Account status
                "createdAt": "timestamp",
                "updatedAt": "timestamp"
            }
        }
        ```
    *   **Authorization:** Authenticated (User or Admin).
*   **PUT `/api/v1/users/me`**
    *   **Description:** Updates the profile of the currently authenticated user.
    *   **Request Body:** (Allow partial updates for name, email, username)
        ```json
        {
            "name": "Johnathan Doe", // Optional
            "email": "johnathan.doe@example.com" // Optional
            // Username might be updatable or not, based on design decision.
            // Password changes should be through a separate endpoint if implemented.
        }
        ```
    *   **Response (200 OK):** Returns the updated user profile.
        ```json
        {
            "status": "success",
            "data": {
                // updated user object
            }
        }
        ```
    *   **Authorization:** Authenticated (User or Admin, can only update their own profile).

### 3.3. Admin-Only API Endpoints (`/api/v1/admin`)

All endpoints under `/api/v1/admin/*` require 'admin' role.

#### 3.3.1. User Management (`/api/v1/admin/users`)
*   **GET `/api/v1/admin/users`**
    *   **Description:** Lists all registered users. Supports pagination and sorting.
    *   **Query Parameters:**
        *   `page` (integer, default: 1): Page number for pagination.
        *   `limit` (integer, default: 10): Number of users per page.
        *   `sortBy` (string, e.g., `username`, `email`, `createdAt`): Field to sort by.
        *   `sortOrder` (string, `asc` or `desc`, default: `asc`): Sort order.
        *   `search` (string, optional): Search term to filter users by username, email, or name.
    *   **Response (200 OK):**
        ```json
        {
            "status": "success",
            "data": {
                "users": [
                    {
                        "id": "uuid-string-1",
                        "username": "johndoe",
                        "email": "john.doe@example.com",
                        "name": "John Doe",
                        "role": "user",
                        "isActive": true,
                        "createdAt": "timestamp"
                    },
                    {
                        "id": "uuid-string-2",
                        "username": "janedoe",
                        "email": "jane.doe@example.com",
                        "name": "Jane Doe",
                        "role": "user",
                        "isActive": false,
                        "createdAt": "timestamp"
                    }
                    // ... other users
                ],
                "pagination": {
                    "currentPage": 1,
                    "totalPages": 5,
                    "totalUsers": 50,
                    "limit": 10
                }
            }
        }
        ```
    *   **Authorization:** Admin Only.
*   **GET `/api/v1/admin/users/{userId}`**
    *   **Description:** Gets the detailed profile of a specific user by their ID.
    *   **Path Parameter:** `userId` (UUID of the user).
    *   **Response (200 OK):** (Similar to `/api/v1/users/me` but for any user)
        ```json
        {
            "status": "success",
            "data": {
                // full user object including administrative fields
            }
        }
        ```
    *   **Authorization:** Admin Only.
*   **PATCH `/api/v1/admin/users/{userId}/status`**
    *   **Description:** Activates or deactivates a user's account.
    *   **Path Parameter:** `userId` (UUID of the user).
    *   **Request Body:**
        ```json
        {
            "isActive": true // or false
        }
        ```
    *   **Response (200 OK):** Returns the updated user object.
        ```json
        {
            "status": "success",
            "data": {
                // updated user object with new isActive status
            }
        }
        ```
    *   **Authorization:** Admin Only.

*(Note: Direct password changes or full profile edits of other users by admins are explicitly excluded from this MVP's API design, aligning with the defined scope. Admins can only view details and change activation status.)*

## 4. Database Schema (MySQL)

The database will use MySQL. SQLx will be used for compile-time checked queries against this schema.

### 4.1. Tables

#### 4.1.1. `users` Table
This table stores information about all registered users, including administrators.

| Column Name     | Data Type                | Constraints & Properties                                  | Description                                     |
|-----------------|--------------------------|-----------------------------------------------------------|-------------------------------------------------|
| `id`            | `VARCHAR(36)`            | `PRIMARY KEY`, `NOT NULL`                                 | UUID for the user (e.g., generated by `uuid` crate). |
| `username`      | `VARCHAR(50)`            | `UNIQUE`, `NOT NULL`                                      | Unique username for login.                      |
| `email`         | `VARCHAR(255)`           | `UNIQUE`, `NOT NULL`                                      | User's email address.                           |
| `password_hash` | `VARCHAR(255)`           | `NOT NULL`                                                | Hashed password (e.g., using bcrypt or argon2). |
| `name`          | `VARCHAR(100)`           | `NOT NULL`                                                | User's full name or display name.               |
| `role`          | `ENUM('user', 'admin')`  | `NOT NULL`, `DEFAULT 'user'`                              | User role, determining access level.            |
| `is_active`     | `BOOLEAN`                | `NOT NULL`, `DEFAULT TRUE`                                | Whether the user account is active or disabled. |
| `created_at`    | `TIMESTAMP`              | `NOT NULL`, `DEFAULT CURRENT_TIMESTAMP`                   | Timestamp of user creation.                     |
| `updated_at`    | `TIMESTAMP`              | `NOT NULL`, `DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP` | Timestamp of last user update.                  |

**Indexes:**
*   Primary Key on `id`.
*   Unique Index on `username`.
*   Unique Index on `email`.
*   Index on `role` (for filtering admins or users).
*   Index on `is_active`.
*   Index on `created_at` (for sorting by registration date).

### 4.2. Relationships
*   Currently, there is only one primary table (`users`). No direct foreign key relationships are defined in this MVP scope between different tables.

### 4.3. Data Types Rationale
*   `VARCHAR(36)` for `id`: Standard length for UUID v4 strings.
*   `VARCHAR(50)` for `username`: A reasonable limit for usernames.
*   `VARCHAR(255)` for `email`: Standard max length for email addresses.
*   `VARCHAR(255)` for `password_hash`: Sufficient to store hashes from bcrypt/argon2.
*   `ENUM('user', 'admin')` for `role`: Ensures data integrity for roles and is efficient.
*   `BOOLEAN` for `is_active`: Clear representation of active/inactive status.
*   `TIMESTAMP` for `created_at`/`updated_at`: Standard for tracking record changes.

### 4.4. Initial Admin User Seeding
An initial admin user needs to be created when the application is first set up or when database migrations are run. This can be achieved through:
1.  **Database Migration Script (SQLx Migrations):** Include an SQL `INSERT` statement in a migration file that runs after the `users` table is created. The password should be pre-hashed using the chosen algorithm.
    *Example (conceptual, actual hashing done by application logic before insertion or a script):*
    ```sql
    -- This is a conceptual SQL, the password_hash would be generated by a script/tool
    -- using the same hashing library as the application (e.g., bcrypt)
    INSERT INTO users (id, username, email, password_hash, name, role, is_active)
    VALUES ('your-generated-uuid', 'admin', 'admin@example.com', 'pre_hashed_admin_password', 'Administrator', 'admin', TRUE);
    ```
2.  **Application Startup Logic (Less Ideal for Production):** A special command or logic within the application that, on first run or via a specific command, creates the admin user if one doesn't exist. This is generally less robust than migration-based seeding for production.

**Recommendation:** Use SQLx migrations to seed the initial admin user. The `.env.example` should specify default credentials for this admin user (e.g., `ADMIN_USERNAME`, `ADMIN_PASSWORD`), and a setup script or manual step will be required to hash this password and place it into the migration or a seeding script. For security, the actual plain text password for the default admin should not be directly in the migration file if the repo is public. A better approach is to have a setup script that prompts for an admin password, hashes it, and then injects it into the seed data or a one-time seed command.

For development, a fixed hashed password for a default admin in a migration is acceptable.

## 5. Technology Stack Confirmation & Configuration

### 5.1. Frontend (React)

*   **Build Tool: pnpm with Workspaces**
    *   **Configuration:** A `pnpm-workspace.yaml` file will define the workspace, likely containing a `frontend` package (and potentially a shared `packages/common` or `packages/ui-core` later if needed, though not strictly for this MVP).
    *   **Setup:** Each package (e.g., `frontend`) will have its own `package.json`. Root `package.json` for shared dev dependencies.
    *   **Benefit:** Efficient installation and linking of local packages.
*   **UI Framework: React (with TypeScript)**
    *   **Setup:** Standard React project setup using Vite for fast development and optimized builds. `tsconfig.json` configured for strict type checking.
*   **UI Components: shadcn/ui**
    *   **Configuration:** Components will be added via the `shadcn/ui` CLI, which copies component code directly into the project for full customization. Tailwind CSS will be configured as required by shadcn/ui.
    *   **Usage:** Used for building the user interface elements (forms, tables, buttons, modals, etc.).
*   **State Management: Zustand**
    *   **Store Structure:**
        *   `authStore`: Manages authentication state (e.g., `isAuthenticated`, `token`, `currentUser`, `role`). Actions for `login`, `logout`, `setAuthUser`.
        *   `userProfileStore` (Optional, if not fully handled by Tanstack Query): For managing temporary state related to user profile editing if needed.
        *   `adminStore` (Optional, specific to admin section state): Could manage state for admin user list filters, pagination, or selected user details if complex.
    *   **Role Handling:** The `authStore` will hold the current user's role (`user` or `admin`), derived from the login response. This role will be used for conditional rendering and routing logic.
*   **API Communication: Axios**
    *   **Configuration:**
        *   Create an Axios instance with a `baseURL` (e.g., `/api/v1`) configured from environment variables (`VITE_API_BASE_URL`).
        *   Implement request interceptors to automatically attach the auth token (from `authStore` or secure storage) to `Authorization` headers.
        *   Implement response interceptors for global error handling (e.g., redirecting to login on 401 errors, showing generic error messages).
*   **Data Fetching & Caching: Tanstack Query (React Query)**
    *   **Cache Setup:**
        *   Default `QueryClient` configuration with sensible `staleTime` and `cacheTime` for different types of data (e.g., user profile, admin user list).
        *   Used for fetching, caching, and synchronizing server state for:
            *   User profile data (`/api/v1/users/me`).
            *   Admin user list (`/api/v1/admin/users`) with support for pagination and filtering parameters as query keys.
    *   **Usage:** `useQuery` for data fetching, `useMutation` for data modification (e.g., profile updates, admin actions).
*   **Routing: Tanstack Router (React Router)**
    *   **Configuration:**
        *   Define routes for public areas (login, registration), user-authenticated areas (profile), and admin-authenticated areas (admin dashboard, user list).
        *   Implement "Protected Routes" components/logic that check authentication status and user role (from `authStore`).
            *   If not authenticated for a protected route, redirect to login.
            *   If authenticated but wrong role (e.g., 'user' trying to access '/admin'), redirect to a 'Forbidden' page or user dashboard.
        *   Separate layouts for user and admin sections if visual distinction is significant.
*   **Forms: Tanstack Form (React Form)**
    *   **Usage:** For handling registration, login, and user profile editing forms.
    *   **Features:** Provides form state management, validation (can integrate with Zod or Yup), and submission handling.
*   **List Virtualization: Tanstack Virtual (React Virtual)**
    *   **Consideration:** Evaluate for the admin user list. If the number of users is expected to be very large (e.g., 1000s), implement virtualization for performance. Otherwise, standard pagination with Tanstack Table might suffice for an MVP. Mark as optional but recommended for scalability.

### 5.2. Backend (Rust - Axum)

*   **Framework: Axum**
    *   **Routing:** Define routes for all API endpoints specified in the API Design section. Use Axum's router with nesting for `/auth`, `/users`, and `/admin` groups.
    *   **Handlers:** Asynchronous functions handling requests and returning responses.
    *   **Extractors:** Utilize Axum extractors for request bodies (JSON), path parameters, query parameters, and headers.
    *   **State:** Manage shared application state (like database connection pool) using Axum's `State` extractor.
*   **Database Interaction: SQLx (with MySQL)**
    *   **Connection Pooling:** Set up an `sqlx::mysql::MySqlPool` and share it across handlers using Axum's `State`. Configure pool size (min/max connections) based on expected load and database limits.
    *   **Compile-time Checked SQL:** Write SQL queries directly in Rust code, validated against the database schema at compile time (via `DATABASE_URL` environment variable during development and `sqlx prepare` in CI/build).
    *   **Migrations:** Use `sqlx-cli` for managing database schema migrations (`sqlx migrate add <name>`, `sqlx migrate run`). Migrations will be written in SQL.
*   **Database: MySQL**
    *   **Version:** Use a recent stable version (e.g., MySQL 8.0+).
    *   **Setup:** Run via Docker Compose for local development.

### 5.3. General
*   **Environment Variables:**
    *   Backend: `DATABASE_URL`, `JWT_SECRET` (or session secret), `PORT`, `RUST_LOG`. Use a `.env` file (gitignored) and provide a `.env.example`.
    *   Frontend: `VITE_API_BASE_URL`. Use `.env.[mode]` files.
*   **Logging:**
    *   Backend: `tracing` or `log` crate with an appropriate subscriber (e.g., `tracing-subscriber` for structured logging).
    *   Frontend: Standard `console.log/error` during development, consider a lightweight logging library for production if error tracking is needed.

## 6. Security Considerations

Security is a critical aspect of this application, especially as it handles user credentials and personal information.

### 6.1. Password Hashing
*   **Algorithm:** Use a strong, adaptive hashing algorithm like **bcrypt** or **Argon2**. Argon2 is generally preferred if available and supported by the Rust ecosystem (e.g., via the `argon2` crate). Bcrypt (via the `bcrypt` crate) is a well-established alternative.
*   **Implementation:** Hashing will occur on the backend during user registration and when updating passwords. Passwords will never be stored in plaintext.
*   **Salt:** Salts will be automatically generated and stored as part of the hash output by these libraries.
*   **Cost Factor:** Configure an appropriate cost factor (work factor) for the chosen algorithm to balance security and performance.

### 6.2. Input Validation
*   **Frontend:** Basic client-side validation (e.g., required fields, email format) for better UX, but not as a security measure. Tanstack Form can be used with libraries like Zod for schema validation.
*   **Backend:** Strict server-side validation for all incoming data (request bodies, query parameters, path parameters).
    *   **Axum Extractors:** Leverage Axum's typed extractors. For complex validation, use a library like `validator` or `serde_valid` in Rust struct definitions that are deserialized from request bodies.
    *   Validate data types, lengths, formats, and ranges.
    *   Ensure usernames and emails are in valid formats.
    *   Sanitize outputs where necessary if they might be re-interpreted as HTML/JS (though React generally handles XSS for data binding).

### 6.3. Secrets Management
*   **Database Credentials:** Store database connection strings and other sensitive credentials (e.g., `JWT_SECRET`) in environment variables.
*   **`.env` Files:** Use `.env` files for local development (added to `.gitignore`).
*   **`.env.example`:** Provide a `.env.example` file in the repository with placeholder values.
*   **Production:** In a production environment, secrets should be injected via the hosting platform's secret management system (e.g., Docker secrets, Kubernetes Secrets, cloud provider KMS).
*   **JWT Secret:** The `JWT_SECRET` (or session signing secret) must be a cryptographically strong random string and kept confidential.

### 6.4. Authentication & Authorization (Role-Based Access Control - RBAC)
*   **Token/Session Management:**
    *   **Tokens (JWT):** If using JWTs, they should be short-lived and include user ID and role in the claims. Transmit JWTs via HttpOnly cookies (for XSS protection) or in the Authorization header (if SPA needs to access it, though HttpOnly is generally safer from XSS). Consider token refresh mechanisms.
    *   **Sessions:** If using server-side sessions, session IDs should be securely generated and stored in HttpOnly, Secure cookies.
*   **Backend Enforcement:** Authorization logic (role checks) MUST be implemented on the backend (Axum middleware or route guards). Client-side checks are for UX only.
    *   Admin endpoints (`/api/v1/admin/*`) must verify the 'admin' role from the validated token/session.
    *   User endpoints (`/api/v1/users/me`) must ensure users can only access/modify their own data.
*   **HTTPS:** Enforce HTTPS for all communication to protect data in transit.

### 6.5. Prevention of Common Web Vulnerabilities
*   **Cross-Site Scripting (XSS):**
    *   **React:** React automatically escapes data rendered in JSX, providing significant protection.
    *   **Shadcn/UI:** Assumed to follow best practices.
    *   **Axum:** Avoid directly embedding user input into HTML responses if ever serving HTML from Axum (though primarily an API). Set `Content-Type: application/json`.
*   **Cross-Site Request Forgery (CSRF):**
    *   **Stateless APIs (JWT in Auth Header):** Generally less susceptible if not using cookies for authentication.
    *   **Cookie-based Auth/Sessions:** Implement CSRF protection (e.g., Axum middleware using `axum_csrf` or similar, generating and validating CSRF tokens). Double-submit cookie pattern or SameSite cookies (`Strict` or `Lax`).
*   **SQL Injection:**
    *   **SQLx:** Using SQLx with parameterized queries (placeholders like `$1`, `$2` or `?`) effectively prevents SQL injection vulnerabilities. Avoid string concatenation to build queries with user input.
*   **Denial of Service (DoS):**
    *   **Rate Limiting:** Implement rate limiting on sensitive endpoints (login, registration) to prevent abuse and brute-force attacks. (e.g., using a middleware like `tower-governor`).
    *   **Input Size Limits:** Configure reasonable request body size limits.
*   **Security Headers:**
    *   Implement security-related HTTP headers like `Strict-Transport-Security`, `X-Content-Type-Options`, `X-Frame-Options`, `Content-Security-Policy` (CSP) where appropriate. Axum middleware can be used to add these.

### 6.6. Dependency Management
*   Regularly update dependencies (both frontend and backend) to patch known vulnerabilities.
*   Use tools like `cargo audit` (Rust) and `npm audit` or `pnpm audit` (Node.js) to check for vulnerabilities in dependencies.

### 6.7. Logging and Monitoring (Basic)
*   Log security-relevant events like failed login attempts, authorization failures, and significant errors.
*   Basic monitoring of application health and error rates. (Advanced audit trails are out of scope).

## 7. Testing Strategy

A comprehensive testing strategy is essential to ensure the reliability, correctness, and security of the application.

### 7.1. General Principles
*   **Test Pyramid:** Focus on a healthy balance: many fast unit tests, fewer integration tests, and a minimal number of end-to-end tests (though full E2E might be out of scope for MVP, integration tests will cover key flows).
*   **Automation:** Tests should be automated and runnable with simple commands (e.g., `pnpm test`, `cargo test`).
*   **CI/CD:** Integrate tests into a CI/CD pipeline to ensure they run on every commit/PR.
*   **Code Coverage:** Aim for reasonable code coverage, particularly for critical logic (auth, business rules).

### 7.2. Frontend Testing (React)

*   **Tools:**
    *   **Test Runner/Framework:** **Vitest** (modern, fast, Jest-compatible API).
    *   **Testing Library:** **React Testing Library (RTL)** (for testing components from a user's perspective).
    *   **Mocking:** Vitest's built-in mocking capabilities. `msw` (Mock Service Worker) can be used to mock API requests at the network level for more realistic integration testing of components.
*   **Unit Tests:**
    *   **Scope:** Individual React components, utility functions, Zustand store logic (reducers, actions).
    *   **Focus:**
        *   Component rendering based on props and state.
        *   User interactions (button clicks, form input) and their effects on component state or callbacks.
        *   Correctness of Zustand store updates and selectors.
        *   Utility function logic.
    *   **Examples:**
        *   Testing if a `Button` component renders its children correctly.
        *   Testing if a form input updates its state on change.
        *   Testing if a Zustand action correctly updates the `isAuthenticated` state.
*   **Integration Tests (Frontend-focused):**
    *   **Scope:** Interactions between multiple components, routing, and components interacting with mocked API services.
    *   **Focus:**
        *   User flows within the frontend (e.g., registration form submission leading to a redirect or state change).
        *   Protected routes correctly redirecting unauthenticated/unauthorized users.
        *   Components fetching and displaying data from mocked API responses (using `msw`).
        *   Tanstack Query hooks behavior with mocked data.
    *   **Examples:**
        *   Testing the login flow: user types credentials, clicks login, `authStore` updates, user is redirected to dashboard.
        *   Testing that the admin user list component correctly fetches and displays mocked user data.

### 7.3. Backend Testing (Rust - Axum)

*   **Tools:**
    *   **Test Runner/Framework:** Standard Rust testing framework (`#[test]` attribute, `cargo test`).
    *   **HTTP Client for Integration Tests:** `reqwest` (for making HTTP requests to Axum handlers) or `axum-test` helpers.
    *   **Mocking/Test Doubles:** `mockall` or manual test doubles for external dependencies if any (though for this MVP, most dependencies are concrete like SQLx). For SQLx, testing against a real test database is preferred.
    *   **Database Testing:** Use a separate test database. SQLx migrations should be run on the test database before tests. Each test could run in a transaction that's rolled back to ensure test isolation.
*   **Unit Tests:**
    *   **Scope:** Individual functions, modules, business logic units, request/response transformation logic.
    *   **Focus:**
        *   Correctness of algorithms (e.g., validation logic).
        *   Transformation of data.
        *   Helper functions.
    *   **Examples:**
        *   Testing a function that validates email formats.
        *   Testing password hashing and verification logic (using a known test vector).
*   **Integration Tests (Backend-focused):**
    *   **Scope:** API endpoints, interaction between handlers, middleware, and the database.
    *   **Focus:**
        *   Full request-response cycle for each API endpoint.
        *   Correctness of database operations (CRUD).
        *   Authentication and authorization middleware logic.
        *   Error handling and response codes.
    *   **Setup:**
        *   Spin up the Axum application in-memory or against a local server.
        *   Prepare a test database (e.g., using Docker, with migrations applied).
        *   Seed data as necessary for specific test cases.
    *   **Examples:**
        *   Testing `POST /api/v1/auth/register`: send valid data, verify user is created in the database, verify 201 response. Send invalid data, verify 400/422 response. Send duplicate username, verify 409 response.
        *   Testing `GET /api/v1/admin/users`: authenticate as admin, verify list of users is returned. Authenticate as user, verify 403 Forbidden.
        *   Testing `PATCH /api/v1/admin/users/{userId}/status`: authenticate as admin, change user status, verify database update and response.

### 7.4. Testing Priorities for MVP
*   **Authentication and Authorization:** Thoroughly test login, registration, logout, token validation, and role-based access control for all relevant API endpoints (both user and admin). This is critical for security.
*   **User Module Core Flows:** Registration, Login, Profile View/Edit.
*   **Admin Module Core Flows:** Admin Login, User Listing, User Detail View, User Activation/Deactivation.
*   **Database Interactions:** Ensure data is correctly stored and retrieved, especially user credentials and roles.

## 8. Development Environment Setup

A consistent and easy-to-set-up development environment is crucial for productivity. We will use Docker and Docker Compose to manage services like MySQL.

### 8.1. Prerequisites
*   **Git:** For version control.
*   **Node.js and pnpm:** For frontend development (Node.js LTS version).
*   **Rust:** For backend development (latest stable version via `rustup`).
*   **Docker and Docker Compose:** For running services like MySQL.
*   **SQLx CLI:** For backend database migrations (`cargo install sqlx-cli`).
*   **IDE/Text Editor:** VS Code with recommended extensions (Rust Analyzer, ESLint, Prettier) or any other preferred IDE.

### 8.2. Docker Compose Setup
A `docker-compose.yml` file will be created at the root of the project to define and manage the MySQL service.

**`docker-compose.yml` (Example):**
```yaml
version: '3.8'

services:
  mysql_db:
    image: mysql:8.0
    container_name: mosip_mvp_mysql
    ports:
      - "3306:3306" # Expose MySQL port to host
    environment:
      MYSQL_ROOT_PASSWORD: rootpassword # For development only
      MYSQL_DATABASE: mosip_mvp_dev
      MYSQL_USER: mosip_user
      MYSQL_PASSWORD: mosip_password
    volumes:
      - mysql_data:/var/lib/mysql # Persist database data
      # Optional: Mount custom MySQL config if needed
      # - ./config/mysql/my.cnf:/etc/mysql/my.cnf
    healthcheck:
      test: ["CMD", "mysqladmin" ,"ping", "-h", "localhost", "-u", "root", "-prootpassword"]
      interval: 10s
      timeout: 5s
      retries: 5

volumes:
  mysql_data:
```

**Key Environment Variables for MySQL in `docker-compose.yml`:**
*   `MYSQL_ROOT_PASSWORD`: Root password for MySQL (use a secure one, even for dev).
*   `MYSQL_DATABASE`: Name of the database to be created automatically.
*   `MYSQL_USER`: Username for the application to connect to the database.
*   `MYSQL_PASSWORD`: Password for the application user.

### 8.3. Backend Setup (Rust/Axum)
1.  **Clone Repository:** `git clone <repository_url>`
2.  **Navigate to Backend Directory:** (Assuming backend is in a `backend` workspace package or root)
3.  **Install Rust:** If not already installed, use `rustup`.
4.  **Configure Environment Variables:**
    *   Copy `.env.example` to `.env` in the backend directory.
    *   Update `.env` with database connection details matching `docker-compose.yml`:
        ```env
        DATABASE_URL="mysql://mosip_user:mosip_password@localhost:3306/mosip_mvp_dev"
        JWT_SECRET="your-strong-jwt-secret" # Generate a strong secret
        RUST_LOG="info,sqlx=warn" # Adjust log levels as needed
        # Other backend specific env vars
        ```
5.  **Start MySQL Container:**
    *   From the project root: `docker-compose up -d mysql_db`
    *   Wait for the database to be healthy (check logs or healthcheck status).
6.  **Run Database Migrations:**
    *   Ensure `sqlx-cli` is installed (`cargo install sqlx-cli --no-default-features --features mysql,rustls`).
    *   From the backend directory (where `sqlx-cli.झzsql` or migrations folder is):
        `sqlx database create` (if not automatically created by Docker entrypoint/SQLx)
        `sqlx migrate run`
7.  **Seed Initial Admin User:**
    *   Migrations should include a step to seed an initial admin user (see Database Schema section). The password for this admin user should be defined (e.g., `ADMIN_USERNAME`, `ADMIN_PASSWORD` in `.env`) and a script or manual step might be needed to hash it for the migration if not hardcoded (hashed) in the migration itself.
    *   *Alternative:* A one-time seeding script or command in the backend application could be run after migrations if preferred over direct SQL seeding for complex setup.
8.  **Build & Run Backend:**
    *   `cargo build`
    *   `cargo run` (The backend server should start, typically on a port like 3000 or 8000)

### 8.4. Frontend Setup (React/pnpm)
1.  **Navigate to Frontend Directory:** (Assuming frontend is in a `frontend` workspace package)
2.  **Install pnpm:** If not already installed (e.g., `npm install -g pnpm`).
3.  **Configure Environment Variables:**
    *   Copy `.env.example` to `.env` (or `.env.development`) in the frontend directory.
    *   Update `.env` with the backend API URL:
        ```env
        VITE_API_BASE_URL="http://localhost:8000/api/v1" # Adjust port if backend runs elsewhere
        ```
4.  **Install Dependencies:**
    *   From the frontend directory (or project root if using pnpm workspaces effectively): `pnpm install`
5.  **Run Frontend Dev Server:**
    *   `pnpm dev` (The React app should start, typically on a port like 5173, and proxy API requests if configured, or directly call the backend URL)

### 8.5. Running Tests
*   **Backend:** `cargo test` (from the backend directory)
*   **Frontend:** `pnpm test` (from the frontend directory)

### 8.6. Initial Admin User Access
*   Once everything is running and the admin user is seeded:
    *   The credentials for the initial admin user (e.g., `admin` / `adminpassword` as defined during seeding) should be documented in the main `README.md`.
    *   Navigate to the application's login page and use these credentials.
    *   Admins should be redirected to the admin dashboard upon successful login.
