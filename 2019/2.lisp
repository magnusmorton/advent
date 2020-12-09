(require :asdf)
(asdf:load-system :uiop)

(defun read-input () 
  (with-open-file (in "input")
    (read-line in)))

(defun get-input ()
  (mapcar 'parse-integer (uiop:split-string (read-input) :separator ",")))

(defun intcode (lst noun verb)
  (setf  (subseq lst 1 3) (list noun verb))
  (loop for i from 0 to (length lst) by 4
     until (= 99 (nth i lst))
     do
       (let* ((val (nth i lst))
              (pos1 (nth (+ i 1) lst))
              (pos2 (nth (+ i 2) lst))
              (pos3 (nth (+ i 3) lst))
              (arg1 (nth pos1 lst))
              (arg2 (nth pos2 lst)))
         (cond ((= 1 val) (setf (nth pos3 lst) (+ arg1 arg2)))
               ((= 2 val) (setf (nth pos3 lst) (* arg1 arg2))))))
  (car lst))

(intcode (get-input) 12 2)

(let ((res 0))
  (loop for i from 0 to 100
     until (= res 19690720)
     do
       (loop for j from 0 to 100
          until (= res 19690720)
          do
            (print (cons i j))
            (setf res (intcode (get-input) i j)))))

