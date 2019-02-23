## RabbitMQ

```bash
rabbitmq-plugins enable rabbitmq_management
rabbitmqctl change_password guest change-me
rabbitmqctl add_user who-am-i change-me
rabbitmqctl set_user_tags who-am-i administrator
rabbitmqctl list_vhosts
rabbitmqctl add_vhost v-host
rabbitmqctl set_permissions -p v-host who-am-i ".*" ".*" ".*"
```
