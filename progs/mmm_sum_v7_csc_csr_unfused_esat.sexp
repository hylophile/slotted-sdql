(lambda $var_01
  (lambda $var_02
    (lambda $var_03
      (lambda $var_04
        (lambda $var_05
          (lambda $var_06
            (sum (var $var_01)
                 $var_07
                 $var_08
                 (* (sum (subarray (var $var_02)
                                   (var $var_08)
                                   (- (get (var $var_01) (+ (var $var_07) 1)) 1))
                         $var_09
                         $var_10
                         (get (var $var_03) (var $var_09)))
                    (sum (subarray (var $var_05)
                                   (get (var $var_04) (var $var_07))
                                   (- (get (var $var_04) (+ (var $var_07) 1)) 1))
                         $var_09
                         $var_10
                         (get (var $var_06) (var $var_09)))))))))))
