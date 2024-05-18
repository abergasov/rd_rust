define data
title1, title2, longtitlemorethan15symbols3
val1, val2, val3, val4
val5, val6, val7
val9, val10, val11, val12
endef
export data
run:
	cargo run -- lowercase "HELLO, WORLD!" \
		uppercase "hello, world!" \
		nospaces "HELLO, WORLD!" \
		slugify "hello, world!" \
		slugify "" \
		apage "a"

run_csv:
	cargo run -- csv "$$data"