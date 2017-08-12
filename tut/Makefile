TARGETS = \
	hello



all: ${TARGETS}

clean:
	touch ${TARGETS}
	rm    ${TARGETS}

mrproper: clean
	touch dummy~
	rm        *~



hello: hello.rs
	rustc hello.rs

