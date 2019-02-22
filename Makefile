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
	DATABASE_URL="postgres://postgres:@localhost:5432/arete" diesel print-schema > src/orm/schema.rs
