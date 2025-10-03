(cd src/details && docker build -t details:local -f Dockerfile .)
(cd src/reviews && docker build -t reviews:local -f Dockerfile .)
(cd src/ratings && docker build -t ratings:local -f Dockerfile .)
(cd src/ratings-v2 && docker build -t ratings-v2:local -f Dockerfile .)