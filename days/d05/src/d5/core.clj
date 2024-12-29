(ns d5.core
  (:require
   [clojure.string :as str])
  (:gen-class))

(defn read-all-lines []
  (str/split-lines (slurp *in*)))

(defn read-input []
  (let [lines (read-all-lines)
        [a, b] (split-with (complement #{""}) lines)
        [a, b] [a, (drop 1 b)]
        a (map #(str/split %1 #"\|") a)
        b (map #(str/split %1 #",") b)
        a (vec (map #(map Integer/parseInt %1) a))
        b (vec (map #(map Integer/parseInt %1) b))]
    [a, b]))

(defn rule-exists? [rules [l r]]
  (> (.indexOf rules [l, r]) -1))

(defn take-middle [items]
  (let [l (count items)
        t (/ (- l 1) 2)] (first (drop t items))))

(defn put-in-order [rules items]
  (sort (fn [l, r] (rule-exists? rules [r, l])) items))

(defn check-rules [rules items]
  (let [indices (take (count items) (range))
        badness (for [i indices
                      j (drop (+ i 1) indices)]
                  (let [l (nth items i)
                        r (nth items j)
                        bad (rule-exists? rules [r, l])] bad))
        bad_items (filter #(= %1 true) badness)
        bad (count bad_items)
        ok (= bad 0)] ok))

(defn -main [& _args]
  (let [[rules, cases] (read-input)
        res (group-by (fn [c] (check-rules rules c)) cases)

        in-order (get res true)
        fixed (map #(put-in-order rules %1) (get res false))

        middles_1 (map take-middle in-order)
        middles_2 (map take-middle fixed)

        part1 (reduce + middles_1)
        part2 (reduce + middles_2)]
    (println part1)
    (println part2)))
