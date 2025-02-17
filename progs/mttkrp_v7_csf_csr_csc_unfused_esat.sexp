(lambda $var_01
  (lambda $var_02
    (lambda $var_03
      (lambda $var_04
        (lambda $var_05
          (lambda $var_06
            (lambda $var_07
              (lambda $var_08
                (lambda $var_09
                  (lambda $var_10
                    (lambda $var_11
                      (lambda $var_12
                        (sum (var $var_01)
                             $var_13
                             $var_14
                             (sing
                              (var $var_14)
                              (sum (subarray (var $var_03)
                                             (get (var $var_02) (var $var_13))
                                             (- (get (var $var_02) (+ (var $var_13) 1)) 1))
                                   $var_15
                                   $var_16
                                   (sum (subarray (var $var_08)
                                                  (get (var $var_07) (var $var_16))
                                                  (- (get (var $var_07) (+ (var $var_16) 1)) 1))
                                        $var_17
                                        $var_18
                                        (sing
                                         (var $var_18)
                                         (* (get (var $var_09) (var $var_17))
                                            (merge
                                             (subarray (var $var_05)
                                                       (get (var $var_04) (var $var_15))
                                                       (- (get (var $var_04) (+ (var $var_15) 1)) 1))
                                             (subarray (var $var_11)
                                                       (get (var $var_10) (var $var_18))
                                                       (- (get (var $var_10) (+ (var $var_18) 1)) 1))
                                             $var_19
                                             $var_21
                                             $var_20
                                             (* (get (var $var_06) (var $var_19))
                                                (get (var $var_12) (var $var_21))))))))))))))))))))))
