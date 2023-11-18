(defpackage :lisp-server
  (:use :cl :cl-who :woo :trivia)
  (:export
    #:main))

(load "~/.quicklisp/setup.lisp")

(ql:quickload "woo")
(ql:quickload "trivia")
(ql:quickload "cl-who")

; For something a bit more high-level, see https://github.com/fukamachi/caveman or
; https://github.com/fukamachi/ningle. Caveman is built on top of ningle so it is slightly more
; high level.

(in-package :lisp-server)

(defun html (status body)
  `(,status (:content-type "text/html") (,body)))

(defun plain (status body)
  `(,status (:content-type "text/plain") (,body)))

(defun render-index ()
  (with-html-output-to-string (out nil :prologue t :indent t)
    (:html :xmlns "http://www.w3.org/1999/xhtml" :xml\:lang "en" :lang "en"
      (:head
        (:meta :http-equiv "Content-Type" :content "text/html;charset=utf-8")
        (:title "Lisp server")
        (:link :type "text/css" :rel "stylesheet" :href "/public/index.css"))
      (:body
        (:p "Hello, there")))))

(defun handler (env)
  (destructuring-bind (&key request-method path-info &allow-other-keys) env
    (match path-info
      ("/" (match request-method
        (:get (html 200 (render-index)))
        (_ (plain 405 "Method not allowed"))))
      ("/hello" (match request-method
        (:get (plain 200 "Hello, world!"))
        (_ (plain 405 "Method not allowed"))))
      ("/public/index.css" (match request-method
        (:get '(200 (:content-type "text/css") #p"public/index.css"))
        (_ (plain 405 "Method not allowed"))))
      (_ (plain 404 "Not found")))))

(defun start-server ()
  (run
    (quote handler)
    :port 4242))

(defun main ()
  (start-server)
  (join-thread
    (find-if
      (lambda (th)
        (search "woo" (thread-name th)))
      (list-all-threads))))
