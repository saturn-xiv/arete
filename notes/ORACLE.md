
- Create user 

```bash
> conn system/123456 as sysdba
> alter session set "_ORACLE_SCRIPT"=true;  
> create user guest identified by 123456;
> grant connect,resource,dba,select any table to guest;
```

- Common commands

```bash
> SHOW SCHEMAS;
> SHOW TABLES;
> SHOW VIEWS;
> describe TABLE-NAME;
> select * from nls_instance_parameters;
> select * from nls_instance_parameters;
> select * from nls_session_parameters;
```
