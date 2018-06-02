(ns cli.index)

(defn reload! []
  (println "Code updated & hot-reloaded."))

(defn -main [& args]
  (println "You passed the following args to imm:")
  (println args))

(set! *main-cli-fn* -main)
