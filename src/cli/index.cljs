(ns cli.index)

(defn -main [& args]
  (println "You passed the following args to imm:")
  (println args))

(set! *main-cli-fn* -main)
