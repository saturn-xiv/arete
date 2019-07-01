
- Create tablespace

```bash
> drop tablespace TABLESPACE including contents;
> create tablespace TABLESPACE datafile 'TABLESPACE.dbf' size 500M autoextend on next 5M maxsize unlimited; 
> drop user USER cascade;
> create user USER identified by PASSWORD default tablespace TABLESPACE;
> grant connect,resource to USER;
```

- Create user 

```bash
> conn system/123456 as sysdba
> alter session set "_ORACLE_SCRIPT"=true;  
> create user guest identified by 123456;
> grant connect,resource,dba,select any table to guest;
```

- Common commands

```bash
> SELECT table_name FROM user_tables; 
> SHOW TABLES;
> SHOW VIEWS;
> describe TABLE-NAME;
> select * from nls_instance_parameters;
> select * from nls_instance_parameters;
> select * from nls_session_parameters;
```

