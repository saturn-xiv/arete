dist=dist

build:
	mkdir -pv $(dist)
	GIT_HEAD=`git rev-parse --short HEAD` BUILD_TIME=`date -R` cargo build --release
	strip -s target/release/arete
	cp -rv target/release/arete db themes locales log4rs.yml package.json package-lock.json LICENSE README.md $(dist)/
	cd dashboard && npm run build
	cp -rv dashboard/build $(dist)/dashboard
	cd $(dist) && tar cfJ ../$(dist).tar.xz *

clean:
	rm -rv $(dist)
	cargo clean
	cd dashboard && rm -rf build

check:
	cargo check
	cargo build
	cargo doc

schema:
	DATABASE_URL="postgres://postgres:@localhost:5432/arete" diesel print-schema > src/orm/schema.rs

npm:
	npm install
	cd dashboard && npm install
