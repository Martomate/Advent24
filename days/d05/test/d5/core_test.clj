(ns d5.core-test
  (:require [clojure.test :refer [deftest is testing]]
            [d5.core :as d5]))

(deftest a-test
  (testing "basic test"
    (is (= (d5/two) 2))))
