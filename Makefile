README.textile: README.textile.in README.sed features.textile
	sed -f README.sed README.textile.in > $@

features.textile: compare.py src/*.rs
	python2 compare.py > $@

clean:
	rm -f README.textile features.textile
