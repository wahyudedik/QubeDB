# 🚀 QubeDB Framework Integration Guide

## Modern Framework Integration dengan QubeDB

Panduan lengkap untuk mengintegrasikan QubeDB dengan framework modern populer.

## 📋 Table of Contents

1. [Laravel (PHP)](#laravel-php)
2. [Django (Python)](#django-python)
3. [Spring Boot (Java)](#spring-boot-java)
4. [Next.js (React)](#nextjs-react)
5. [NestJS (Node.js)](#nestjs-nodejs)
6. [Go (Gin/Echo)](#go-ginecho)
7. [Rust (Actix/Axum)](#rust-actixaxum)
8. [Flutter (Dart)](#flutter-dart)
9. [Vue.js (Nuxt)](#vuejs-nuxt)
10. [Angular](#angular)

---

## 🐘 Laravel (PHP)

### Installation

```bash
composer require qubedb/laravel-driver
```

### Configuration

```php
// config/database.php
'connections' => [
    'qubedb' => [
        'driver' => 'qubedb',
        'host' => env('QUBEDB_HOST', 'localhost'),
        'port' => env('QUBEDB_PORT', 8080),
        'database' => env('QUBEDB_DATABASE', 'qubedb'),
        'username' => env('QUBEDB_USERNAME', 'admin'),
        'password' => env('QUBEDB_PASSWORD', ''),
        'ssl' => env('QUBEDB_SSL', false),
    ],
],
```

### Usage

```php
// Model
class User extends Model
{
    protected $connection = 'qubedb';
    protected $table = 'users';
    
    protected $casts = [
        'metadata' => 'array', // JSON support
        'embedding' => 'array', // Vector support
    ];
}

// Controller
class UserController extends Controller
{
    public function index()
    {
        // Relational query
        $users = User::where('age', '>', 18)->get();
        
        // Vector search
        $similar = User::vectorSearch('embedding', [0.1, 0.2, 0.3], 5)->get();
        
        // Graph query
        $friends = User::graph()->neighbors('friends')->get();
        
        return response()->json($users);
    }
    
    public function store(Request $request)
    {
        $user = User::create([
            'name' => $request->name,
            'age' => $request->age,
            'metadata' => $request->metadata, // JSON
            'embedding' => $request->embedding, // Vector
        ]);
        
        return response()->json($user);
    }
}
```

### Advanced Features

```php
// Multi-model operations
class ProductController extends Controller
{
    public function search(Request $request)
    {
        $query = $request->get('q');
        
        // Text search
        $textResults = Product::where('name', 'like', "%{$query}%")->get();
        
        // Vector similarity search
        $vector = $this->getEmbedding($query);
        $vectorResults = Product::vectorSearch('embedding', $vector, 10)->get();
        
        // Graph recommendations
        $recommendations = Product::graph()
            ->neighbors('similar_to')
            ->where('category', $request->category)
            ->get();
        
        return response()->json([
            'text' => $textResults,
            'vector' => $vectorResults,
            'recommendations' => $recommendations,
        ]);
    }
}
```

---

## 🐍 Django (Python)

### Installation

```bash
pip install qubedb-django
```

### Configuration

```python
# settings.py
DATABASES = {
    'default': {
        'ENGINE': 'qubedb_django.backend',
        'NAME': 'qubedb',
        'HOST': 'localhost',
        'PORT': 8080,
        'USER': 'admin',
        'PASSWORD': '',
        'OPTIONS': {
            'ssl': False,
        }
    }
}
```

### Models

```python
from django.db import models
from qubedb_django.fields import VectorField, JSONField

class User(models.Model):
    name = models.CharField(max_length=100)
    age = models.IntegerField()
    metadata = JSONField(default=dict)  # JSON support
    embedding = VectorField(dimension=512)  # Vector support
    
    class Meta:
        db_table = 'users'

class Product(models.Model):
    name = models.CharField(max_length=200)
    category = models.CharField(max_length=50)
    specs = JSONField(default=dict)
    embedding = VectorField(dimension=384)
    
    class Meta:
        db_table = 'products'
```

### Views

```python
from django.http import JsonResponse
from django.views import View
from .models import User, Product
from qubedb_django.queries import vector_search, graph_query

class UserListView(View):
    def get(self, request):
        # Relational query
        users = User.objects.filter(age__gt=18)
        
        # Vector search
        query_vector = [0.1, 0.2, 0.3, 0.4, 0.5]
        similar_users = vector_search(User, 'embedding', query_vector, k=5)
        
        # Graph query
        friends = graph_query(User, 'friends', 'alice')
        
        return JsonResponse({
            'users': list(users.values()),
            'similar': list(similar_users.values()),
            'friends': list(friends.values()),
        })

class ProductSearchView(View):
    def post(self, request):
        query = request.json.get('query')
        
        # Multi-model search
        results = {
            'text': Product.objects.filter(name__icontains=query),
            'vector': vector_search(Product, 'embedding', query, k=10),
            'graph': graph_query(Product, 'similar_to', query),
        }
        
        return JsonResponse(results)
```

---

## ☕ Spring Boot (Java)

### Dependencies

```xml
<dependency>
    <groupId>com.qubedb</groupId>
    <artifactId>qubedb-spring-boot-starter</artifactId>
    <version>0.1.0</version>
</dependency>
```

### Configuration

```yaml
# application.yml
spring:
  datasource:
    driver-class-name: com.qubedb.jdbc.Driver
    url: jdbc:qubedb://localhost:8080/qubedb
    username: admin
    password: 
  qubedb:
    ssl: false
    timeout: 30
```

### Entity

```java
@Entity
@Table(name = "users")
public class User {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;
    
    private String name;
    private Integer age;
    
    @Column(columnDefinition = "JSON")
    private String metadata; // JSON support
    
    @Column(columnDefinition = "VECTOR")
    private float[] embedding; // Vector support
    
    // Getters and setters
}
```

### Repository

```java
@Repository
public interface UserRepository extends JpaRepository<User, Long> {
    
    // Relational queries
    List<User> findByAgeGreaterThan(Integer age);
    
    // Vector search
    @Query("SELECT * FROM users WHERE vector_search(embedding, ?1) < 0.5")
    List<User> findSimilarUsers(float[] queryVector);
    
    // Graph queries
    @Query("SELECT * FROM users WHERE graph_neighbors('friends', id)")
    List<User> findFriends(Long userId);
}
```

### Controller

```java
@RestController
@RequestMapping("/api/users")
public class UserController {
    
    @Autowired
    private UserRepository userRepository;
    
    @GetMapping
    public ResponseEntity<List<User>> getUsers() {
        List<User> users = userRepository.findByAgeGreaterThan(18);
        return ResponseEntity.ok(users);
    }
    
    @PostMapping("/search")
    public ResponseEntity<List<User>> searchUsers(@RequestBody SearchRequest request) {
        // Vector similarity search
        List<User> similar = userRepository.findSimilarUsers(request.getVector());
        
        // Graph recommendations
        List<User> recommendations = userRepository.findFriends(request.getUserId());
        
        return ResponseEntity.ok(similar);
    }
}
```

---

## ⚛️ Next.js (React)

### Installation

```bash
npm install qubedb-js
```

### Configuration

```javascript
// lib/qubedb.js
import { QubeDB } from 'qubedb-js';

const qubedb = new QubeDB({
  host: process.env.QUBEDB_HOST || 'localhost',
  port: process.env.QUBEDB_PORT || 8080,
  database: process.env.QUBEDB_DATABASE || 'qubedb',
  username: process.env.QUBEDB_USERNAME || 'admin',
  password: process.env.QUBEDB_PASSWORD || '',
});

export default qubedb;
```

### API Routes

```javascript
// pages/api/users.js
import qubedb from '../../lib/qubedb';

export default async function handler(req, res) {
  if (req.method === 'GET') {
    try {
      // Relational query
      const users = await qubedb.query('SELECT * FROM users WHERE age > ?', [18]);
      
      res.status(200).json(users.rows);
    } catch (error) {
      res.status(500).json({ error: error.message });
    }
  }
  
  if (req.method === 'POST') {
    try {
      const { name, age, metadata, embedding } = req.body;
      
      // Insert user
      const result = await qubedb.query(
        'INSERT INTO users (name, age, metadata, embedding) VALUES (?, ?, ?, ?)',
        [name, age, JSON.stringify(metadata), JSON.stringify(embedding)]
      );
      
      res.status(201).json({ id: result.insertId });
    } catch (error) {
      res.status(500).json({ error: error.message });
    }
  }
}
```

### Components

```jsx
// components/UserList.jsx
import { useState, useEffect } from 'react';

export default function UserList() {
  const [users, setUsers] = useState([]);
  const [loading, setLoading] = useState(true);
  
  useEffect(() => {
    fetchUsers();
  }, []);
  
  const fetchUsers = async () => {
    try {
      const response = await fetch('/api/users');
      const data = await response.json();
      setUsers(data);
    } catch (error) {
      console.error('Error fetching users:', error);
    } finally {
      setLoading(false);
    }
  };
  
  const searchSimilar = async (userId) => {
    try {
      const response = await fetch(`/api/users/${userId}/similar`);
      const similar = await response.json();
      setUsers(similar);
    } catch (error) {
      console.error('Error searching similar users:', error);
    }
  };
  
  if (loading) return <div>Loading...</div>;
  
  return (
    <div>
      <h2>Users</h2>
      {users.map(user => (
        <div key={user.id} className="user-card">
          <h3>{user.name}</h3>
          <p>Age: {user.age}</p>
          <button onClick={() => searchSimilar(user.id)}>
            Find Similar
          </button>
        </div>
      ))}
    </div>
  );
}
```

---

## 🚀 NestJS (Node.js)

### Installation

```bash
npm install @qubedb/nestjs
```

### Module

```typescript
// app.module.ts
import { Module } from '@nestjs/common';
import { QubeDBModule } from '@qubedb/nestjs';

@Module({
  imports: [
    QubeDBModule.forRoot({
      host: process.env.QUBEDB_HOST || 'localhost',
      port: parseInt(process.env.QUBEDB_PORT) || 8080,
      database: process.env.QUBEDB_DATABASE || 'qubedb',
      username: process.env.QUBEDB_USERNAME || 'admin',
      password: process.env.QUBEDB_PASSWORD || '',
    }),
  ],
})
export class AppModule {}
```

### Service

```typescript
// users.service.ts
import { Injectable } from '@nestjs/common';
import { QubeDBService } from '@qubedb/nestjs';

@Injectable()
export class UsersService {
  constructor(private readonly qubedb: QubeDBService) {}
  
  async findAll(): Promise<any[]> {
    const result = await this.qubedb.query('SELECT * FROM users WHERE age > ?', [18]);
    return result.rows;
  }
  
  async create(userData: any): Promise<any> {
    const { name, age, metadata, embedding } = userData;
    
    const result = await this.qubedb.query(
      'INSERT INTO users (name, age, metadata, embedding) VALUES (?, ?, ?, ?)',
      [name, age, JSON.stringify(metadata), JSON.stringify(embedding)]
    );
    
    return { id: result.insertId };
  }
  
  async findSimilar(userId: number, vector: number[]): Promise<any[]> {
    const result = await this.qubedb.query(
      'SELECT * FROM users WHERE vector_search(embedding, ?) < 0.5 AND id != ?',
      [JSON.stringify(vector), userId]
    );
    
    return result.rows;
  }
}
```

### Controller

```typescript
// users.controller.ts
import { Controller, Get, Post, Body, Param } from '@nestjs/common';
import { UsersService } from './users.service';

@Controller('users')
export class UsersController {
  constructor(private readonly usersService: UsersService) {}
  
  @Get()
  async findAll() {
    return this.usersService.findAll();
  }
  
  @Post()
  async create(@Body() userData: any) {
    return this.usersService.create(userData);
  }
  
  @Post(':id/similar')
  async findSimilar(@Param('id') id: string, @Body() body: { vector: number[] }) {
    return this.usersService.findSimilar(parseInt(id), body.vector);
  }
}
```

---

## 🐹 Go (Gin/Echo)

### Installation

```bash
go get github.com/qubedb/go-driver
```

### Gin Framework

```go
// main.go
package main

import (
    "github.com/gin-gonic/gin"
    "github.com/qubedb/go-driver"
    "database/sql"
    _ "github.com/qubedb/go-driver"
)

func main() {
    // Connect to QubeDB
    db, err := sql.Open("qubedb", "qubedb://localhost:8080/qubedb")
    if err != nil {
        panic(err)
    }
    defer db.Close()
    
    r := gin.Default()
    
    r.GET("/users", func(c *gin.Context) {
        rows, err := db.Query("SELECT * FROM users WHERE age > ?", 18)
        if err != nil {
            c.JSON(500, gin.H{"error": err.Error()})
            return
        }
        defer rows.Close()
        
        var users []map[string]interface{}
        for rows.Next() {
            var id int
            var name string
            var age int
            err := rows.Scan(&id, &name, &age)
            if err != nil {
                continue
            }
            users = append(users, map[string]interface{}{
                "id": id, "name": name, "age": age,
            })
        }
        
        c.JSON(200, users)
    })
    
    r.POST("/users", func(c *gin.Context) {
        var user struct {
            Name string `json:"name"`
            Age  int    `json:"age"`
        }
        
        if err := c.ShouldBindJSON(&user); err != nil {
            c.JSON(400, gin.H{"error": err.Error()})
            return
        }
        
        result, err := db.Exec("INSERT INTO users (name, age) VALUES (?, ?)", user.Name, user.Age)
        if err != nil {
            c.JSON(500, gin.H{"error": err.Error()})
            return
        }
        
        id, _ := result.LastInsertId()
        c.JSON(201, gin.H{"id": id})
    })
    
    r.Run(":3000")
}
```

### Echo Framework

```go
// main.go
package main

import (
    "github.com/labstack/echo/v4"
    "github.com/qubedb/go-driver"
    "database/sql"
    _ "github.com/qubedb/go-driver"
)

func main() {
    db, err := sql.Open("qubedb", "qubedb://localhost:8080/qubedb")
    if err != nil {
        panic(err)
    }
    defer db.Close()
    
    e := echo.New()
    
    e.GET("/users", func(c echo.Context) error {
        rows, err := db.Query("SELECT * FROM users WHERE age > ?", 18)
        if err != nil {
            return c.JSON(500, map[string]string{"error": err.Error()})
        }
        defer rows.Close()
        
        var users []map[string]interface{}
        for rows.Next() {
            var id int
            var name string
            var age int
            err := rows.Scan(&id, &name, &age)
            if err != nil {
                continue
            }
            users = append(users, map[string]interface{}{
                "id": id, "name": name, "age": age,
            })
        }
        
        return c.JSON(200, users)
    })
    
    e.Logger.Fatal(e.Start(":3000"))
}
```

---

## 🦀 Rust (Actix/Axum)

### Dependencies

```toml
[dependencies]
qubedb-core = "0.1.0"
actix-web = "4.0"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Actix Web

```rust
// main.rs
use actix_web::{web, App, HttpServer, Result};
use qubedb_core::embedded::EmbeddedQubeDB;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    age: i32,
}

struct AppState {
    db: Mutex<EmbeddedQubeDB>,
}

async fn get_users(data: web::Data<AppState>) -> Result<web::Json<Vec<User>>> {
    let db = data.db.lock().unwrap();
    let result = db.execute("SELECT * FROM users WHERE age > 18").await?;
    
    let users: Vec<User> = result.rows.into_iter().map(|row| {
        User {
            id: row.get("id").and_then(|v| v.as_i32()),
            name: row.get("name").and_then(|v| v.as_string()).unwrap_or_default(),
            age: row.get("age").and_then(|v| v.as_i32()).unwrap_or(0),
        }
    }).collect();
    
    Ok(web::Json(users))
}

async fn create_user(
    data: web::Data<AppState>,
    user: web::Json<User>
) -> Result<web::Json<serde_json::Value>> {
    let mut db = data.db.lock().unwrap();
    
    let result = db.execute(
        "INSERT INTO users (name, age) VALUES (?, ?)",
        [&user.name, &user.age.to_string()]
    ).await?;
    
    Ok(web::Json(serde_json::json!({
        "id": result.insert_id,
        "message": "User created successfully"
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = EmbeddedQubeDB::open("./qubedb_data").unwrap();
    let app_state = web::Data::new(AppState {
        db: Mutex::new(db),
    });
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(create_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### Axum

```rust
// main.rs
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use qubedb_core::embedded::EmbeddedQubeDB;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    age: i32,
}

struct AppState {
    db: Arc<Mutex<EmbeddedQubeDB>>,
}

async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    let db = state.db.lock().await;
    let result = db.execute("SELECT * FROM users WHERE age > 18").await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let users: Vec<User> = result.rows.into_iter().map(|row| {
        User {
            id: row.get("id").and_then(|v| v.as_i32()),
            name: row.get("name").and_then(|v| v.as_string()).unwrap_or_default(),
            age: row.get("age").and_then(|v| v.as_i32()).unwrap_or(0),
        }
    }).collect();
    
    Ok(Json(users))
}

async fn create_user(
    State(state): State<AppState>,
    Json(user): Json<User>
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut db = state.db.lock().await;
    
    let result = db.execute(
        "INSERT INTO users (name, age) VALUES (?, ?)",
        [&user.name, &user.age.to_string()]
    ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(serde_json::json!({
        "id": result.insert_id,
        "message": "User created successfully"
    })))
}

#[tokio::main]
async fn main() {
    let db = EmbeddedQubeDB::open("./qubedb_data").unwrap();
    let app_state = AppState {
        db: Arc::new(Mutex::new(db)),
    };
    
    let app = Router::new()
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .with_state(app_state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

---

## 🎯 Flutter (Dart)

### Dependencies

```yaml
# pubspec.yaml
dependencies:
  qubedb_dart: ^0.1.0
  http: ^0.13.0
```

### Service

```dart
// lib/services/qubedb_service.dart
import 'package:qubedb_dart/qubedb_dart.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';

class QubeDBService {
  final String baseUrl;
  
  QubeDBService({required this.baseUrl});
  
  Future<List<Map<String, dynamic>>> getUsers() async {
    final response = await http.get(Uri.parse('$baseUrl/api/users'));
    
    if (response.statusCode == 200) {
      return List<Map<String, dynamic>>.from(json.decode(response.body));
    } else {
      throw Exception('Failed to load users');
    }
  }
  
  Future<Map<String, dynamic>> createUser(Map<String, dynamic> user) async {
    final response = await http.post(
      Uri.parse('$baseUrl/api/users'),
      headers: {'Content-Type': 'application/json'},
      body: json.encode(user),
    );
    
    if (response.statusCode == 201) {
      return json.decode(response.body);
    } else {
      throw Exception('Failed to create user');
    }
  }
  
  Future<List<Map<String, dynamic>>> findSimilarUsers(
    int userId, 
    List<double> vector
  ) async {
    final response = await http.post(
      Uri.parse('$baseUrl/api/users/$userId/similar'),
      headers: {'Content-Type': 'application/json'},
      body: json.encode({'vector': vector}),
    );
    
    if (response.statusCode == 200) {
      return List<Map<String, dynamic>>.from(json.decode(response.body));
    } else {
      throw Exception('Failed to find similar users');
    }
  }
}
```

### Widget

```dart
// lib/widgets/user_list.dart
import 'package:flutter/material.dart';
import '../services/qubedb_service.dart';

class UserList extends StatefulWidget {
  @override
  _UserListState createState() => _UserListState();
}

class _UserListState extends State<UserList> {
  final QubeDBService _service = QubeDBService(baseUrl: 'http://localhost:3000');
  List<Map<String, dynamic>> _users = [];
  bool _loading = true;
  
  @override
  void initState() {
    super.initState();
    _loadUsers();
  }
  
  Future<void> _loadUsers() async {
    try {
      final users = await _service.getUsers();
      setState(() {
        _users = users;
        _loading = false;
      });
    } catch (e) {
      setState(() {
        _loading = false;
      });
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Error loading users: $e')),
      );
    }
  }
  
  @override
  Widget build(BuildContext context) {
    if (_loading) {
      return Center(child: CircularProgressIndicator());
    }
    
    return ListView.builder(
      itemCount: _users.length,
      itemBuilder: (context, index) {
        final user = _users[index];
        return ListTile(
          title: Text(user['name'] ?? ''),
          subtitle: Text('Age: ${user['age'] ?? 0}'),
          trailing: IconButton(
            icon: Icon(Icons.search),
            onPressed: () => _findSimilar(user['id']),
          ),
        );
      },
    );
  }
  
  Future<void> _findSimilar(int userId) async {
    try {
      final similar = await _service.findSimilarUsers(userId, [0.1, 0.2, 0.3]);
      // Show similar users dialog
      showDialog(
        context: context,
        builder: (context) => AlertDialog(
          title: Text('Similar Users'),
          content: Column(
            mainAxisSize: MainAxisSize.min,
            children: similar.map((user) => ListTile(
              title: Text(user['name']),
              subtitle: Text('Age: ${user['age']}'),
            )).toList(),
          ),
        ),
      );
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Error finding similar users: $e')),
      );
    }
  }
}
```

---

## 🟢 Vue.js (Nuxt)

### Installation

```bash
npm install @qubedb/nuxt
```

### Configuration

```javascript
// nuxt.config.js
export default {
  modules: [
    '@qubedb/nuxt'
  ],
  qubedb: {
    host: process.env.QUBEDB_HOST || 'localhost',
    port: process.env.QUBEDB_PORT || 8080,
    database: process.env.QUBEDB_DATABASE || 'qubedb',
  }
}
```

### Plugin

```javascript
// plugins/qubedb.client.js
import { QubeDB } from '@qubedb/nuxt'

export default function ({ $config }, inject) {
  const qubedb = new QubeDB({
    host: $config.qubedb.host,
    port: $config.qubedb.port,
    database: $config.qubedb.database,
  })
  
  inject('qubedb', qubedb)
}
```

### Component

```vue
<!-- components/UserList.vue -->
<template>
  <div>
    <h2>Users</h2>
    <div v-if="loading">Loading...</div>
    <div v-else>
      <div v-for="user in users" :key="user.id" class="user-card">
        <h3>{{ user.name }}</h3>
        <p>Age: {{ user.age }}</p>
        <button @click="findSimilar(user.id)">Find Similar</button>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      users: [],
      loading: true
    }
  },
  async mounted() {
    await this.loadUsers()
  },
  methods: {
    async loadUsers() {
      try {
        const result = await this.$qubedb.query('SELECT * FROM users WHERE age > ?', [18])
        this.users = result.rows
      } catch (error) {
        console.error('Error loading users:', error)
      } finally {
        this.loading = false
      }
    },
    async findSimilar(userId) {
      try {
        const result = await this.$qubedb.query(
          'SELECT * FROM users WHERE vector_search(embedding, ?) < 0.5 AND id != ?',
          [JSON.stringify([0.1, 0.2, 0.3]), userId]
        )
        console.log('Similar users:', result.rows)
      } catch (error) {
        console.error('Error finding similar users:', error)
      }
    }
  }
}
</script>
```

---

## 🔺 Angular

### Installation

```bash
npm install @qubedb/angular
```

### Service

```typescript
// src/app/services/qubedb.service.ts
import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class QubeDBService {
  private baseUrl = 'http://localhost:3000/api';
  
  constructor(private http: HttpClient) {}
  
  getUsers(): Observable<any[]> {
    return this.http.get<any[]>(`${this.baseUrl}/users`);
  }
  
  createUser(user: any): Observable<any> {
    return this.http.post<any>(`${this.baseUrl}/users`, user);
  }
  
  findSimilarUsers(userId: number, vector: number[]): Observable<any[]> {
    return this.http.post<any[]>(`${this.baseUrl}/users/${userId}/similar`, {
      vector: vector
    });
  }
}
```

### Component

```typescript
// src/app/components/user-list/user-list.component.ts
import { Component, OnInit } from '@angular/core';
import { QubeDBService } from '../../services/qubedb.service';

@Component({
  selector: 'app-user-list',
  templateUrl: './user-list.component.html',
  styleUrls: ['./user-list.component.css']
})
export class UserListComponent implements OnInit {
  users: any[] = [];
  loading = true;
  
  constructor(private qubedbService: QubeDBService) {}
  
  ngOnInit() {
    this.loadUsers();
  }
  
  loadUsers() {
    this.qubedbService.getUsers().subscribe({
      next: (users) => {
        this.users = users;
        this.loading = false;
      },
      error: (error) => {
        console.error('Error loading users:', error);
        this.loading = false;
      }
    });
  }
  
  findSimilar(userId: number) {
    const vector = [0.1, 0.2, 0.3, 0.4, 0.5];
    this.qubedbService.findSimilarUsers(userId, vector).subscribe({
      next: (similar) => {
        console.log('Similar users:', similar);
        // Handle similar users
      },
      error: (error) => {
        console.error('Error finding similar users:', error);
      }
    });
  }
}
```

### Template

```html
<!-- src/app/components/user-list/user-list.component.html -->
<div>
  <h2>Users</h2>
  <div *ngIf="loading">Loading...</div>
  <div *ngIf="!loading">
    <div *ngFor="let user of users" class="user-card">
      <h3>{{ user.name }}</h3>
      <p>Age: {{ user.age }}</p>
      <button (click)="findSimilar(user.id)">Find Similar</button>
    </div>
  </div>
</div>
```

---

## 🚀 Best Practices

### 1. Connection Management

```javascript
// Always use connection pooling
const pool = new QubeDBPool({
  host: 'localhost',
  port: 8080,
  database: 'qubedb',
  min: 2,
  max: 10
});
```

### 2. Error Handling

```javascript
try {
  const result = await qubedb.query('SELECT * FROM users');
  return result.rows;
} catch (error) {
  if (error.code === 'CONNECTION_ERROR') {
    // Handle connection errors
  } else if (error.code === 'QUERY_ERROR') {
    // Handle query errors
  } else {
    // Handle other errors
  }
}
```

### 3. Performance Optimization

```javascript
// Use prepared statements
const stmt = await qubedb.prepare('SELECT * FROM users WHERE age > ?');
const result = await stmt.execute([18]);

// Use batch operations
await qubedb.batch([
  'INSERT INTO users (name, age) VALUES (?, ?)',
  'INSERT INTO users (name, age) VALUES (?, ?)'
], [
  ['Alice', 25],
  ['Bob', 30]
]);
```

### 4. Security

```javascript
// Always use parameterized queries
const result = await qubedb.query(
  'SELECT * FROM users WHERE name = ?',
  [userInput] // Never concatenate user input
);

// Use SSL in production
const qubedb = new QubeDB({
  host: 'localhost',
  port: 8080,
  ssl: true,
  sslCert: '/path/to/cert.pem'
});
```

---

## 📞 Support

- 📧 Email: support@qubedb.com
- 💬 Discord: [Join our community](https://discord.gg/qubedb)
- 📖 Documentation: [docs.qubedb.com](https://docs.qubedb.com)
- 🐛 Issues: [GitHub Issues](https://github.com/qubedb/qubedb/issues)

---

**QubeDB** - The future of databases is here! 🚀
