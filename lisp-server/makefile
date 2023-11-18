run:
	sbcl \
		--load lisp-server.asd \
		--eval '(ql:quickload :lisp-server)' \
		--eval '(lisp-server:main)'
		
build:
	sbcl --eval '(sb-ext:save-lisp-and-die #p"lisp-server" :toplevel #'main :exectuable t)'
