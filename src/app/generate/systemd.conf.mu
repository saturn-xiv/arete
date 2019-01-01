[Unit]
Description={{description}}
After=redis.service postgresql.service rabbitmq.service

[Service]
Type=simple
User={{user}}
Group={{group}}
WorkingDirectory={{root}}
ExecStart={{root}}/{{name}}
Restart=on-failure # or always, on-abort, etc

[Install]
WantedBy=multi-user.target
