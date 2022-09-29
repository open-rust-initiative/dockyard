# 概述
dockyard 是一个符合 OCI Distribution 标准的 Registry 服务，使用rust语言编写

### 技术栈

+ 后端：tokio runtime + actix-web + diesel (orm) + tracing (log)
+ 前端：vue3 + element-plus + echarts + pinia + vue-i18n

## 安装

+ 配置文件（.env）

  | var                                                | detail                                                       |
    | -------------------------------------------------- | ------------------------------------------------------------ |
  | JWT_SECRET (required)                              | jwt服务的密钥                                                |
  | TOKEN_EXPIRES_IN (required)                        | token有效时间                                                |
  | SERVERHOST (required)                              | 对为暴露的域名，容器化部署时不可为localhost，需要http或https前缀 |
  | BASE_STORGE_URL (required)                         | 本地存储目录，镜像文件皆存于之下                             |
  | DATABASE_POOL_MAX_SIZE (default 15 )               | 数据库连接池最大连接数                                       |
  | DATABASE_POOL_CONNECTION_TIMEOUT (default 30 sec ) | 数据库连接池超时时间                                         |
  | REDIS_POOL_MAX_SIZE (default 15 )                  | Redis连接池最大连接数                                        |
  | REDIS_POOL_CONNECTION_TIMEOUT (default 30 sec)     | Redis连接池超时时间                                          |

+ 容器化自动部署（适合生产环境部署，默认使用https）

  使用`install.sh`

  注意编译环境

    + nodejs > 14
    + `libssl-dev` on Ubuntu or `openssl-devel` on Fedora. `pkg-config`
    + `libmysqlclient-dev` on Ubuntu or `mysql-devel` on Fedora

  执行流程

    1. 将`migrations`下的*up.sql*文件复制到db目录下，运行*Dockerfile*（以mysql:8为基础），使得容器创建时自动运行`sql`文件，创建数据库和表，创建镜像 `dockyard/database`

    2. 使用npm编译前端工程，输出到`static`目录，由actix-web提供服务

    3. 创建容器网路`dockyard`，在该网络下创建*dockyard-db* *dockyard-redis*容器实例

    4. 编译*dockyard* 后端服务，基于 *db* 和 *redis* 实例 自动生成*DATABASE_URL* *REDIS_URL* 到 .env 文件中

    5. 修改*BASE_STORGE_URL* 为 `/opt/dockyard/files/` ，在容器实例中 上传的镜像文件皆保存于之下。注意

       *SERVERHOST* 设置为https

    6. 构建镜像，创建实例*dockyard-core* 默认绑定443端口

+ 手动部署（适合debug）

    1. 进入 `portal` 目录  安装依赖`npm install` 构建 `npm run build` 生成结果到 `static`目录

    2. 使用 `migrations`下的*up.sql* 创建所需的数据库和表

    3. 运行 `cargo build --release` 编译rust 文件

    4. 编辑.env 配置文件 根据现有 mysql 和 redis 添加 *DATABASE_URL* *REDIS_URL*

    5. 将 `target/release/dockyard` 文件 、`.env` 、`static`目录 、ssl密钥文件 `cert.pem key.pem` 放入到同一目录下

       `./dockyard` 即可运行。（注意执行环境需要libmysqlclient-dev）

+ https 配置

  将SERVERHOST前缀设置为https，根据自身需要替换ssl 密钥，或自行生成

  ```shell
  openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'
  ```
    注意自行生成的ssl密钥，没有三方证书，故需要在`/etc/docker/daemon.json` 配置

    ```json
    "insecure-registries": [
        "oci.org"
    ]
    ```

## 测试

+ OCI  标准 提供了 一致性检验工具 https://github.com/opencontainers/distribution-spec/tree/main/conformance

  `conformance` 目录为该工具的复制版

+ `config`文件中即 `conformance.test`所需的变量
  `source ./config` 加载变量的配置
  `source ./unconfig` 取消 `config`变量设置

  变量细节参考 该目录下的`README.md`文件

+ 运行`./conformance.test` 会在该目录下生成`report.html`文件 
+ 注意https测试不成功,需要配置为http,建议配置`SERVERHOST=http://127.0.0.1:4000`，
  并修改`conformance/config`文件:`OCI_ROOT_URL="http://127.0.0.1:4000"`
  注意 `BASE_STORGE_URL` 对应路径有文件创建权限 
  注意必须声明 `DATABASE_URL` `REDIS_URL`


### How to use?

### How to build?

### How to add your cargo plugin to Lathes?

### How to contribute?

This project enforce the [DCO](https://developercertificate.org).

Contributors sign-off that they adhere to these requirements by adding a Signed-off-by line to commit messages.

```bash
This is my commit message

Signed-off-by: Random J Developer <random@developer.example.org>
```

Git even has a -s command line option to append this automatically to your commit message:

```bash
$ git commit -s -m 'This is my commit message'
```

### License

Freighter is licensed under this Licensed:

* MIT LICENSE ( [LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)

### Acknowledgements


  

  




