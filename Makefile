dist=dist

deb: frontend
	cargo deb

musl: frontend
	mkdir -pv $(dist)	
	PKG_CONFIG_ALLOW_CROSS=1 cargo build --release --target=x86_64-unknown-linux-musl	
	strip -s target/x86_64-unknown-linux-musl/release/arete
	cp -rv target/x86_64-unknown-linux-musl/release/arete assets log4rs.yml package.json package-lock.json LICENSE README.md $(dist)/	
	cp -rv dashboard/dist $(dist)/dashboard
	cd $(dist) && tar cfJ ../$(dist).tar.xz *

frontend:
	if [ ! -d "node_modules" ]; then npm install; fi
	cd dashboard && (if [ ! -d "node_modules" ]; then npm install; fi) && npm run build

clean:
	cargo clean
	rm -rf $(dist) $(dist).tar.xz node_modules
	cd dashboard && rm -rf dist node_modules

