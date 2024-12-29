(defproject d5 "0.1"
  :dependencies [[org.clojure/clojure "1.12.0"]
                 [clojure.java-time/clojure.java-time "1.4.3"]]
  :main ^:skip-aot d5.core
  :aot [java-time.api]
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all
                       :jvm-opts ["-Dclojure.compiler.direct-linking=true"]}})
