-- Your SQL goes here
-- Your SQL goes here
CREATE DATABASE IF NOT EXISTS dockyard;
use dockyard;
create table if not exists user(
       id int primary key auto_increment,
       username char(255) unique not null ,
       name char(20) not null ,
       password char(64) not null ,
       admin bool not null ,
       email varchar(50) not null ,
       comment varchar(255) null
);
create table if not exists fs_layers(
       id int unsigned primary key auto_increment,
       digest varchar(255) not null unique,
       size int unsigned not null,
       mediaType varchar(100) not null default 'application/vnd.oci.image.rootfs.diff.tar.gzip',
       path TEXT not null,
       creation_time datetime default CURRENT_TIMESTAMP not null
);
create table if not exists images(
     id int unsigned primary key auto_increment,
     library varchar(30) not null default 'default',
     name varchar(255) not null,
     tag varchar(255) not null,
     fslayer_configid int unsigned not null ,
     creation_time datetime default CURRENT_TIMESTAMP  not null ,
     pull_time datetime default '0001-01-01 00:00:00' not null ,
     push_time datetime default  CURRENT_TIMESTAMP on update CURRENT_TIMESTAMP  not null ,
     constraint unique_image unique (library,name,tag),
     constraint image_fs_id foreign key (fslayer_configid) references fs_layers(id) on delete cascade
);
create table if not exists config_layers(
   id int unsigned primary key auto_increment,
    configid int unsigned not null,
    layersid int unsigned not null,
    creation_time datetime default CURRENT_TIMESTAMP  not null ,
    constraint configid_fs_id foreign key (configid) references fs_layers(id) on delete cascade,
    constraint layers_fs_id foreign key (layersid) references fs_layers(id) on delete cascade
);