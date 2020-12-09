(defun calc (x)
  (- (floor x 3) 2))


(defun calc-recur (x)
  (let ((res (calc x)))
    (if (< res 0)
        0
        (+ res (calc-recur res)))))

(defun read-input () 
  (with-open-file (in "input.dat")
    (loop for line = (read-line in nil)
       until (null line)
       collect (parse-integer line))))



