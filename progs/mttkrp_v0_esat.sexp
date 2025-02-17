(lambda $var_01
  (lambda $var_02
    (lambda $var_03
      (sum (var $var_01)
           $var_04
           $var_05
           (sing (var $var_04)
                 (sum (var $var_05)
                      $var_06
                      $var_07
                      (sum (get (var $var_02) (var $var_06))
                           $var_08
                           $var_09
                           (sing (var $var_08)
                                 (* (var $var_09)
                                    (sum (var $var_07)
                                         $var_10
                                         $var_11
                                         (* (var $var_11)
                                            (get (get (var $var_03) (var $var_08))
                                                 (var $var_10)))))))))))))
