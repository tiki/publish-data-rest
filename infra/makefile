.PHONY: clean, clean-deep

zoneId = 7be31970110fa56b1f8e1fe915a8b938
routePrefix = ingest.mytiki.com/api/latest
namePrefix = publish-data-rest

clean:
	rm -rf ../endpoint/build
	cd ../endpoint && cargo remove publish-data-rest-schema  && cargo add --path ../schema --features dummy
	cd ../endpoint && cargo clean

deploy: clean
	cd ../endpoint && cargo remove publish-data-rest-schema && cargo add --path ../schema --features $(feature)
	npx wrangler deploy --name $(namePrefix)-$(subst _,-,$(feature)) --route $(routePrefix)/$(subst _,-,$(feature))

dev: clean
	cd ../endpoint && cargo remove publish-data-rest-schema && cargo add --path ../schema --features $(feature)
	npx wrangler dev --name $(namePrefix)-$(subst _,-,$(feature)) --route $(routePrefix)/$(subst _,-,$(feature))
