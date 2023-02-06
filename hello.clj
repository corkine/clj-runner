; bb
#!/usr/bin/env bb -Sdeps '{:paths ["."] :deps {etaoin/etaoin {:mvn/version,"1.0.39"}}}'
#!/usr/bin/env bb
(ns hello)
(println "Hello World!")