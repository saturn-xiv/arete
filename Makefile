dist=dist

package: assets
	mkdir -pv $(dist)	
	PKG_CONFIG_ALLOW_CROSS=1 cargo build --release --target=x86_64-unknown-linux-musl	
	strip -s target/x86_64-unknown-linux-musl/release/arete
	cp -rv target/x86_64-unknown-linux-musl/release/arete db themes locales log4rs.yml package.json package-lock.json LICENSE README.md $(dist)/	
	cp -rv dashboard/build $(dist)/dashboard
	cd $(dist) && tar cfJ ../$(dist).tar.xz *

assets:
	if [ ! -d "node_modules" ]; then npm install; fi
	cd dashboard && (if [ ! -d "node_modules" ]; then npm install; fi) && npm run build

clean:
	rm -rv $(dist) $(dist).tar.xz
	cargo clean
	cd dashboard && rm -rf build

schema:
	diesel print-schema -o schema_migrations > src/orm/schema.rs
	diesel print-schema -o settings > src/settings/schema.rs
	diesel print-schema -o locales > src/i18n/schema.rs
