
- Create tablespace

```sql
> drop tablespace TABLESPACE including contents and datafiles;
> create tablespace TABLESPACE datafile 'TABLESPACE.dbf' size 500M autoextend on next 5M maxsize unlimited; 
> drop user USER cascade;
> create user USER identified by PASSWORD default tablespace TABLESPACE;
> grant connect,resource to USER;
```

- Create user 

```sql
> conn system/123456 as sysdba
> alter session set "_ORACLE_SCRIPT"=true;  
> create user guest identified by 123456;
> grant connect,resource,dba,select any table to guest;
```

- Common commands

```sql
> SELECT table_name FROM user_tables; 
> SHOW TABLES;
> SHOW VIEWS;
> describe TABLE-NAME;
> select * from nls_instance_parameters;
> select * from nls_instance_parameters;
> select * from nls_session_parameters;
```

- Check encode

```sql
> select userenv(‘language’) from dual;
```

- Change encode

```sql
> connect sys as sysdba
> shutdown immediate
> startup mount
> alter system enable restricted session;
> alter system set JOB_QUEUE_PROCESSES=0;
> alter system set AQ_TM_PROCESSES=0; 
> alter database open; 
> alter database character set internal_use AL32UTF8;
> shutdown immediate 
> startup
```

- For chinese

```bash
NLS_CHARACTERSET=AL32UTF8
NLS_LANG=SIMPLIFIED CHINESE_CHINA.AL32UTF8
```