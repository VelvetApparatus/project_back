# Backend API Documentation

This documentation provides an overview of the backend API routes for the application. The backend is built using the Actix Web framework in Rust, and it includes various routes for authentication, chat functionality, user profiles, WebSocket connections, and search features.

## Table of Contents
1. [Authentication](#authentication)
   - [User Registration](#user-registration)
   - [User Login](#user-login)
   - [User Logout](#user-logout)
2. [Chat](#chat)
   - [Messages](#messages)
     - [Create Message](#create-message)
     - [Get Messages](#get-messages)
   - [Channels](#channels)
     - [Show Channels](#show-channels)
     - [Create Channel](#create-channel)
     - [Join Channel](#join-channel)
     - [Leave Channel](#leave-channel)
3. [Profile](#profile)
   - [Get User Profile](#get-user-profile)
   - [Update User Profile](#update-user-profile)
4. [WebSocket](#websocket)
   - [Start Connection](#start-connection)
5. [Search](#search)
   - [Search Users](#search-users)
   - [Search Channels](#search-channels)

## Authentication

### User Registration
- **Route:** `/api/v1/auth/reg`
- **Method:** `POST`
- **Handler:** `reg`
- **BodyParams**
  ```rust
  pub struct RegistrationData {
    pub username: String,
    pub email: String,
    pub password: String,
  }
  ```

### User Login
- **Route:** `/api/v1/auth/log_in`
- **Method:** `POST`
- **Handler:** `log_in`
- With 200 Response status sends cookie 
- **BodyParams**
  ```rust
  pub struct AuthorizationData {
    pub email: String,
    pub password: String,
  }
  ```

### User Logout
- **Route:** `/api/v1/auth/log_out`
- **Method:** `GET`
- **Handler:** `log_out` 
- Drops a Cookie

## Chat

### Messages

#### Create Message
- **Route:** `/api/v1/chat/messages/create`
- **Method:** `POST`
- **Handler:** `create_message`
- **BodyParams**
  ```rust
  pub struct SendMessageBody {
    pub reciever: Uuid,
    pub body: String,
  }
  ```

#### Get Messages
- **Route:** `/api/v1/chat/messages/get`
- **Method:** `POST`
- **Handler:** `get_messages`
- Returns messages from start_index to end_index
- **BodyParams**
  ```rust
  pub struct GetMessagesData {
    pub start_index: i32,
    pub end_index: i32,
    pub channel_id: Uuid 
  }
  ```

### Channels

#### Show Channels
- **Route:** `/api/v1/chat/channels/get`
- **Method:** `GET`
- **Handler:** `show_channels`
- Shows channels for current user.
  
#### Create Channel
- **Route:** `/api/v1/chat/channels/create`
- **Method:** `POST`
- **Handler:** `create_channel`
- **BodyParams**
  ```
  pub struct CreateChannelData {
    pub name: String,
    pub users: Vec<Uuid>,
  }
  ```
  where users -- list of users which be invited after creating

#### Join Channel
- **Route:** `/api/v1/chat/channels/join`
- **Method:** `POST`
- **Handler:** `join_channel`
- **BodyParams**
  ```
  pub struct JoinData { 
    pub channel_id: Uuid
  }
  ```

#### Leave Channel
- **Route:** `/api/v1/chat/channels/out`
- **Method:** `POST`
- **Handler:** `out_channel`
- **BodyParams**
  ```
  pub struct OutData {
    pub channel_id: Uuid
  }
  ```

## Profile

### Get User Profile
- **Route:** `/api/v1/profile/get_me`
- **Method:** `GET`
- **Handler:** `get_me`

### Update User Profile
- **Route:** `/api/v1/profile/update`
- **Method:** `POST`
- **Handler:** `update_me`
- **BodyParams**
  ```
  pub struct UpdateMeData {
    background: String,
    icon: String,
    username: String,
  }
  ```

## WebSocket

### Start Connection
- **Route:** `/api/v1/stream`
- **Method:** WebSocket
- **Handler:** `start_connection`
- Creates websocket connection for real-time communications
- **MessageStruct**
  ```
  pub struct ClientActorMessage {
    pub id: Vec<Uuid>,
    pub msg: chat::message::Message,
  }
  ```
  Sends msg to all users in 'id' vector

## Search

### Search Users
- **Route:** `/api/v1/search/users`
- **Method:** `GET`
- **Handler:** `search_users`
- **QueryParams** search_text
- Returns all users wich has a substring 'search_text'

### Search Channels
- **Route:** `/api/v1/search/channels`
- **Method:** `GET`
- **Handler:** `search_channels`
- - **QueryParams** search_text
- Returns all channels wich has a substring 'search_text'

