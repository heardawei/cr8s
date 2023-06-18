# 根据当前文件夹的docket-compose.yaml文件，启动所有容器
docker-compose up -d

# diesel库初始化
docker-compose exec app diesel setup

# diesel为rustaceans创建migration
docker-compose exec app diesel migration generate create_rustaceans

# down.sql
# DROP TABLE rustaceans

# up.sql
# CREATE TABLE rustaceans(
#   id SERIAL PRIMARY KEY,
#   name VARCHAR NOT NULL,
#   email VARCHAR NOT NULL,
#   created_at TIMESTAMP DEFAULT NOW() NOT NULL
# )

# diesel为crates创建migration
docker-compose exec app diesel migration generate create_crates

# down.sql
# DROP TABLE crates

# up.sql
# CREATE TABLE crates (
#   id SERIAL PRIMARY KEY,
#   rustacean_id integer NOT NULL REFERENCES rustaceans(id),
#   code varchar(64) NOT NULL,
#   name varchar(128) NOT NULL,
#   version varchar(64) NOT NULL,
#   description text,
#   created_at TIMESTAMP DEFAULT NOW() NOT NULL
# )

# diesel运行所有挂起的migration
docker-compose exec app diesel migration run

# 启动主程序
docket-compose exec app cargo run

# 运行测试
docket-compose exec app cargo test
