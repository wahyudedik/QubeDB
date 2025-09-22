# 🖥️ QubeDB GUI Management Features

Dokumentasi lengkap fitur-fitur management GUI QubeDB yang memudahkan pengelolaan database.

## 📋 Table of Contents

1. [Dashboard](#dashboard)
2. [Server Settings](#server-settings)
3. [Database Management](#database-management)
4. [Table Management](#table-management)
5. [User Management](#user-management)
6. [Import/Export](#importexport)
7. [Backup/Restore](#backuprestore)
8. [Performance Monitoring](#performance-monitoring)
9. [Framework Integration](#framework-integration)
10. [Security & Permissions](#security--permissions)

---

## 🎯 Dashboard

### Overview
Dashboard memberikan overview lengkap tentang status server, database, dan performa.

### Features
- **Server Status**: Online/Offline, Uptime, Version, Port
- **Database Statistics**: Jumlah database, tabel, records, ukuran
- **Performance Metrics**: Queries/sec, Response time, Memory usage, CPU usage
- **Recent Activity**: Log aktivitas terbaru dengan timestamp
- **Quick Actions**: Shortcut untuk aksi-aksi umum

### Screenshot
```
┌─────────────────────────────────────────────────────────┐
│ Dashboard                                               │
├─────────────────────────────────────────────────────────┤
│ Server Status    │ Database Stats    │ Performance     │
│ • Online         │ • 5 Databases     │ • 125 qps       │
│ • 2 days uptime  │ • 23 Tables       │ • 2.3ms avg     │
│ • Port 8080      │ • 1,234 Records   │ • 156 MB RAM    │
│ • v1.0.0         │ • 45.2 MB Size    │ • 12% CPU       │
└─────────────────────────────────────────────────────────┘
```

---

## ⚙️ Server Settings

### Network Settings
- **Host/IP**: Konfigurasi alamat server (default: localhost)
- **Port**: Port server (default: 8080)
- **Max Connections**: Batas koneksi maksimal (default: 100)
- **SSL/TLS**: Enable/disable enkripsi koneksi

### Authentication
- **Admin Username**: Username administrator
- **Admin Password**: Password administrator
- **Session Timeout**: Waktu timeout session (menit)
- **Require Authentication**: Wajib login untuk akses

### Performance
- **Memory Limit**: Batas penggunaan memori (MB)
- **Query Timeout**: Timeout eksekusi query (detik)
- **Cache Size**: Ukuran cache (MB)
- **Data Compression**: Enable/disable kompresi data

### Logging
- **Log Level**: Error, Warning, Info, Debug
- **Log File Path**: Lokasi file log
- **Max Log Size**: Ukuran maksimal log file (MB)
- **Audit Logging**: Enable/disable audit log

---

## 🗄️ Database Management

### Create Database
```sql
CREATE DATABASE my_database
CHARACTER SET utf8mb4
COLLATION utf8mb4_unicode_ci;
```

### Features
- **Database Wizard**: Step-by-step database creation
- **Character Set**: UTF-8, Latin1, dll
- **Collation**: Unicode, General, dll
- **Database Browser**: Tree view dengan context menu
- **Database Stats**: Size, tables, records, last modified
- **Quick Actions**: Connect, Backup, Delete

### Database Operations
- **Connect**: Koneksi ke database
- **Backup**: Buat backup database
- **Restore**: Restore dari backup
- **Delete**: Hapus database (dengan konfirmasi)
- **Rename**: Ubah nama database
- **Clone**: Duplikasi database

---

## 📊 Table Management

### Table Designer
- **Visual Designer**: Drag-and-drop interface
- **Column Types**: String, Integer, Float, Boolean, Date, JSON, Vector
- **Constraints**: Primary Key, Foreign Key, Unique, Not Null
- **Indexes**: B-Tree, Hash, Vector indexes
- **Relationships**: One-to-One, One-to-Many, Many-to-Many

### Table Operations
- **Create Table**: Visual table creation
- **Alter Table**: Modify table structure
- **Drop Table**: Delete table (dengan konfirmasi)
- **Truncate Table**: Clear all data
- **Table Stats**: Row count, size, indexes

### Column Management
- **Add Column**: Tambah kolom baru
- **Modify Column**: Ubah tipe/constraint kolom
- **Drop Column**: Hapus kolom
- **Reorder Columns**: Urutkan kolom

---

## 👥 User Management

### User Roles
- **Administrator**: Full access
- **Developer**: Read/Write access
- **Analyst**: Read-only access
- **Guest**: Limited access

### User Operations
- **Create User**: Tambah user baru
- **Edit User**: Ubah informasi user
- **Delete User**: Hapus user
- **Reset Password**: Reset password user
- **Assign Roles**: Assign role ke user

### Permissions
- **Database Level**: Access ke database tertentu
- **Table Level**: Access ke tabel tertentu
- **Column Level**: Access ke kolom tertentu
- **Operation Level**: SELECT, INSERT, UPDATE, DELETE

---

## 📥 Import/Export

### Supported Formats
- **CSV**: Comma-separated values
- **JSON**: JavaScript Object Notation
- **SQL**: SQL dump files
- **Excel**: Microsoft Excel files
- **XML**: Extensible Markup Language

### Import Features
- **File Upload**: Upload file dari komputer
- **URL Import**: Import dari URL
- **Database Import**: Import dari database lain
- **Mapping**: Map kolom source ke target
- **Validation**: Validasi data sebelum import
- **Preview**: Preview data sebelum import

### Export Features
- **Query Results**: Export hasil query
- **Table Data**: Export data tabel
- **Database Schema**: Export struktur database
- **Full Backup**: Export seluruh database
- **Custom Query**: Export hasil query custom

---

## 💾 Backup/Restore

### Backup Types
- **Full Backup**: Backup seluruh database
- **Incremental Backup**: Backup perubahan saja
- **Schema Backup**: Backup struktur saja
- **Data Backup**: Backup data saja

### Backup Scheduling
- **Manual Backup**: Backup manual
- **Scheduled Backup**: Backup otomatis
- **Retention Policy**: Atur retensi backup
- **Compression**: Kompresi backup file

### Restore Options
- **Point-in-time**: Restore ke waktu tertentu
- **Selective Restore**: Restore database/table tertentu
- **Full Restore**: Restore seluruh database
- **Validation**: Validasi backup sebelum restore

---

## 📈 Performance Monitoring

### Real-time Metrics
- **Queries per Second**: Jumlah query per detik
- **Average Response Time**: Rata-rata waktu response
- **Memory Usage**: Penggunaan memori
- **CPU Usage**: Penggunaan CPU
- **Disk I/O**: Input/Output disk
- **Network I/O**: Input/Output network

### Historical Data
- **Performance Trends**: Trend performa over time
- **Peak Usage**: Penggunaan puncak
- **Slow Queries**: Query yang lambat
- **Resource Usage**: Penggunaan resource

### Alerts
- **Performance Alerts**: Alert saat performa turun
- **Resource Alerts**: Alert saat resource habis
- **Error Alerts**: Alert saat ada error
- **Custom Alerts**: Alert custom

---

## 🔌 Framework Integration

### Laravel Integration
```php
// .env
DB_CONNECTION=qubedb
DB_HOST=localhost
DB_PORT=8080
DB_DATABASE=my_database
DB_USERNAME=admin
DB_PASSWORD=password
DB_SSL=false

// config/database.php
'connections' => [
    'qubedb' => [
        'driver' => 'qubedb',
        'host' => env('DB_HOST', 'localhost'),
        'port' => env('DB_PORT', 8080),
        'database' => env('DB_DATABASE', 'qubedb'),
        'username' => env('DB_USERNAME', 'admin'),
        'password' => env('DB_PASSWORD', ''),
        'ssl' => env('DB_SSL', false),
    ],
],
```

### Django Integration
```python
# settings.py
DATABASES = {
    'default': {
        'ENGINE': 'qubedb_django.backend',
        'NAME': 'my_database',
        'HOST': 'localhost',
        'PORT': 8080,
        'USER': 'admin',
        'PASSWORD': 'password',
        'OPTIONS': {
            'ssl': False,
        }
    }
}
```

### Spring Boot Integration
```yaml
# application.yml
spring:
  datasource:
    driver-class-name: com.qubedb.jdbc.Driver
    url: jdbc:qubedb://localhost:8080/my_database
    username: admin
    password: password
  qubedb:
    ssl: false
    timeout: 30
```

### Node.js Integration
```javascript
// database.js
const { QubeDB } = require('qubedb-js');

const db = new QubeDB({
  host: 'localhost',
  port: 8080,
  database: 'my_database',
  username: 'admin',
  password: 'password'
});
```

---

## 🔐 Security & Permissions

### Authentication
- **JWT Tokens**: JSON Web Token authentication
- **Session Management**: Session timeout dan management
- **Password Policy**: Aturan password yang kuat
- **Two-Factor Authentication**: 2FA support

### Authorization
- **Role-Based Access Control**: RBAC system
- **Resource Permissions**: Permission per resource
- **Operation Permissions**: Permission per operation
- **Time-based Access**: Akses berdasarkan waktu

### Security Features
- **SSL/TLS Encryption**: Enkripsi koneksi
- **Data Encryption**: Enkripsi data at rest
- **Audit Logging**: Log semua aktivitas
- **IP Whitelisting**: Restrict access by IP
- **Rate Limiting**: Batas request per user

---

## 🚀 Quick Start Guide

### 1. Install QubeDB
```bash
# Download installer
QubeDB-Setup.exe

# Or build from source
git clone https://github.com/qubedb/qubedb.git
cd qubedb
cargo build --release
```

### 2. Launch GUI
```bash
# Windows
qubedb-gui.exe

# Or from desktop shortcut
# Double-click "QubeDB Desktop"
```

### 3. Configure Server
1. Open **Server Settings**
2. Set **Host**: localhost
3. Set **Port**: 8080
4. Set **Admin Password**
5. Click **Save Settings**

### 4. Create Database
1. Go to **Databases** tab
2. Click **Create Database**
3. Enter database name
4. Select character set
5. Click **Create Database**

### 5. Connect Framework
1. Go to **Framework Integration**
2. Select your framework (Laravel, Django, etc.)
3. Copy configuration
4. Paste to your project
5. Install driver
6. Test connection

---

## 📞 Support

- 📧 **Email**: support@qubedb.com
- 💬 **Discord**: [Join our community](https://discord.gg/qubedb)
- 📖 **Documentation**: [docs.qubedb.com](https://docs.qubedb.com)
- 🐛 **Issues**: [GitHub Issues](https://github.com/qubedb/qubedb/issues)

---

**QubeDB GUI** - The future of database management! 🚀
