# 根据当前文件夹的docket-compose.yaml文件，启动所有容器
docker-compose up -d

# diesel库初始化
docker-compose exec app diesel setup

# diesel为rustaceans创建migration
docker-compose exec app diesel migration generate create_rustaceans

# diesel为crates创建migration
docker-compose exec app diesel migration generate create_crates

# diesel运行所有挂起的migration
docker-compose exec app diesel run

# 启动主程序
docket-compose exec app cargo run

# 运行测试
docket-compose exec app cargo test
