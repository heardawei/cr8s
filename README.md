# cr8s

## 配置运行环境

### diesel库初始化

```cmd
docker-compose exec app diesel setup
```

### diesel为rustaceans创建migration

```cmd
docker-compose exec app diesel migration generate create_rustaceans
```

- down.sql

```sql
DROP TABLE rustaceans
```

- up.sql

```sql
CREATE TABLE rustaceans(
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  created_at TIMESTAMP DEFAULT NOW() NOT NULL
)
```

### diesel为crates创建migration

```cmd
docker-compose exec app diesel migration generate create_crates
```

- down.sql

```sql
DROP TABLE crates
```

- up.sql

```sql
CREATE TABLE crates (
  id SERIAL PRIMARY KEY,
  rustacean_id integer NOT NULL REFERENCES rustaceans(id),
  code varchar(64) NOT NULL,
  name varchar(128) NOT NULL,
  version varchar(64) NOT NULL,
  description text,
  created_at TIMESTAMP DEFAULT NOW() NOT NULL
)
```

### diesel运行所有挂起的migration

```cmd
docker-compose exec app diesel migration run
```

## 启动程序

```cmd
docker-compose exec app cargo run
```

## 运行测试

```cmd
docker-compose exec app cargo test
```
