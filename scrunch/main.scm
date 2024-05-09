(require db)
(require xml)
(require racket/date)
(require web-server/templates)

(date-display-format 'iso-8601)

(define (now)
  (date->string (current-date)))

(define conn
  (sqlite3-connect #:database "./db/crunch.db"))

(define (vector->struct name vec)
  ; todo - this probably doesn't work
  (struct (splice vector->list vec)))

;;; --- Link ---

(struct link [id profile-id title href updated-at created-at])

(define (link/find id)
  (let ([sql "select * from links where id = $1;"])
    (vector->struct link
                    (query-maybe-row conn sql))))

(define (link/find-all)
  (let ([sql "select * from links;"])
    (vector->struct link
                    (query-rows conn sql))))

(define (link/create! profile-id title href)
  (let ([sql "insert into links
              (profile_id, title, href, created_at, updated_at)
              values ($1, $2, $3, $4, $5);"])
    (query-exec conn sql profile-id title href (now) (now))))

(define (link/modify! id title href)
  (let ([sql "update links
              set title = $1, href = $2, updated_at = $3
              where id = $4;"])
    (query-exec conn sql title href (now) id)))

(define (link/delete! id)
  (let ([sql "delete from links where id = $1;"])
    (query-exec conn sql)))

(define (link/show link)
  (xexpr->string
    `(li
       (a ((href ,(href link)) (target "_blank")) ,(title link)))))

(define (link/index links)
  (xexpr->string
    `(ul
       ,@(map show links))))
