(lambda $var_01
  (lambda $var_02
    (lambda $var_03
      (lambda $var_04
        (lambda $var_05
          (lambda $var_06
            (sum (sum (var $var_01)
                      $var_07
                      $var_08
                      (let (get
                            (var $var_01)
                            (+
                             (var $var_07)
                             1))
                        $var_09
                        (sing (var $var_07)
                              (sum (subarray (var $var_02) (var $var_08) (- (var $var_09) 1))
                                   $var_10
                                   $var_11
                                   (sing (unique (var $var_11)) (get (var $var_03) (var $var_10)))))))
                 $var_07
                 $var_08
                 (sing (var $var_07)
                       (sum (var $var_08)
                            $var_09
                            $var_10
                            (* (var $var_10)
                               (get (sum (var $var_04)
                                         $var_11
                                         $var_12
                                         (let (get
                                               (var $var_04)
                                               (+
                                                (var $var_11)
                                                1))
                                           $var_13
                                           (sing (var $var_11)
                                                 (sum (subarray (var $var_05)
                                                                (var $var_12)
                                                                (- (var $var_13) 1))
                                                      $var_14
                                                      $var_15
                                                      (sing (unique (var $var_15))
                                                            (get (var $var_06) (var $var_14)))))))
                                    (var $var_09))))))))))))
