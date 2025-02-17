(lambda $var_01
  (lambda $var_02
    (sum (var $var_01)
         $var_03
         $var_04
         (* (sum (var $var_04) $var_05 $var_06 (var $var_06))
            (sum (get (var $var_02) (var $var_03)) $var_05 $var_06 (var $var_06))))))
