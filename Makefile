docs/index.html: index.textile
	redcloth index.textile > docs/index.html

index.textile: index.textile.in index.sed features.textile
	sed -f index.sed index.textile.in > $@

features.textile: compare.py src/*.rs
	python2 compare.py > $@

clean:
	rm -f index.textile features.textile
