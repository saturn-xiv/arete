## MySql

-   create database with owner

```bash
$ mysql -u root -p
> CREATE DATABASE db-name;
> CREATE USER 'who-am-i'@'localhost' IDENTIFIED BY 'change-me';
> GRANT ALL PRIVILEGES ON db-name.* TO 'who-am-i'@'localhost';
> FLUSH PRIVILEGES;
```


- change user password

```bash
> USE mysql;
> UPDATE user SET password=PASSWORD('YourPasswordHere') WHERE User='root' AND Host = 'localhost';
> FLUSH PRIVILEGES;
```

- backup & restore

```bash
$ mysqldump -u root -p --all-databases > all.sql 
$ mysqldump -u root -p --opt --skip-extended-insert one > one.sql
$ mysql -u root -p one < one.sql
```

-   usage

```bash
SHOW DATABASES; # show databases
USE db-name; # connect database
SHOW TABLES; # show tables
DESC table-name; # show table scheme
```

-   forgot mysql root password

create file  /tmp/reset.mysqld

```sql
SET PASSWORD FOR root@localhost = PASSWORD('change-me');
```

edit file /etc/mysql/my.cnf

```text
[mysqld]
init-file=/tmp/reset.mysqld
```
