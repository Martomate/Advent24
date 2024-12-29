(ns d5.core-test
  (:require [clojure.test :refer [deftest is testing]]
            [d5.core :as d5]))

(deftest rule-exists
  (testing "it exists"
    (is (d5/rule-exists? [[7, 5]] [7, 5])))
  (testing "it does not exist"
    (is (not (d5/rule-exists? [[7, 4]] [7, 5]))))
  (testing "the reverse exists"
    (is (not (d5/rule-exists? [[5, 7]] [7, 5]))))
  )
