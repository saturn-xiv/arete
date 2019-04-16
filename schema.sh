#!/bin/sh

set -e

DATABASE_URL="postgres://postgres:@localhost:5432/arete"
diesel print-schema -o schema_migrations > src/orm/schema.rs
# diesel print-schema -o locales > src/i18n/schema.rs
# diesel print-schema -o settings > src/settings/schema.rs	
# diesel print-schema -o users logs policies attachments notifications cards links friend_links leave_words votes tags tag_resources categories category_resources > src/plugins/nut/schema.rs
# diesel print-schema -o forum_topics forum_posts > src/plugins/forum/schema.rs
# diesel print-schema -o survey_forms survey_fields survey_responses survey_logs survey_subscribers > src/plugins/survey/schema.rs
# diesel print-schema -o vip_members > src/plugins/vip/schema.rs

# DATABASE_URL="mysql://root:@localhost:3306/arete"
# diesel print-schema -o schema_migrations > src/orm/schema.rs
# diesel print-schema -o locales > src/i18n/schema.rs
# diesel print-schema -o settings > src/settings/schema.rs	
# diesel print-schema -o users logs policies attachments notifications cards links friend_links leave_words votes tags tag_resources categories category_resources > src/plugins/nut/schema.rs
# diesel print-schema -o forum_topics forum_posts > src/plugins/forum/schema.rs
# diesel print-schema -o survey_forms survey_fields survey_responses survey_logs survey_subscribers > src/plugins/survey/schema.rs
# diesel print-schema -o vip_members > src/plugins/vip/schema.rs

# DATABASE_URL="tmp/db"
# diesel print-schema -o schema_migrations > src/orm/schema.rs
# diesel print-schema -o locales > src/i18n/schema.rs
# diesel print-schema -o settings > src/settings/schema.rs	
# diesel print-schema -o users logs policies attachments notifications cards links friend_links leave_words votes tags tag_resources categories category_resources > src/plugins/nut/schema.rs
# diesel print-schema -o forum_topics forum_posts > src/plugins/forum/schema.rs
# diesel print-schema -o survey_forms survey_fields survey_responses survey_logs survey_subscribers > src/plugins/survey/schema.rs
# diesel print-schema -o vip_members > src/plugins/vip/schema.rs

exit 0
