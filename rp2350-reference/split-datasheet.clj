(ns split-datasheet
  (:require [clojure.java.io :as io]
            [clojure.java.shell :as shell]
            [clojure.string :as str]))

(def printed-ranges
  [{:name "01-introduction" :start 13 :end 23}
   {:name "02-system-bus" :start 24 :end 34}
   {:name "03a-processor-SIO" :start 35 :end 81}
   {:name "03b-processor-interrupts-debug" :start 82 :end 99}
   {:name "03c-cortex-m33-coprocessors" :start 100 :end 122}
   {:name "03d-cortex-m33-processor" :start 123 :end 232}
   {:name "03e-hazard3-processor" :start 233 :end 334}
   {:name "03f-architecture-switching" :start 335 :end 336}
   {:name "04-memory" :start 337 :end 352}
   {:name "05a-bootrom-concepts" :start 353 :end 374}
   {:name "05b-bootrom-apis" :start 375 :end 415}
   {:name "05c-bootrom-usb-uart" :start 416 :end 440}
   {:name "06-power" :start 441 :end 493}
   {:name "07-resets" :start 494 :end 512}
   {:name "08a-clocks-overview" :start 513 :end 553}
   {:name "08b-clocks-oscillators" :start 554 :end 586}
   {:name "09a-gpio-overview" :start 587 :end 603}
   {:name "09b-gpio-io-user-bank" :start 604 :end 759}
   {:name "09c-gpio-io-qspi-pads" :start 760 :end 815}
   {:name "10-security" :start 816 :end 875}
   {:name "11a-pio-overview-model" :start 876 :end 888}
   {:name "11b-pio-instructions" :start 889 :end 901}
   {:name "11c-pio-details-examples" :start 902 :end 960}
   {:name "12a-uart" :start 961 :end 982}
   {:name "12b-i2c" :start 983 :end 1045}
   {:name "12c-spi" :start 1046 :end 1065}
   {:name "12d-adc-temp" :start 1066 :end 1075}
   {:name "12e-pwm" :start 1076 :end 1093}
   {:name "12f-dma" :start 1094 :end 1140}
   {:name "12g-usb" :start 1141 :end 1181}
   {:name "12h-timers-watchdog" :start 1182 :end 1201}
   {:name "12i-hstx" :start 1202 :end 1211}
   {:name "12j-trng-sha256" :start 1212 :end 1225}
   {:name "12k-qspi-qmi" :start 1226 :end 1248}
   {:name "12l-system-control" :start 1249 :end 1267}
   {:name "13-otp" :start 1268 :end 1326}
   {:name "14-electrical" :start 1327 :end 1348}
   {:name "15-appendices" :start 1349 :end 1378}])

(def bookmark-checks
  [{:range-name "01-introduction" :title-prefix "Chapter 1. "}
   {:range-name "02-system-bus" :title-prefix "Chapter 2. "}
   {:range-name "03a-processor-SIO" :title-prefix "Chapter 3. "}
   {:range-name "04-memory" :title-prefix "Chapter 4. "}
   {:range-name "05a-bootrom-concepts" :title-prefix "Chapter 5. "}
   {:range-name "06-power" :title-prefix "Chapter 6. "}
   {:range-name "07-resets" :title-prefix "Chapter 7. "}
   {:range-name "08a-clocks-overview" :title-prefix "Chapter 8. "}
   {:range-name "09a-gpio-overview" :title-prefix "Chapter 9. "}
   {:range-name "10-security" :title-prefix "Chapter 10. "}
   {:range-name "11a-pio-overview-model" :title-prefix "Chapter 11. "}
   {:range-name "12a-uart" :title-prefix "Chapter 12. "}
   {:range-name "13-otp" :title-prefix "Chapter 13. "}
   {:range-name "14-electrical" :title-prefix "Chapter 14. "}
   {:range-name "15-appendices" :title-prefix "Appendix A:"}])

(defn die [message]
  (binding [*out* *err*]
    (println (str "error: " message)))
  (System/exit 1))

(defn warn [message]
  (binding [*out* *err*]
    (println (str "warning: " message))))

(defn run-capture [command & args]
  (let [{:keys [exit out err]} (apply shell/sh command args)]
    (if (zero? exit)
      out
      (die (str "command failed: "
                (str/join " " (cons command args))
                " ("
                (if (str/blank? err) (str "exit " exit) (str/trim err))
                ")")))))

(defn run-command [command & args]
  (let [{:keys [exit err]} (apply shell/sh command args)]
    (when-not (zero? exit)
      (die (str "command failed: "
                (str/join " " (cons command args))
                " ("
                (if (str/blank? err) (str "exit " exit) (str/trim err))
                ")")))))

(defn require-tool [tool-name]
  (let [{:keys [exit]} (shell/sh "sh" "-lc" (str "command -v " tool-name))]
    (when-not (zero? exit)
      (die (str tool-name " is required but not installed")))))

(defn parse-int [text context]
  (try
    (Integer/parseInt (str/trim text))
    (catch Exception _
      (die (str "invalid integer for " context ": " text)))))

(defn parse-bookmarks [bookmark-dump]
  (loop [lines (str/split-lines bookmark-dump)
         current-title nil
         bookmarks []]
    (if (empty? lines)
      (if (empty? bookmarks)
        (die "no bookmarks found in PDF metadata")
        bookmarks)
      (let [line (str/trim (first lines))]
        (cond
          (str/starts-with? line "BookmarkTitle: ")
          (recur (rest lines)
                 (subs line (count "BookmarkTitle: "))
                 bookmarks)

          (and current-title (str/starts-with? line "BookmarkPageNumber: "))
          (let [page-text (subs line (count "BookmarkPageNumber: "))
                page-number (parse-int page-text "bookmark page number")]
            (recur (rest lines)
                   nil
                   (conj bookmarks {:title current-title :page page-number})))

          :else
          (recur (rest lines) current-title bookmarks))))))

(defn bookmark-page [bookmarks title-prefix]
  (if-let [match (first (filter #(str/starts-with? (:title %) title-prefix) bookmarks))]
    (:page match)
    (die (str "could not find bookmark starting with: " title-prefix))))

(defn range-start [ranges range-name]
  (:start (first (filter #(= (:name %) range-name) ranges))))

(defn build-physical-ranges [bookmarks]
  (let [chapter1-start (bookmark-page bookmarks "Chapter 1. ")
        appendix-start (bookmark-page bookmarks "Appendix A:")
        first-printed-page (:start (first printed-ranges))
        page-offset (- chapter1-start first-printed-page)]
    (when (neg? page-offset)
      (die (str "calculated negative page offset: " page-offset)))
    (let [front-matter-end (dec chapter1-start)]
      (when (< front-matter-end 1)
        (die (str "invalid front matter end page: " front-matter-end)))
      (let [physical-ranges (into [{:name "00-front-matter" :start 1 :end front-matter-end}]
                                  (map (fn [entry]
                                         (-> entry
                                             (update :start + page-offset)
                                             (update :end + page-offset)))
                                       printed-ranges))
            appendices-start (range-start physical-ranges "15-appendices")]
        (when (not= appendices-start appendix-start)
          (die (str "appendix start mismatch: range starts at "
                    appendices-start
                    ", bookmark starts at "
                    appendix-start)))
        {:ranges physical-ranges :page-offset page-offset}))))

(defn validate-ranges [ranges total-pages]
  (loop [remaining ranges
         previous-end 0
         line-number 1]
    (if (empty? remaining)
      (when (< previous-end total-pages)
        (warn (str "ranges end at "
                   previous-end
                   " but source has "
                   total-pages
                   " pages (trailing pages excluded)")))
      (let [{:keys [name start end]} (first remaining)]
        (when (> start end)
          (die (str "line " line-number " has start > end: " name " " start "-" end)))
        (when (<= start previous-end)
          (die (str "line "
                    line-number
                    " overlaps prior range: "
                    name
                    " starts at "
                    start
                    ", previous ended at "
                    previous-end)))
        (when (not= start (inc previous-end))
          (die (str "line "
                    line-number
                    " has gap before "
                    name
                    ": expected "
                    (inc previous-end)
                    ", got "
                    start)))
        (when (> end total-pages)
          (die (str "line "
                    line-number
                    " exceeds source page count ("
                    total-pages
                    "): "
                    name
                    " ends at "
                    end)))
        (recur (rest remaining) end (inc line-number))))))

(defn validate-bookmark-alignment [ranges bookmarks]
  (doseq [{:keys [range-name title-prefix]} bookmark-checks]
    (let [expected (bookmark-page bookmarks title-prefix)
          actual (range-start ranges range-name)]
      (when (nil? actual)
        (die (str "could not find generated range: " range-name)))
      (when (not= actual expected)
        (die (str "bookmark mismatch for "
                  range-name
                  ": range starts at "
                  actual
                  ", bookmark starts at "
                  expected))))))

(defn human-size [size-bytes]
  (let [units ["B" "K" "M" "G" "T"]]
    (loop [size (double size-bytes) index 0]
      (if (and (>= size 1024.0) (< index (dec (count units))))
        (recur (/ size 1024.0) (inc index))
        (if (zero? index)
          (format "%d%s" (long size) (nth units index))
          (format "%.1f%s" size (nth units index)))))))

(defn split-range [src-path out-dir {:keys [name start end]}]
  (let [pages (inc (- end start))
        out-path (.getPath (io/file out-dir (str name ".pdf")))]
    (println (format "  %s (%dpp: %d-%d)" name pages start end))
    (run-command "qpdf"
                 src-path
                 "--pages"
                 "."
                 (str start "-" end)
                 "--"
                 out-path)))

(defn list-output-pdfs [out-dir]
  (let [files (->> (.listFiles (io/file out-dir))
                   (filter #(str/ends-with? (.getName %) ".pdf"))
                   (sort-by #(.length %) >))]
    (doseq [file files]
      (println (format "%5s %s" (human-size (.length file)) (.getName file))))))

(defn default-base-dir []
  (let [cwd (System/getProperty "user.dir")
        candidates [cwd (str (io/file cwd "rp2350-reference"))]]
    (or (first (filter #(-> (io/file % "rp2350-datasheet.pdf") .exists) candidates))
        cwd)))

(defn total-pages [src-path]
  (parse-int (run-capture "qpdf" "--show-npages" src-path) "total page count"))

(defn -main [& args]
  (when (> (count args) 2)
    (binding [*out* *err*]
      (println "usage: clojure split-datasheet.clj [source.pdf] [output_dir]"))
    (System/exit 2))

  (let [base-dir (default-base-dir)
        src-path (if (>= (count args) 1)
                   (nth args 0)
                   (.getPath (io/file base-dir "rp2350-datasheet.pdf")))
        out-dir (if (>= (count args) 2)
                  (nth args 1)
                  (.getPath (io/file base-dir "datasheet")))]
    (require-tool "qpdf")
    (require-tool "pdftk")
    (when-not (.exists (io/file src-path))
      (die (str "source PDF not found: " src-path)))

    (let [bookmarks (parse-bookmarks (run-capture "pdftk" src-path "dump_data"))
          {:keys [ranges page-offset]} (build-physical-ranges bookmarks)
          pages (total-pages src-path)]
      (validate-ranges ranges pages)
      (validate-bookmark-alignment ranges bookmarks)

      (.mkdirs (io/file out-dir))
      (println (str "Splitting RP2350 datasheet from: " src-path " (page offset: +" page-offset ")"))
      (println)
      (doseq [range ranges]
        (split-range src-path out-dir range))
      (println)
      (println (str "Done! Files in: " out-dir))
      (list-output-pdfs out-dir))))

(apply -main *command-line-args*)
