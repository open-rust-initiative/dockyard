database_password=$(cat .env | grep DATABASE_PASSWORD | awk  -F= '{print $2}' | tr -d "\n")
cp ./migrations/v1/up.sql ./db/
cd db &&
 docker build -t dockyard/database:0.1 . &&
 cd ../
cd portal && npm install && npm run build && cd ../

docker network create --driver bridge dockyard
docker run --name dockyard-db --network dockyard -e MYSQL_ROOT_PASSWORD=$database_password  -d dockyard/database:0.1  --character-set-server=utf8mb4 --collation-server=utf8mb4_unicode_ci
docker run --name dockyard-redis --network dockyard -d redis
cargo build --release
database_url="DATABASE_URL=mysql://root:"$database_password"@dockyard-db:3306/dockyard?useSSL=false&allowPublicKeyRetrieval=true"
redis_url="REDIS_URL=redis://dockyard-redis:6379"
echo $database_url >> .env
echo $redis_url >> .env
sed -i 's#^BASE_STORGE_URL=.*#BASE_STORGE_URL=/opt/dockyard/files/#g' .env
docker build -t dockyard/core:0.1 .
docker run --name dockyard-core -p 443:4000 --network dockyard  -itd dockyard/core:0.1



