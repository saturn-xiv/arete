#!/bin/sh

set -e

declare -A db
db["postgresql"]="postgres://postgres:@localhost:5432/arete"
db["mysql"]="mysql://root:@localhost:3306/arete"
db["sqlite"]="tmp/db"

for i in "${!db[@]}"
do
    echo "generate schema files for $i"
    export DATABASE_URL=${db[$i]}
    diesel print-schema -o schema_migrations > src/orm/$i/schema.rs
    diesel print-schema -o locales > src/i18n/$i/schema.rs
    diesel print-schema -o settings > src/settings/$i/schema.rs	
    diesel print-schema -o users logs policies attachments notifications cards links friend_links leave_words votes tags tag_resources categories category_resources > src/plugins/nut/$i/schema.rs
    diesel print-schema -o forum_topics forum_posts > src/plugins/forum/$i/schema.rs
    diesel print-schema -o survey_forms survey_fields survey_responses survey_logs survey_subscribers > src/plugins/survey/$i/schema.rs
    diesel print-schema -o vip_members > src/plugins/vip/$i/schema.rs
    diesel print-schema -o vpn_users vpn_logs > src/plugins/ops/vpn/$i/schema.rs
    diesel print-schema -o monitor_logs > src/plugins/ops/monitor/$i/schema.rs
    diesel print-schema -o ops_mail_domains ops_mail_users ops_mail_aliases > src/plugins/ops/mail/$i/schema.rs
done

cargo fmt

exit 0
