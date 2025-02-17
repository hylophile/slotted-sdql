(lambda $var_01
  (lambda $var_02
    (sum (var $var_02)
         $var_03
         $var_04
         (sing (var $var_03)
               (sum (var $var_04)
                    $var_05
                    $var_06
                    (sing (var $var_05)
                          (sum (var $var_01)
                               $var_07
                               $var_08
                               (sing (var $var_07)
                                     (sum (var $var_06)
                                          $var_09
                                          $var_10
                                          (* (var $var_10) (get (var $var_08) (var $var_09))))))))))))
