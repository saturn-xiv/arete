dist=dist

musl: react
	mkdir -pv $(dist)	
	PKG_CONFIG_ALLOW_CROSS=1 cargo build --release --target=x86_64-unknown-linux-musl	
	strip -s target/x86_64-unknown-linux-musl/release/arete
	cp -rv target/x86_64-unknown-linux-musl/release/arete log4rs.yml package.json package-lock.json LICENSE README.md $(dist)/	
	cp -rv dashboard/build $(dist)/dashboard
	cd $(dist) && tar cfJ ../$(dist).tar.xz *

react:
	if [ ! -d "node_modules" ]; then npm install; fi
	cd dashboard && (if [ ! -d "node_modules" ]; then npm install; fi) && npm run build

clean:
	rm -rv $(dist) $(dist).tar.xz node_modules
	cargo clean
	cd dashboard && rm -rf build node_modules

schema:
	diesel print-schema -o schema_migrations > src/orm/schema.rs
	diesel print-schema -o locales > src/i18n/schema.rs
	diesel print-schema -o settings > src/settings/schema.rs	
	diesel print-schema -o users logs policies attachments notifications cards links friend_links leave_words votes tags tag_resources categories category_resources > src/plugins/nut/schema.rs
	diesel print-schema -o forum_topics forum_posts > src/plugins/forum/schema.rs
	diesel print-schema -o survey_forms survey_fields survey_responses survey_logs survey_subscribers > src/plugins/survey/schema.rs
	diesel print-schema -o vip_members > src/plugins/vip/schema.rs
